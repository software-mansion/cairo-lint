//! This module provides functionality to handle any code that comes as the product of procedural macros.

use cairo_lang_diagnostics::ToOption;
use cairo_lang_filesystem::{db::get_originating_location, ids::FileId, span::TextOffset};
use cairo_lang_parser::db::ParserGroup;
use cairo_lang_semantic::db::SemanticGroup;
use cairo_lang_syntax::node::{ids::SyntaxStablePtrId, SyntaxNode};

fn find_syntax_node_at_offset(
    db: &dyn ParserGroup,
    file: FileId,
    offset: TextOffset,
) -> Option<SyntaxNode> {
    Some(db.file_syntax(file).to_option()?.lookup_offset(db, offset))
}

pub fn get_originating_syntax_node_for(
    db: &dyn SemanticGroup,
    ptr: &SyntaxStablePtrId,
) -> Option<SyntaxNode> {
    let (file, span) = get_originating_location(
        db,
        ptr.file_id(db),
        ptr.lookup(db).span_without_trivia(db),
        None,
    );

    find_syntax_node_at_offset(db.upcast(), file, span.start)
}
