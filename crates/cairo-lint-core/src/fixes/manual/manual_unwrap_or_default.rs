use cairo_lang_syntax::node::TypedSyntaxNode;
use cairo_lang_syntax::node::{
    ast::{Condition, Expr},
    db::SyntaxGroup,
    SyntaxNode,
};

/// Rewrites manual unwrap or default to use unwrap_or_default
pub fn fix_manual_unwrap_or_default(
    db: &dyn SyntaxGroup,
    node: SyntaxNode,
) -> Option<(SyntaxNode, String)> {
    // Check if the node is a general expression
    let expr = Expr::from_syntax_node(db, node.clone());

    let matched_expr = match expr {
        // Handle the case where the expression is a match expression
        Expr::Match(expr_match) => expr_match.expr(db).as_syntax_node(),

        // Handle the case where the expression is an if-let expression
        Expr::If(expr_if) => {
            // Extract the condition from the if-let expression
            let condition = expr_if.condition(db);

            match condition {
                Condition::Let(condition_let) => {
                    // Extract and return the syntax node for the matched expression
                    condition_let.expr(db).as_syntax_node()
                }
                _ => panic!("Expected an `if let` expression."),
            }
        }
        // Handle unsupported expressions
        _ => panic!("The expression cannot be simplified to `.unwrap_or_default()`."),
    };

    let indent = node
        .get_text(db)
        .chars()
        .take_while(|c| c.is_whitespace())
        .collect::<String>();
    Some((
        node,
        format!(
            "{indent}{}.unwrap_or_default()",
            matched_expr.get_text_without_trivia(db)
        ),
    ))
}
