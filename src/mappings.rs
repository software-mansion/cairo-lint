//! This module provides functionality to handle any code that comes as the product of procedural macros.

use std::collections::{HashSet, VecDeque};

use cairo_lang_defs::ids::LanguageElementId;
use cairo_lang_defs::ids::ModuleId;
use cairo_lang_defs::ids::ModuleItemId;
use cairo_lang_diagnostics::ToOption;
use cairo_lang_filesystem::db::get_parent_and_mapping;
use cairo_lang_filesystem::db::translate_location;
use cairo_lang_filesystem::ids::FileLongId;
use cairo_lang_filesystem::{db::get_originating_location, ids::FileId, span::TextOffset};
use cairo_lang_parser::db::ParserGroup;
use cairo_lang_semantic::db::SemanticGroup;
use cairo_lang_syntax::node::kind::SyntaxKind;
use cairo_lang_syntax::node::{ast::ModuleItem, ids::SyntaxStablePtrId, SyntaxNode};
use cairo_lang_utils::ordered_hash_set::OrderedHashSet;
use cairo_lang_utils::LookupIntern;

/// Copied from https://github.com/software-mansion/cairols/blob/0bb49e7d2f89ffe68ba20379c20b63fc49f82557/src/lang/db/semantic.rs#L326.
pub fn get_node_resultants(db: &dyn SemanticGroup, node: SyntaxNode) -> Option<Vec<SyntaxNode>> {
    let main_file = node.stable_ptr(db).file_id(db);

    let (mut files, _) = file_and_subfiles_with_corresponding_modules(db, main_file)?;

    files.remove(&main_file);

    let files: Vec<_> = files.into_iter().collect();
    let resultants = find_generated_nodes(db, &files, node);

    Some(resultants.into_iter().collect())
}

/// Returns the originating syntax node for a given stable pointer.
pub fn get_origin_syntax_node(
    db: &dyn SemanticGroup,
    ptr: &SyntaxStablePtrId,
) -> Option<SyntaxNode> {
    let syntax_node = ptr.lookup(db.upcast());
    let (file, span) = get_originating_location(
        db,
        ptr.file_id(db),
        ptr.lookup(db).span_without_trivia(db),
        None,
    );

    // Heuristically find the syntax node at the given offset.
    // We match the ancestors with node text to ensure we get the whole node.
    return find_syntax_node_at_offset(db.upcast(), file, span.start)?
        .ancestors_with_self(db)
        .find(|node| node.get_text_without_trivia(db) == syntax_node.get_text_without_trivia(db));
}

pub fn get_origin_module_item_as_syntax_node(
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
        .find(|n| ModuleItem::is_variant(n.kind(db)))
}

fn find_syntax_node_at_offset(
    db: &dyn ParserGroup,
    file: FileId,
    offset: TextOffset,
) -> Option<SyntaxNode> {
    Some(db.file_syntax(file).to_option()?.lookup_offset(db, offset))
}

/// Copied from https://github.com/software-mansion/cairols/blob/0bb49e7d2f89ffe68ba20379c20b63fc49f82557/src/lang/db/semantic.rs#L290.
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

/// Copied from https://github.com/software-mansion/cairols/blob/0bb49e7d2f89ffe68ba20379c20b63fc49f82557/src/lang/db/semantic.rs#L508.
fn find_generated_nodes(
    db: &dyn SemanticGroup,
    node_descendant_files: &[FileId],
    node: SyntaxNode,
) -> OrderedHashSet<SyntaxNode> {
    let start_file = node.stable_ptr(db).file_id(db);

    let mut result = OrderedHashSet::default();

    let mut is_replaced = false;

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

        for token in file_syntax.tokens(db) {
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
