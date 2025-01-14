use cairo_lang_syntax::node::{
    ast::{Condition, ExprIf},
    db::SyntaxGroup,
    SyntaxNode, TypedSyntaxNode,
};

/// Rewrites a useless `if let` to a simple `if`
pub fn fix_equatable_if_let(
    db: &dyn SyntaxGroup,
    node: SyntaxNode,
) -> Option<(SyntaxNode, String)> {
    let expr = ExprIf::from_syntax_node(db, node.clone());
    let condition = expr.condition(db);

    let fixed_condition = match condition {
        Condition::Let(condition_let) => {
            format!(
                "{} == {} ",
                condition_let
                    .expr(db)
                    .as_syntax_node()
                    .get_text_without_trivia(db),
                condition_let
                    .patterns(db)
                    .as_syntax_node()
                    .get_text_without_trivia(db),
            )
        }
        _ => panic!("Incorrect diagnostic"),
    };

    Some((
        node,
        format!(
            "{}{}{}",
            expr.if_kw(db).as_syntax_node().get_text(db),
            fixed_condition,
            expr.if_block(db).as_syntax_node().get_text(db),
        ),
    ))
}
