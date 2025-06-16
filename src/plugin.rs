use anyhow::{anyhow, Result};
use cairo_lang_defs::ids::{LanguageElementId, ModuleId, ModuleItemId};
use cairo_lang_defs::plugin::PluginDiagnostic;
use cairo_lang_diagnostics::ToOption;
use cairo_lang_filesystem::db::{
    get_originating_location, get_parent_and_mapping, translate_location, FilesGroup,
};
use cairo_lang_filesystem::ids::{FileId, FileLongId};
use cairo_lang_filesystem::span::TextOffset;
use cairo_lang_parser::db::ParserGroup;
use cairo_lang_parser::printer::print_tree;
use cairo_lang_semantic::db::SemanticGroup;
use cairo_lang_semantic::plugin::{AnalyzerPlugin, PluginSuite};
use cairo_lang_syntax::node::ast::ModuleItem;
use cairo_lang_syntax::node::db::SyntaxGroup;
use cairo_lang_syntax::node::helpers::QueryAttrs;
use cairo_lang_syntax::node::kind::SyntaxKind;
use cairo_lang_syntax::node::SyntaxNode;
use cairo_lang_syntax::node::TypedSyntaxNode;
use cairo_lang_utils::ordered_hash_set::OrderedHashSet;
use cairo_lang_utils::LookupIntern;
use if_chain::if_chain;
use std::collections::{HashSet, VecDeque};
use std::sync::Arc;

use crate::context::{
    get_all_checking_functions, get_name_for_diagnostic_message, get_unique_allowed_names,
    is_lint_enabled_by_default,
};
use crate::CairoLintToolMetadata;

pub fn cairo_lint_plugin_suite(tool_metadata: CairoLintToolMetadata) -> Result<PluginSuite> {
    let mut suite = PluginSuite::default();
    validate_cairo_lint_metadata(&tool_metadata)?;
    suite.add_analyzer_plugin_ex(Arc::new(CairoLint::new(false, tool_metadata)));
    Ok(suite)
}

pub fn cairo_lint_plugin_suite_without_metadata_validation(
    tool_metadata: CairoLintToolMetadata,
) -> PluginSuite {
    let mut suite = PluginSuite::default();
    suite.add_analyzer_plugin_ex(Arc::new(CairoLint::new(false, tool_metadata)));
    suite
}

pub fn cairo_lint_allow_plugin_suite() -> PluginSuite {
    let mut suite = PluginSuite::default();
    suite.add_analyzer_plugin::<CairoLintAllow>();
    suite
}

#[derive(Debug, Default)]
pub struct CairoLint {
    include_compiler_generated_files: bool,
    tool_metadata: CairoLintToolMetadata,
}

impl CairoLint {
    pub fn new(
        include_compiler_generated_files: bool,
        tool_metadata: CairoLintToolMetadata,
    ) -> Self {
        Self {
            include_compiler_generated_files,
            tool_metadata,
        }
    }

    pub fn include_compiler_generated_files(&self) -> bool {
        self.include_compiler_generated_files
    }

    pub fn tool_metadata(&self) -> &CairoLintToolMetadata {
        &self.tool_metadata
    }
}

impl AnalyzerPlugin for CairoLint {
    fn declared_allows(&self) -> Vec<String> {
        get_unique_allowed_names()
            .iter()
            .map(ToString::to_string)
            .collect()
    }

