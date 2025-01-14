use cairo_lang_syntax::node::TypedSyntaxNode;
use cairo_lang_syntax::node::{ast::Expr, db::SyntaxGroup, SyntaxNode};

use crate::lints::double_comparison::{
    determine_simplified_operator, extract_binary_operator_expr, operator_to_replace,
};

/// Rewrites a double comparison. Ex: `a > b || a == b` to `a >= b`
pub fn fix_double_comparison(
    db: &dyn SyntaxGroup,
    node: SyntaxNode,
) -> Option<(SyntaxNode, String)> {
    let expr = Expr::from_syntax_node(db, node.clone());

    if let Expr::Binary(binary_op) = expr {
        let lhs = binary_op.lhs(db);
        let rhs = binary_op.rhs(db);
        let middle_op = binary_op.op(db);

        if let (Some(lhs_op), Some(rhs_op)) = (
            extract_binary_operator_expr(&lhs, db),
            extract_binary_operator_expr(&rhs, db),
        ) {
            let simplified_op = determine_simplified_operator(&lhs_op, &rhs_op, &middle_op);

            if let Some(simplified_op) = simplified_op {
                if let Some(operator_to_replace) = operator_to_replace(lhs_op) {
                    let lhs_text = lhs
                        .as_syntax_node()
                        .get_text(db)
                        .replace(operator_to_replace, simplified_op);
                    return Some((node, lhs_text.to_string()));
                }
            }
        }
    }

    None
}
