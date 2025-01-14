use cairo_lang_syntax::node::{
    ast::{Expr, ExprBinary},
    db::SyntaxGroup,
    SyntaxNode, TypedSyntaxNode,
};

/// Rewrites a manual implementation of int ge min one x - 1 >= y
pub fn fix_int_ge_min_one(db: &dyn SyntaxGroup, node: ExprBinary) -> Option<(SyntaxNode, String)> {
    let Expr::Binary(lhs_exp) = node.lhs(db) else {
        panic!("should be substraction")
    };
    let rhs = node.rhs(db).as_syntax_node().get_text(db);

    let lhs = lhs_exp.lhs(db).as_syntax_node().get_text(db);

    let fix = format!("{} > {} ", lhs.trim(), rhs.trim());
    Some((node.as_syntax_node(), fix))
}
