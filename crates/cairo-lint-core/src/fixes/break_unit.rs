use cairo_lang_syntax::node::{db::SyntaxGroup, SyntaxNode};

/// Rewrites `break ();` as `break;` given the node text contains it.
pub fn fix_break_unit(db: &dyn SyntaxGroup, node: SyntaxNode) -> Option<(SyntaxNode, String)> {
    Some((
        node.clone(),
        node.get_text(db).replace("break ();", "break;").to_string(),
    ))
}