    fn diagnostics(&self, db: &dyn SemanticGroup, module_id: ModuleId) -> Vec<PluginDiagnostic> {
        let mut diags: Vec<(PluginDiagnostic, FileId)> = Vec::new();
        let Ok(items) = db.module_items(module_id) else {
            return Vec::default();
        };
        println!("items: {:?}", items);
        for item in &*items {
            println!("===============================\n");
            let module_file = db.module_main_file(module_id).unwrap();
            let item_file = item.stable_location(db).file_id(db).lookup_intern(db);
            println!("item_file: {:?}", item_file);
            println!(
                "include_compiler_generated_files: {}",
                self.include_compiler_generated_files
            );
            println!(
                "item: {:?}, item_file: {:?}, module_file: {:?}",
                item, item_file, module_file
            );
            println!(
                "code: {}\n\n",
                item.stable_location(db)
                    .stable_ptr()
                    .lookup(db.upcast())
                    .get_text(db.upcast())
            );

            let is_generated_item =
                matches!(item_file, FileLongId::Virtual(_) | FileLongId::External(_));

            // Skip compiler generated files. By default it checks whether the item is inside the virtual or external file.
            // if !self.include_compiler_generated_files && is_generated_item {
            //     continue;
            // }

            if is_generated_item {
                let origin_node = get_origin_syntax_node(db, item);
                if let Some(node) = origin_node {
                    let resultants = get_node_resultants(db, node);
                    println!("---\nresulntants: {:?}\n---", resultants);
                    if_chain! {
                        if let Some(resultants) = resultants;
                        if resultants.len() == 1;
                        then {
                            println!("CODE From generated file: {}", resultants[0].get_text(db.upcast()));
                            println!("CODE From og file: {}", node.get_text(db.upcast()));
                        }
                    }

                    // if_chain! {
                    //     // if let Some(resultants) = resultants;
                    //     if resultants.len() == 1;
                    //     then {
                    //         println!("CODE From generated file: {}", resultants[0].get_text(db.upcast()));
                    //         println!("CODE From og file: {}", node.get_text(db.upcast()));
                    //     }
                    // }
                }
                // println!("^^^ From generated file");
            }

            let checking_functions = get_all_checking_functions();
            let mut item_diagnostics = Vec::new();

            for checking_function in checking_functions {
                checking_function(db, item, &mut item_diagnostics);
            }

            diags.extend(item_diagnostics.into_iter().map(|diag| (diag, module_file)));
        }

        diags
            .into_iter()
            .filter(|diag| {
                let diagnostic = &diag.0;
                let node = diagnostic.stable_ptr.lookup(db.upcast());
                let allowed_name = get_name_for_diagnostic_message(&diagnostic.message).unwrap();
                let default_allowed = is_lint_enabled_by_default(&diagnostic.message).unwrap();
                let is_rule_allowed_globally = *self
                    .tool_metadata
                    .get(allowed_name)
                    .unwrap_or(&default_allowed);
                !node_has_ascendants_with_allow_name_attr(db.upcast(), node, allowed_name)
                    && is_rule_allowed_globally
            })
            .map(|diag| diag.0)
            .collect()
    }
}

fn get_origin_syntax_node(
    db: &dyn SemanticGroup,
    module_item_id: &ModuleItemId,
) -> Option<SyntaxNode> {
    let ptr = module_item_id.stable_location(db).stable_ptr();
    let (file, span) = get_originating_location(
        db,
        ptr.file_id(db),
        ptr.lookup(db).span_without_trivia(db),
        None,
    );

    find_syntax_node_at_offset(db.upcast(), file, span.start)?
        .ancestors_with_self(db)
        .find(|n| ModuleItem::cast(db, *n).is_some())
}

fn find_syntax_node_at_offset(
    db: &dyn ParserGroup,
    file: FileId,
    offset: TextOffset,
) -> Option<SyntaxNode> {
    Some(db.file_syntax(file).to_option()?.lookup_offset(db, offset))
}

fn get_node_resultants(db: &dyn SemanticGroup, node: SyntaxNode) -> Option<Vec<SyntaxNode>> {
    let main_file = node.stable_ptr(db).file_id(db);

    let (mut files, _) = file_and_subfiles_with_corresponding_modules(db, main_file)?;

    files.remove(&main_file);

    let files: Vec<_> = files.into_iter().collect();
    files.iter().for_each(|file| {
        println!("File: {:?}", file.lookup_intern(db));
    });
    let resultants = find_generated_nodes(db, &files, node);

    Some(resultants.into_iter().collect())
}

