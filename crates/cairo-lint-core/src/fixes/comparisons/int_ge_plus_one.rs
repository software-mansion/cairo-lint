use cairo_lang_syntax::node::{
    ast::{Expr, ExprBinary},
    db::SyntaxGroup,
    SyntaxNode, TypedSyntaxNode,
};

/// Rewrites a manual implementation of int ge plus one x >= y + 1
pub fn fix_int_ge_plus_one(db: &dyn SyntaxGroup, node: SyntaxNode) -> Option<(SyntaxNode, String)> {
    let node = ExprBinary::from_syntax_node(db, node);
    let lhs = node.lhs(db).as_syntax_node().get_text(db);

    let Expr::Binary(rhs_exp) = node.rhs(db) else {
        panic!("should be addition")
    };
    let rhs = rhs_exp.lhs(db).as_syntax_node().get_text(db);

    let fix = format!("{} > {} ", lhs.trim(), rhs.trim());
    Some((node.as_syntax_node(), fix))
}
