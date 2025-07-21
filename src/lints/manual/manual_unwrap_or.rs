use cairo_lang_defs::ids::ModuleItemId;
use cairo_lang_defs::plugin::PluginDiagnostic;
use cairo_lang_diagnostics::Severity;
use cairo_lang_semantic::db::SemanticGroup;
use cairo_lang_syntax::node::{SyntaxNode, TypedStablePtr, TypedSyntaxNode, ast, db::SyntaxGroup};

use crate::{
    context::CairoLintKind,
    corelib::CorelibContext,
    fixer::InternalFix,
    queries::{get_all_function_bodies, get_all_if_expressions, get_all_match_expressions},
};
use crate::{
    context::Lint,
    lints::manual::{ManualLint, check_manual, check_manual_if},
};

pub struct ManualUnwrapOr;

/// ## What it does
///
/// Finds patterns that reimplement `Option::unwrap_or` or `Result::unwrap_or`.
///
/// ## Example
///
/// ```cairo
/// let foo: Option<i32> = None;
/// match foo {
///     Some(v) => v,
///     None => 1,
/// };
/// ```
///
/// Can be simplified to:
///
/// ```cairo
/// let foo: Option<i32> = None;
/// foo.unwrap_or(1);
/// ```
impl Lint for ManualUnwrapOr {
    fn allowed_name(&self) -> &'static str {
        "manual_unwrap_or"
    }

    fn diagnostic_message(&self) -> &'static str {
        "Manual `unwrap_or` detected. Consider using `unwrap_or()` instead."
    }

    fn kind(&self) -> CairoLintKind {
        CairoLintKind::ManualUnwrapOr
    }

    fn has_fixer(&self) -> bool {
        true
    }

    fn fix(&self, db: &dyn SemanticGroup, node: SyntaxNode) -> Option<InternalFix> {
        fix_manual_unwrap_or(db, node)
    }

    fn fix_message(&self) -> Option<&'static str> {
        Some("Use `unwrap_or()` instead of manual pattern")
    }
}

#[tracing::instrument(skip_all, level = "trace")]
pub fn check_manual_unwrap_or(
    db: &dyn SemanticGroup,
    _corelib_context: &CorelibContext,
    item: &ModuleItemId,
    diagnostics: &mut Vec<PluginDiagnostic>,
) {
    let function_bodies = get_all_function_bodies(db, item);
    for function_body in function_bodies {
        let if_exprs = get_all_if_expressions(&function_body);
        let match_exprs = get_all_match_expressions(&function_body);
        let arenas = &function_body.arenas;
        for match_expr in match_exprs {
            if check_manual(db, &match_expr, arenas, ManualLint::ManualUnwrapOr) {
                diagnostics.push(PluginDiagnostic {
                    stable_ptr: match_expr.stable_ptr.untyped(),
                    message: ManualUnwrapOr.diagnostic_message().to_owned(),
                    severity: Severity::Warning,
                    inner_span: None,
                });
            }
        }
        for if_expr in if_exprs {
            if check_manual_if(db, &if_expr, arenas, ManualLint::ManualUnwrapOr) {
                diagnostics.push(PluginDiagnostic {
                    stable_ptr: if_expr.stable_ptr.untyped(),
                    message: ManualUnwrapOr.diagnostic_message().to_owned(),
                    severity: Severity::Warning,
                    inner_span: None,
                });
            }
        }
    }
}

