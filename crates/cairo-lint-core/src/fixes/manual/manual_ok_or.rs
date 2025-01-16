use cairo_lang_syntax::node::{
    ast::{ExprIf, ExprMatch},
    db::SyntaxGroup,
    kind::SyntaxKind,
    SyntaxNode, TypedSyntaxNode,
};

use crate::fixes::manual::helpers::{
    expr_if_get_var_name_and_err, expr_match_get_var_name_and_err,
};

/// Rewrites a manual implementation of ok_or
pub fn fix_manual_ok_or(db: &dyn SyntaxGroup, node: SyntaxNode) -> Option<(SyntaxNode, String)> {
    let fix = match node.kind(db) {
        SyntaxKind::ExprMatch => {
            let expr_match = ExprMatch::from_syntax_node(db, node.clone());

            let (option_var_name, none_arm_err) =
                expr_match_get_var_name_and_err(expr_match, db, 1);

            format!("{option_var_name}.ok_or({none_arm_err})")
        }
        SyntaxKind::ExprIf => {
            let expr_if = ExprIf::from_syntax_node(db, node.clone());

            let (option_var_name, err) = expr_if_get_var_name_and_err(expr_if, db);

            format!("{option_var_name}.ok_or({err})")
        }
        _ => panic!("SyntaxKind should be either ExprIf or ExprMatch"),
    };
    Some((node, fix))
}
