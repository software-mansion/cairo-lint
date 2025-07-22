use std::collections::{HashSet, VecDeque};

use cairo_lang_defs::ids::{LanguageElementId, ModuleId};
use cairo_lang_defs::plugin::PluginDiagnostic;
use cairo_lang_filesystem::db::{get_parent_and_mapping, translate_location};
use cairo_lang_filesystem::ids::{CodeOrigin, FileId, FileLongId};
use cairo_lang_semantic::db::SemanticGroup;
use cairo_lang_syntax::node::SyntaxNode;
use cairo_lang_syntax::node::db::SyntaxGroup;
use cairo_lang_syntax::node::helpers::QueryAttrs;
use cairo_lang_syntax::node::kind::SyntaxKind;
use cairo_lang_utils::LookupIntern;
use cairo_lang_utils::Upcast;
use cairo_lang_utils::ordered_hash_set::OrderedHashSet;
use if_chain::if_chain;

use crate::CairoLintToolMetadata;
use crate::context::{
    get_all_checking_functions, get_name_for_diagnostic_message, is_lint_enabled_by_default,
};
use crate::corelib::CorelibContext;
use crate::mappings::{get_origin_module_item_as_syntax_node, get_origin_syntax_node};

mod db;
pub use db::{LinterAnalysisDatabase, LinterAnalysisDatabaseBuilder};

#[derive(PartialEq, Eq, Hash, Debug, Clone)]
pub struct LinterDiagnosticParams {
    pub only_generated_files: bool,
    pub tool_metadata: CairoLintToolMetadata,
}

#[salsa::query_group(LinterDatabase)]
pub trait LinterGroup: SemanticGroup + Upcast<dyn SemanticGroup> {
    fn linter_diagnostics(
        &self,
        corelib_context: CorelibContext,
        params: LinterDiagnosticParams,
        module_id: ModuleId,
    ) -> Vec<PluginDiagnostic>;

    fn node_resultants(&self, node: SyntaxNode) -> Option<Vec<SyntaxNode>>;

    fn file_and_subfiles_with_corresponding_modules(
        &self,
        file: FileId,
    ) -> Option<(HashSet<FileId>, HashSet<ModuleId>)>;

    fn find_generated_nodes(
        &self,
        node_descendant_files: Vec<FileId>,
        node: SyntaxNode,
    ) -> OrderedHashSet<SyntaxNode>;
}

#[tracing::instrument(skip_all, level = "trace")]
fn linter_diagnostics(
    db: &dyn LinterGroup,
    corelib_context: CorelibContext,
    params: LinterDiagnosticParams,
    module_id: ModuleId,
) -> Vec<PluginDiagnostic> {
    let mut diags: Vec<(PluginDiagnostic, FileId)> = Vec::new();
    let Ok(items) = db.module_items(module_id) else {
        return Vec::default();
    };
    for item in &*items {
        let mut item_diagnostics = Vec::new();
        let module_file = db.module_main_file(module_id).unwrap();
        let item_file = item.stable_location(db).file_id(db).lookup_intern(db);
        let is_generated_item =
            matches!(item_file, FileLongId::Virtual(_) | FileLongId::External(_));

        if is_generated_item && !params.only_generated_files {
            let item_syntax_node = item.stable_location(db).stable_ptr().lookup(db.upcast());
            let origin_node = get_origin_module_item_as_syntax_node(db, item);

            if_chain! {
                if let Some(node) = origin_node;
                if let Some(resultants) = db.node_resultants(node);
                // Check if the item has only a single resultant, as if there is multiple resultants,
                // we would generate different diagnostics for each of resultants.
                // If we don't check this, we might generate different diagnostics for the same item,
                // which is a very unpredictable behavior.
                if resultants.len() == 1;
                // We don't do the `==` check here, as the origin node always has the proc macro attributes.
                // It also means that if the macro changed anything in the original item code,
                // we won't be processing it, as it might lead to unexpected behavior.
                if node.get_text_without_trivia(db).contains(&item_syntax_node.get_text_without_trivia(db));
                then {
                    let checking_functions = get_all_checking_functions();
                    for checking_function in checking_functions {
                        checking_function(db, &corelib_context, item, &mut item_diagnostics);
                    }

                    diags.extend(item_diagnostics.into_iter().filter_map(|mut diag| {
                      let ptr = diag.stable_ptr;
                      diag.stable_ptr = get_origin_syntax_node(db, &ptr)?.stable_ptr(db);
                      Some((diag, module_file))}));
                }
            }
        } else if !is_generated_item || params.only_generated_files {
            let checking_functions = get_all_checking_functions();
            for checking_function in checking_functions {
                checking_function(db, &corelib_context, item, &mut item_diagnostics);
            }

            diags.extend(item_diagnostics.into_iter().filter_map(|diag| {
                // If the diagnostic is not mapped to an on-disk file, it mean that it's an inline macro diagnostic.
                get_origin_syntax_node(db, &diag.stable_ptr).map(|_| (diag, module_file))
            }));
        }
    }

    diags
        .into_iter()
        .filter(|diag: &(PluginDiagnostic, FileId)| {
            let diagnostic = &diag.0;
            let node = diagnostic.stable_ptr.lookup(db.upcast());
            let allowed_name = get_name_for_diagnostic_message(&diagnostic.message).unwrap();
            let default_allowed = is_lint_enabled_by_default(&diagnostic.message).unwrap();
            let is_rule_allowed_globally = *params
                .tool_metadata
                .get(allowed_name)
                .unwrap_or(&default_allowed);
            !node_has_ascendants_with_allow_name_attr(db.upcast(), node, allowed_name)
                && is_rule_allowed_globally
        })
        .map(|diag| diag.0)
        .collect()
}

