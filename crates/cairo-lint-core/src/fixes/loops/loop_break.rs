use cairo_lang_syntax::node::{
    ast::{Expr, ExprLoop, OptionElseClause, Statement},
    db::SyntaxGroup,
    SyntaxNode, TypedSyntaxNode,
};

use crate::fixes::helper::{
    invert_condition, remove_break_from_block, remove_break_from_else_clause,
};

/// Converts a `loop` with a conditionally-breaking `if` statement into a `while` loop.
///
/// This function transforms loops that have a conditional `if` statement
/// followed by a `break` into a `while` loop, which can simplify the logic
/// and improve readability.
///
/// # Arguments
///
/// * `db` - Reference to the `SyntaxGroup` for syntax tree access.
/// * `node` - The `SyntaxNode` representing the loop expression.
///
/// # Returns
///
/// A `String` containing the transformed loop as a `while` loop, preserving
/// the original formatting and indentation.
///
/// # Example
///
/// ```
/// let mut x = 0;
/// loop {
///     if x > 5 {
///         break;
///     }
///     x += 1;
/// }
/// ```
///
/// Would be converted to:
///
/// ```
/// let mut x = 0;
/// while x <= 5 {
///     x += 1;
/// }
/// ```
pub fn fix_loop_break(db: &dyn SyntaxGroup, node: SyntaxNode) -> Option<(SyntaxNode, String)> {
    let loop_expr = ExprLoop::from_syntax_node(db, node.clone());
    let indent = node
        .get_text(db)
        .chars()
        .take_while(|c| c.is_whitespace())
        .collect::<String>();
    let mut condition_text = String::new();
    let mut loop_body = String::new();

    if let Some(Statement::Expr(expr_statement)) =
        loop_expr.body(db).statements(db).elements(db).first()
    {
        if let Expr::If(if_expr) = expr_statement.expr(db) {
            condition_text = invert_condition(
                &if_expr
                    .condition(db)
                    .as_syntax_node()
                    .get_text_without_trivia(db),
            );

            loop_body.push_str(&remove_break_from_block(db, if_expr.if_block(db), &indent));

            if let OptionElseClause::ElseClause(else_clause) = if_expr.else_clause(db) {
                loop_body.push_str(&remove_break_from_else_clause(db, else_clause, &indent));
            }
        }
    }

    for statement in loop_expr
        .body(db)
        .statements(db)
        .elements(db)
        .iter()
        .skip(1)
    {
        loop_body.push_str(&format!(
            "{}    {}\n",
            indent,
            statement.as_syntax_node().get_text_without_trivia(db)
        ));
    }

    Some((
        node,
        format!(
            "{}while {} {{\n{}{}}}\n",
            indent, condition_text, loop_body, indent
        ),
    ))
}
