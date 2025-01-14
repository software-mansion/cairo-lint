use cairo_lang_syntax::node::{
    ast::{Expr, ExprLoop, OptionPatternEnumInnerPattern, Pattern, Statement},
    db::SyntaxGroup,
    SyntaxNode, TypedSyntaxNode,
};
use if_chain::if_chain;

use crate::fixes::helper::indent_snippet;

/// Rewrites this:
///
/// ```ignore
/// loop {
///     match some_span.pop_front() {
///         Option::Some(val) => do_smth(val),
///         Option::None => break;
///     }
/// }
/// ```
/// to this:
/// ```ignore
/// for val in span {
///     do_smth(val);
/// };
/// ```
pub fn fix_loop_match_pop_front(
    db: &dyn SyntaxGroup,
    node: SyntaxNode,
) -> Option<(SyntaxNode, String)> {
    let expr_loop = ExprLoop::from_syntax_node(db, node.clone());
    let body = expr_loop.body(db);
    let Statement::Expr(expr) = &body.statements(db).elements(db)[0] else {
        panic!(
            "Wrong statement type. This is probably a bug in the lint detection. Please report it"
        )
    };
    let Expr::Match(expr_match) = expr.expr(db) else {
        panic!(
            "Wrong expression type. This is probably a bug in the lint detection. Please report it"
        )
    };
    let val = expr_match.expr(db);
    let span_name = match val {
        Expr::FunctionCall(func_call) => func_call.arguments(db).arguments(db).elements(db)[0]
            .arg_clause(db)
            .as_syntax_node()
            .get_text_without_trivia(db),
        Expr::Binary(dot_call) => dot_call
            .lhs(db)
            .as_syntax_node()
            .get_text_without_trivia(db),
        _ => panic!(
            "Wrong expressiin type. This is probably a bug in the lint detection. Please report it"
        ),
    };
    let mut elt_name = "".to_owned();
    let mut some_arm = "".to_owned();
    let arms = expr_match.arms(db).elements(db);

    let mut loop_span = node.span(db);
    loop_span.end = node.span_start_without_trivia(db);
    let indent = node
        .get_text(db)
        .chars()
        .take_while(|c| c.is_whitespace())
        .collect::<String>();
    let trivia = node.clone().get_text_of_span(db, loop_span);
    let trivia = if trivia.is_empty() {
        trivia
    } else {
        format!("{indent}{trivia}\n")
    };
    for arm in arms {
        if_chain! {
            if let Pattern::Enum(enum_pattern) = &arm.patterns(db).elements(db)[0];
            if let OptionPatternEnumInnerPattern::PatternEnumInnerPattern(var) = enum_pattern.pattern(db);
            then {
                elt_name = var.pattern(db).as_syntax_node().get_text_without_trivia(db);
                some_arm = if let Expr::Block(block_expr) = arm.expression(db) {
                    block_expr.statements(db).as_syntax_node().get_text(db)
                } else {
                    arm.expression(db).as_syntax_node().get_text(db)
                }
            }
        }
    }
    Some((
        node,
        indent_snippet(
            &format!("{trivia}for {elt_name} in {span_name} {{\n{some_arm}\n}};\n"),
            indent.len() / 4,
        ),
    ))
}