#[tracing::instrument(skip_all, level = "trace")]
fn fix_manual_unwrap_or(db: &dyn SyntaxGroup, node: SyntaxNode) -> Option<InternalFix> {
    let expr = ast::Expr::from_syntax_node(db, node);

    let (matched_expr, or_body) = match &expr {
        ast::Expr::Match(expr_match) => {
            let mut arms = expr_match.arms(db).elements(db);
            let matched_expr = expr_match.expr(db).as_syntax_node();
            let arm = arms.nth(1).expect("Expected a `match` with second arm.");

            let or_body = if let ast::Expr::Block(block) = arm.expression(db) {
                let block_statements = block.statements(db).node.get_text(db);

                // If the block has more than one line or a comment, we need to adjust the indentation.
                // Otherwise, we can remove `{ }` and whitespaces.
                if block_statements.lines().count() > 1
                    || block.as_syntax_node().get_text(db).contains("//")
                {
                    let (text, _) = get_adjusted_lines_and_indent(db, node, &arm);
                    text
                } else {
                    block_statements.trim().to_string()
                }
            } else {
                let expression_text = arm.expression(db).as_syntax_node().get_text(db);

                // Comments that are right behind the arrow.
                let arrow_trivia = arm
                    .arrow(db)
                    .trailing_trivia(db)
                    .node
                    .get_text(db)
                    .trim_end()
                    .to_string();

                // If the expression has more than one line or a comment after the arrow, we need to adjust the indentation.
                // Otherwise, we can remove whitespaces.
                if expression_text.lines().count() > 1 || arrow_trivia.contains("//") {
                    let (text, expression_bracket_indent) =
                        get_adjusted_lines_and_indent(db, node, &arm);
                    format!(
                        "{arrow_trivia}\n{}\n{}",
                        text,
                        " ".repeat(expression_bracket_indent)
                    )
                } else {
                    expression_text.trim().to_string()
                }
            };

            (matched_expr, or_body)
        }

        ast::Expr::If(expr_if) => {
            let mut conditions = expr_if.conditions(db).elements(db);
            let matched_expr = conditions
                .next()
                .expect("Expected an `if` with a condition.");
            let condition = match matched_expr {
                ast::Condition::Let(condition_let) => condition_let.expr(db).as_syntax_node(),
                _ => panic!("Expected an `if let` expression."),
            };

            let ast::OptionElseClause::ElseClause(else_clause) = expr_if.else_clause(db) else {
                panic!("No else clause found.");
            };

            let or_body = match else_clause.else_block_or_if(db) {
                ast::BlockOrIf::Block(block) => {
                    let mut text = block.statements(db).node.get_text(db);

                    // If the block has more than one line or a comment, we want the whole block.
                    if text.lines().count() > 1
                        || block.as_syntax_node().get_text(db).contains("//")
                    {
                        text = else_clause
                            .else_block_or_if(db)
                            .as_syntax_node()
                            .get_text(db);
                    }

                    text.trim().to_string()
                }
                // This case is not possible, because we check for standard `else` not `else if`
                ast::BlockOrIf::If(_) => panic!("Else if can not be changed to unwrap_or"),
            };

            (condition, or_body)
        }

        _ => panic!("The expression is expected to be either a `match` or an `if` statement."),
    };

    let indent = node
        .get_text(db)
        .chars()
        .take_while(|c| c.is_whitespace())
        .collect::<String>();

    Some(InternalFix {
        node,
        suggestion: format!(
            "{indent}{}.unwrap_or({or_body})",
            matched_expr.get_text(db).trim_end()
        ),
        description: ManualUnwrapOr.fix_message().unwrap().to_string(),
        import_addition_paths: None,
    })
}

// Adjusts the arm body indentation to align with the match closing bracket.
//
// Match arms typically have extra indentation that should be removed when converting to unwrap_or.
// The base indentation level is determined by the match arm's starting position.
fn get_adjusted_lines_and_indent(
    db: &(dyn SyntaxGroup),
    node: SyntaxNode,
    arm: &ast::MatchArm,
) -> (String, usize) {
    let arm_body_text = arm.expression(db).as_syntax_node().get_text(db);
    let lines: Vec<&str> = arm_body_text.lines().collect();

    let expression_text = node.get_text(db);
    let expression_bracket = expression_text.lines().last().unwrap();

    // Calculate the indentation of the `}` in the given expression
    let expression_bracket_indent = expression_bracket
        .chars()
        .take_while(|c| c.is_whitespace())
        .count();

    // Calculate the indentation of the 'match arm'
    let arm_ident = arm
        .as_syntax_node()
        .get_text(db)
        .chars()
        .take_while(|c| c.is_whitespace())
        .count();

    let difference = arm_ident.saturating_sub(expression_bracket_indent);

    // If the arm has unusual indentation, do not adjust it.
    if difference == 0 {
        return (arm_body_text, expression_bracket_indent);
    }

    let mut adjusted_lines = vec![];

    // Adjust the indentation of each subsequent line
    for line in lines.iter() {
        // Check if the substring up to 'difference' contains only whitespace
        if line.len() > difference && line[..difference].trim().is_empty() {
            let trimmed = &line[difference..];
            adjusted_lines.push(trimmed);
        } else {
            adjusted_lines.push(line);
        }
    }

    (adjusted_lines.join("\n"), expression_bracket_indent)
}
