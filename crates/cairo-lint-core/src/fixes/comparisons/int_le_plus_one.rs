use cairo_lang_syntax::node::{
    ast::{Expr, ExprBinary},
    db::SyntaxGroup,
    SyntaxNode, TypedSyntaxNode,
};

/// Rewrites a manual implementation of int le plus one x + 1 <= y
pub fn fix_int_le_plus_one(db: &dyn SyntaxGroup, node: SyntaxNode) -> Option<(SyntaxNode, String)> {
    let node = ExprBinary::from_syntax_node(db, node);
    let Expr::Binary(lhs_exp) = node.lhs(db) else {
        panic!("should be addition")
    };
    let rhs = node.rhs(db).as_syntax_node().get_text(db);

    let lhs = lhs_exp.lhs(db).as_syntax_node().get_text(db);

    let fix = format!("{} < {} ", lhs.trim(), rhs.trim());
    Some((node.as_syntax_node(), fix))
}
