use cairo_lang_syntax::node::{
    ast::{BlockOrIf, Expr, ExprIf, OptionElseClause, Statement},
    db::SyntaxGroup,
    SyntaxNode, TypedSyntaxNode,
};

/// Transforms nested `if-else` statements into a more compact `if-else if` format.
///
/// Simplifies an expression by converting nested `if-else` structures into a single `if-else
/// if` statement while preserving the original formatting and indentation.
///
/// # Arguments
///
/// * `db` - Reference to the `SyntaxGroup` for syntax tree access.
/// * `node` - The `SyntaxNode` containing the expression.
///
/// # Returns
///
/// A `String` with the refactored `if-else` structure.
pub fn fix_collapsible_if_else(
    db: &dyn SyntaxGroup,
    if_expr: &ExprIf,
) -> Option<(SyntaxNode, String)> {
    let OptionElseClause::ElseClause(else_clause) = if_expr.else_clause(db) else {
        return None;
    };
    if let BlockOrIf::Block(block_expr) = else_clause.else_block_or_if(db) {
        if let Some(Statement::Expr(statement_expr)) =
            block_expr.statements(db).elements(db).first()
        {
            if let Expr::If(if_expr) = statement_expr.expr(db) {
                // Construct the new "else if" expression
                let condition = if_expr.condition(db).as_syntax_node().get_text(db);
                let if_body = if_expr.if_block(db).as_syntax_node().get_text(db);
                let else_body = if_expr.else_clause(db).as_syntax_node().get_text(db);

                // Preserve original indentation
                let original_indent = else_clause
                    .as_syntax_node()
                    .get_text(db)
                    .chars()
                    .take_while(|c| c.is_whitespace())
                    .collect::<String>();

                return Some((
                    else_clause.as_syntax_node(),
                    format!(
                        "{}else if {} {} {}",
                        original_indent, condition, if_body, else_body
                    ),
                ));
            }
        }
    }

    // If we can't transform it, return the original text
    None
}
