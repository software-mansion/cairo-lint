use cairo_lang_syntax::node::{ast::Expr, db::SyntaxGroup, SyntaxNode, TypedSyntaxNode};

use super::helper::indent_snippet;

/// Removes unnecessary double parentheses from a syntax node.
///
/// Simplifies an expression by stripping extra layers of parentheses while preserving
/// the original formatting and indentation.
///
/// # Arguments
///
/// * `db` - Reference to the `SyntaxGroup` for syntax tree access.
/// * `node` - The `SyntaxNode` containing the expression.
///
/// # Returns
///
/// A `String` with the simplified expression.
///
/// # Example
///
/// Input: `((x + y))`
/// Output: `x + y`
pub fn fix_double_parens(db: &dyn SyntaxGroup, node: SyntaxNode) -> Option<(SyntaxNode, String)> {
    let mut expr = Expr::from_syntax_node(db, node.clone());

    while let Expr::Parenthesized(inner_expr) = expr {
        expr = inner_expr.expr(db);
    }

    Some((
        node.clone(),
        indent_snippet(
            &expr.as_syntax_node().get_text_without_trivia(db),
            node.get_text(db)
                .chars()
                .take_while(|c| c.is_whitespace())
                .collect::<String>()
                .len()
                / 4,
        ),
    ))
}