#[tracing::instrument(level = "trace", skip(db))]
fn node_resultants(db: &dyn LinterGroup, node: SyntaxNode) -> Option<Vec<SyntaxNode>> {
    let main_file = node.stable_ptr(db).file_id(db);

    let (mut files, _) = db.file_and_subfiles_with_corresponding_modules(main_file)?;

    files.remove(&main_file);

    let files: Vec<_> = files.into_iter().collect();
    let resultants = db.find_generated_nodes(files, node);

    Some(resultants.into_iter().collect())
}

#[tracing::instrument(level = "trace", skip(db))]
pub fn file_and_subfiles_with_corresponding_modules(
    db: &dyn LinterGroup,
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

#[tracing::instrument(level = "trace", skip(db))]
pub fn find_generated_nodes(
    db: &dyn LinterGroup,
    node_descendant_files: Vec<FileId>,
    node: SyntaxNode,
) -> OrderedHashSet<SyntaxNode> {
    let start_file = node.stable_ptr(db).file_id(db);

    let mut result = OrderedHashSet::default();

    let mut is_replaced = false;

    for file in node_descendant_files.iter().cloned() {
        let Some((parent, mappings)) = get_parent_and_mapping(db, file) else {
            continue;
        };

        if parent != start_file {
            continue;
        }

        let Ok(file_syntax) = db.file_syntax(file) else {
            continue;
        };

        let mappings: Vec<_> = mappings
            .iter()
            .filter(|mapping| match mapping.origin {
                CodeOrigin::CallSite(_) => true,
                CodeOrigin::Start(start) => start == node.span(db).start,
                CodeOrigin::Span(span) => node.span(db).contains(span),
            })
            .cloned()
            .collect();
        if mappings.is_empty() {
            continue;
        }

        let is_replacing_og_item = match file.lookup_intern(db) {
            FileLongId::Virtual(vfs) => vfs.original_item_removed,
            FileLongId::External(id) => db.ext_as_virtual(id).original_item_removed,
            _ => unreachable!(),
        };

        let mut new_nodes: OrderedHashSet<_> = Default::default();

        for mapping in &mappings {
            for token in file_syntax.lookup_offset(db, mapping.span.start).tokens(db) {
                // Skip end of the file terminal, which is also a syntax tree leaf.
                // As `ModuleItemList` and `TerminalEndOfFile` have the same parent,
                // which is the `SyntaxFile`, so we don't want to take the `SyntaxFile`
                // as an additional resultant.
                if token.kind(db) == SyntaxKind::TerminalEndOfFile {
                    continue;
                }
                let nodes: Vec<_> = token
                    .ancestors_with_self(db)
                    .map_while(|new_node| {
                        translate_location(&mappings, new_node.span(db))
                            .map(|span_in_parent| (new_node, span_in_parent))
                    })
                    .take_while(|(_, span_in_parent)| node.span(db).contains(*span_in_parent))
                    .collect();

                if let Some((last_node, _)) = nodes.last().cloned() {
                    let (new_node, _) = nodes
                        .into_iter()
                        .rev()
                        .take_while(|(node, _)| node.span(db) == last_node.span(db))
                        .last()
                        .unwrap();

                    new_nodes.insert(new_node);
                }
            }
        }

        // If there is no node found, don't mark it as potentially replaced.
        if !new_nodes.is_empty() {
            is_replaced = is_replaced || is_replacing_og_item;
        }

        for new_node in new_nodes {
            result.extend(find_generated_nodes(
                db,
                node_descendant_files.clone(),
                new_node,
            ));
        }
    }

    if !is_replaced {
        result.insert(node);
    }

    result
}

#[tracing::instrument(skip_all, level = "trace")]
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
