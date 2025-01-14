use cairo_lang_syntax::node::{
    ast::{Expr, ExprBinary},
    db::SyntaxGroup,
    SyntaxNode, TypedSyntaxNode,
};

/// Rewrites a manual implementation of int le min one x <= y -1
pub fn fix_int_le_min_one(db: &dyn SyntaxGroup, node: ExprBinary) -> Option<(SyntaxNode, String)> {
    let lhs = node.lhs(db).as_syntax_node().get_text(db);

    let Expr::Binary(rhs_exp) = node.rhs(db) else {
        panic!("should be substraction")
    };
    let rhs = rhs_exp.lhs(db).as_syntax_node().get_text(db);

    let fix = format!("{} < {} ", lhs.trim(), rhs.trim());
    Some((node.as_syntax_node(), fix))
}