fn file_and_subfiles_with_corresponding_modules(
    db: &dyn SemanticGroup,
    file: FileId,
) -> Option<(HashSet<FileId>, HashSet<ModuleId>)> {
    let mut modules: HashSet<_> = db.file_modules(file).ok()?.iter().copied().collect();
    let mut files = HashSet::from([file]);
    // Collect descendants of `file`
    // and modules from all virtual files that are descendants of `file`.
    //
    // Caveat: consider a situation `file1` --(child)--> `file2` with file contents:
    // - `file1`: `mod file2_origin_module { #[file2]fn sth() {} }`
    // - `file2`: `mod mod_from_file2 { }`
    //  It is important that `file2` content contains a module.
    //
    // Problem: in this situation it is not enough to call `db.file_modules(file1_id)` since
    //  `mod_from_file2` won't be in the result of this query.
    // Solution: we can find file id of `file2`
    //  (note that we only have file id of `file1` at this point)
    //  in `db.module_files(mod_from_file1_from_which_file2_origins)`.
    //  Then we can call `db.file_modules(file2_id)` to obtain module id of `mod_from_file2`.
    //  We repeat this procedure until there is nothing more to collect.
    let mut modules_queue: VecDeque<_> = modules.iter().copied().collect();
    while let Some(module_id) = modules_queue.pop_front() {
        for file_id in db.module_files(module_id).ok()?.iter() {
            if files.insert(*file_id) {
                for module_id in db.file_modules(*file_id).ok()?.iter() {
                    if modules.insert(*module_id) {
                        modules_queue.push_back(*module_id);
                    }
                }
            }
        }
    }
    Some((files, modules))
}
/// See [`LsSemanticGroup::get_node_resultants`].
fn find_generated_nodes(
    db: &dyn SemanticGroup,
    node_descendant_files: &[FileId],
    node: SyntaxNode,
) -> OrderedHashSet<SyntaxNode> {
    let start_file = node.stable_ptr(db).file_id(db);

    let mut result = OrderedHashSet::default();

    let mut is_replaced = false;

    println!("Node span: {:?}", node.span(db));
    println!("Node text:\n {}", node.get_text(db));
    for &file in node_descendant_files {
        let Some((parent, mappings)) = get_parent_and_mapping(db, file) else {
            continue;
        };

        if parent != start_file {
            continue;
        }

        let Ok(file_syntax) = db.file_syntax(file) else {
            continue;
        };

        let is_replacing_og_item = match file.lookup_intern(db) {
            FileLongId::Virtual(vfs) => vfs.original_item_removed,
            FileLongId::External(id) => db.ext_as_virtual(id).original_item_removed,
            _ => unreachable!(),
        };

        let mut new_nodes: OrderedHashSet<_> = Default::default();
        println!("File: \n{}", print_tree(db, &file_syntax, false, false));

        for token in file_syntax.tokens(db) {
            if token.kind(db) == SyntaxKind::TerminalEndOfFile {
                continue;
            }
            let nodes: Vec<_> = token
                .ancestors_with_self(db)
                .map_while(|new_node| {
                    translate_location(&mappings, new_node.span(db))
                        .map(|span_in_parent| (new_node, span_in_parent))
                })
                .take_while(|(_, span_in_parent)| {
                    // println!("Span in parent: {:?}", span_in_parent);
                    node.span(db).contains(*span_in_parent)
                })
                .collect();

            if let Some((last_node, _)) = nodes.last().cloned() {
                println!("Last node kind: {}", last_node.kind(db));
                println!("Last node span: {:?}", last_node.span(db));
                let (new_node, _) = nodes
                    .into_iter()
                    .rev()
                    .take_while(|(node, _)| {
                        println!("node kind: {}", node.kind(db));
                        println!("node span: {:?}", node.span(db));
                        node.span(db) == last_node.span(db)
                    })
                    .last()
                    .unwrap();

                println!("New node: {:?}", new_node);

                new_nodes.insert(new_node);
            }
        }

        // If there is no node found, don't mark it as potentially replaced.
        if !new_nodes.is_empty() {
            is_replaced = is_replaced || is_replacing_og_item;
        }

        for new_node in new_nodes {
            result.extend(find_generated_nodes(db, node_descendant_files, new_node));
        }
    }

    if !is_replaced {
        result.insert(node);
    }

    result
}

/// Plugin with `declared_allows` matching these of [`CairoLint`] that does not emit diagnostics.
/// Add it when `CairoLint` is not present to avoid compiler warnings on unsupported
/// `allow` attribute arguments.
#[derive(Debug, Default)]
pub struct CairoLintAllow;

impl AnalyzerPlugin for CairoLintAllow {
    fn diagnostics(&self, _db: &dyn SemanticGroup, _module_id: ModuleId) -> Vec<PluginDiagnostic> {
        Vec::new()
    }

    fn declared_allows(&self) -> Vec<String> {
        get_unique_allowed_names()
            .iter()
            .map(ToString::to_string)
            .collect()
    }
}

fn node_has_ascendants_with_allow_name_attr(
    db: &dyn SyntaxGroup,
    node: SyntaxNode,
    allowed_name: &'static str,
) -> bool {
    for node in node.ancestors_with_self(db) {
        if node.has_attr_with_arg(db, "allow", allowed_name) {
            return true;
        }
    }
    false
}

fn validate_cairo_lint_metadata(tool_metadata: &CairoLintToolMetadata) -> Result<()> {
    for (name, _) in tool_metadata.iter() {
        if !get_unique_allowed_names().contains(&name.as_str()) {
            return Err(anyhow!(
                "The lint '{}' specified in `Scarb.toml` is not supported by the Cairo lint.",
                name
            ));
        }
    }
    Ok(())
}
