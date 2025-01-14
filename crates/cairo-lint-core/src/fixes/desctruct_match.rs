use cairo_lang_syntax::node::TypedSyntaxNode;
use cairo_lang_syntax::node::{
    ast::{ExprMatch, Pattern},
    db::SyntaxGroup,
    SyntaxNode,
};

use crate::lints::single_match::is_expr_unit;

use super::helper::indent_snippet;

/// Fixes a destructuring match by converting it to an if-let expression.
///
/// This method handles matches with two arms, where one arm is a wildcard (_)
/// and the other is either an enum or struct pattern.
///
/// # Arguments
///
/// * `db` - A reference to the SyntaxGroup
/// * `node` - The SyntaxNode representing the match expression
///
/// # Returns
///
/// A `String` containing the if-let expression that replaces the match.
///
/// # Panics
///
/// Panics if the diagnostic is incorrect (i.e., the match doesn't have the expected structure).
pub fn fix_destruct_match(db: &dyn SyntaxGroup, node: SyntaxNode) -> Option<(SyntaxNode, String)> {
    let match_expr = ExprMatch::from_syntax_node(db, node.clone());
    let arms = match_expr.arms(db).elements(db);
    let first_arm = &arms[0];
    let second_arm = &arms[1];
    let (pattern, first_expr) = match (
        &first_arm.patterns(db).elements(db)[0],
        &second_arm.patterns(db).elements(db)[0],
    ) {
        (Pattern::Underscore(_), Pattern::Enum(pat)) => (pat.as_syntax_node(), second_arm),
        (Pattern::Enum(pat), Pattern::Underscore(_)) => (pat.as_syntax_node(), first_arm),
        (Pattern::Underscore(_), Pattern::Struct(pat)) => (pat.as_syntax_node(), second_arm),
        (Pattern::Struct(pat), Pattern::Underscore(_)) => (pat.as_syntax_node(), first_arm),
        (Pattern::Enum(pat1), Pattern::Enum(pat2)) => {
            if is_expr_unit(second_arm.expression(db), db) {
                (pat1.as_syntax_node(), first_arm)
            } else {
                (pat2.as_syntax_node(), second_arm)
            }
        }
        (_, _) => panic!("Incorrect diagnostic"),
    };
    let mut pattern_span = pattern.span(db);
    pattern_span.end = pattern.span_start_without_trivia(db);
    let indent = node
        .get_text(db)
        .chars()
        .take_while(|c| c.is_whitespace())
        .collect::<String>();
    let trivia = pattern.clone().get_text_of_span(db, pattern_span);
    Some((
        node,
        indent_snippet(
            &format!(
                "{trivia}{indent}if let {} = {} {{\n{}\n}}",
                pattern.get_text_without_trivia(db),
                match_expr
                    .expr(db)
                    .as_syntax_node()
                    .get_text_without_trivia(db),
                first_expr
                    .expression(db)
                    .as_syntax_node()
                    .get_text_without_trivia(db),
            ),
            indent.len() / 4,
        ),
    ))
}
