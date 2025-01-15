use cairo_lang_syntax::node::{ast::ExprBinary, db::SyntaxGroup, SyntaxNode, TypedSyntaxNode};

use crate::lints::bool_comparison::generate_fixed_text_for_comparison;

/// Rewrites a bool comparison to a simple bool. Ex: `some_bool == false` would be rewritten to
/// `!some_bool`
pub fn fix_bool_comparison(db: &dyn SyntaxGroup, node: SyntaxNode) -> Option<(SyntaxNode, String)> {
    let node = ExprBinary::from_syntax_node(db, node);
    let lhs = node.lhs(db).as_syntax_node().get_text(db);
    let rhs = node.rhs(db).as_syntax_node().get_text(db);

    let result = generate_fixed_text_for_comparison(db, lhs.as_str(), rhs.as_str(), node.clone());
    Some((node.as_syntax_node(), result))
}
