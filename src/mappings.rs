use crate::upstream::file_syntax;
use cairo_lang_defs::ids::{LanguageElementId, ModuleItemId};
use cairo_lang_diagnostics::ToOption;
use cairo_lang_filesystem::{db::get_originating_location, ids::FileId, span::TextOffset};
use cairo_lang_syntax::node::{SyntaxNode, ast::ModuleItem, ids::SyntaxStablePtrId};
use salsa::Database;

#[tracing::instrument(level = "trace", skip(db))]
pub fn get_origin_module_item_as_syntax_node<'db>(
    db: &'db dyn Database,
    module_item_id: &ModuleItemId<'db>,
) -> Option<SyntaxNode<'db>> {
    let ptr = module_item_id.stable_location(db).stable_ptr();
    let (file, span) = get_originating_location(
        db,
        ptr.file_id(db),
        ptr.lookup(db).span_without_trivia(db),
        None,
    );

    find_syntax_node_at_offset(db, file, span.start)?
        .ancestors_with_self(db)
        .find(|n| ModuleItem::is_variant(n.kind(db)))
}

/// Returns the originating syntax node for a given stable pointer.
#[tracing::instrument(level = "trace", skip(db))]
pub fn get_origin_syntax_node<'db>(
    db: &'db dyn Database,
    ptr: &SyntaxStablePtrId<'db>,
) -> Option<SyntaxNode<'db>> {
    let syntax_node = ptr.lookup(db);
    let (file, span) = get_originating_location(
        db,
        ptr.file_id(db),
        ptr.lookup(db).span_without_trivia(db),
        None,
    );

    // Heuristically find the syntax node at the given offset.
    // We match the ancestors with node text to ensure we get the whole node.
    return find_syntax_node_at_offset(db, file, span.start)?
        .ancestors_with_self(db)
        .find(|node| node.get_text_without_trivia(db) == syntax_node.get_text_without_trivia(db));
}

#[tracing::instrument(level = "trace", skip(db))]
pub fn get_originating_syntax_node_for<'db>(
    db: &'db dyn Database,
    ptr: &SyntaxStablePtrId<'db>,
) -> Option<SyntaxNode<'db>> {
    let (file, span) = get_originating_location(
        db,
        ptr.file_id(db),
        ptr.lookup(db).span_without_trivia(db),
        None,
    );

    find_syntax_node_at_offset(db, file, span.start)
}

fn find_syntax_node_at_offset<'db>(
    db: &'db dyn Database,
    file: FileId<'db>,
    offset: TextOffset,
) -> Option<SyntaxNode<'db>> {
    Some(file_syntax(db, file).to_option()?.lookup_offset(db, offset))
}
