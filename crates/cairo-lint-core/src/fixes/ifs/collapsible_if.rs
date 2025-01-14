use cairo_lang_syntax::node::{
    ast::{Expr, ExprIf, OptionElseClause, Statement},
    db::SyntaxGroup,
    SyntaxNode, TypedSyntaxNode,
};

use crate::fixes::helper::indent_snippet;

/// Attempts to fix a collapsible if-statement by combining its conditions.
/// This function detects nested `if` statements where the inner `if` can be collapsed
/// into the outer one by combining their conditions with `&&`. It reconstructs the
/// combined condition and the inner block, preserving the indentation and formatting.
///
/// # Arguments
///
/// * `db` - A reference to the `SyntaxGroup`, which provides access to the syntax tree.
/// * `node` - A `SyntaxNode` representing the outer `if` statement that might be collapsible.
///
/// # Returns
///
/// A `String` containing the fixed code with the combined conditions if a collapsible
/// `if` is found. If no collapsible `if` is detected, the original text of the node is
/// returned.
pub fn fix_collapsible_if(db: &dyn SyntaxGroup, node: SyntaxNode) -> Option<(SyntaxNode, String)> {
    let expr_if = ExprIf::from_syntax_node(db, node.clone());
    let outer_condition = expr_if
        .condition(db)
        .as_syntax_node()
        .get_text_without_trivia(db);
    let if_block = expr_if.if_block(db);

    let statements = if_block.statements(db).elements(db);
    if statements.len() != 1 {
        return None;
    }

    if let Some(Statement::Expr(inner_expr_stmt)) = statements.first() {
        if let Expr::If(inner_if_expr) = inner_expr_stmt.expr(db) {
            match inner_if_expr.else_clause(db) {
                OptionElseClause::Empty(_) => {}
                OptionElseClause::ElseClause(_) => {
                    return None;
                }
            }

            match expr_if.else_clause(db) {
                OptionElseClause::Empty(_) => {}
                OptionElseClause::ElseClause(_) => {
                    return None;
                }
            }

            let inner_condition = inner_if_expr
                .condition(db)
                .as_syntax_node()
                .get_text_without_trivia(db);
            let combined_condition = format!("({}) && ({})", outer_condition, inner_condition);
            let inner_if_block = inner_if_expr.if_block(db).as_syntax_node().get_text(db);

            let indent = expr_if
                .if_kw(db)
                .as_syntax_node()
                .get_text(db)
                .chars()
                .take_while(|c| c.is_whitespace())
                .count();
            return Some((
                node,
                indent_snippet(
                    &format!("if {} {}", combined_condition, inner_if_block,),
                    indent / 4,
                ),
            ));
        }
    }
    None
}
