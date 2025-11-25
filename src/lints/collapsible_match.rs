use cairo_lang_defs::{ids::ModuleItemId, plugin::PluginDiagnostic};
use cairo_lang_diagnostics::Severity;
use cairo_lang_semantic::{Arenas, Expr, ExprMatch, MatchArm, Statement, VarId};
use cairo_lang_syntax::node::{SyntaxNode, TypedStablePtr, TypedSyntaxNode};
use salsa::Database;

use crate::{
    context::{CairoLintKind, Lint},
    fixer::InternalFix,
    lints::manual::helpers::extract_pattern_variable,
    queries::{get_all_function_bodies, get_all_match_expressions},
};

pub struct CollapsibleMatch;

/// ## What it does
///
/// Checks for nested `match` statements that can be collapsed into a single `match` statement.
/// Note that this lint is not intended to find all cases where nested match patterns can be merged, but only cases where merging would most likely make the code more readable.
/// ## Example
///
/// ```cairo
/// fn func(opt: Option<Result<u32, felt252>>) {
///     let n = match opt {
///         Some(n) => match n {
///             Ok(n) => n,
///             _ => return,
///         }
///         None => return,
///     };
/// }
/// ```
///
/// Can be collapsed to
///
/// ```cairo
/// fn func(opt: Option<Result<u32, felt252>>) {
///     let n = match opt {
///         Some(Ok(n)) => n,
///         _ => return,
///     };
/// }
/// ```
impl Lint for CollapsibleMatch {
    fn allowed_name(&self) -> &'static str {
        "collapsible_match"
    }

    fn diagnostic_message(&self) -> &'static str {
        "Nested `match` statements can be collapsed into a single `match` statement."
    }

    fn kind(&self) -> CairoLintKind {
        CairoLintKind::CollapsibleMatch
    }

    fn has_fixer(&self) -> bool {
        true
    }

    fn fix<'db>(&self, db: &'db dyn Database, node: SyntaxNode<'db>) -> Option<InternalFix<'db>> {
        fix_collapsible_match(db, node)
    }

    fn fix_message(&self) -> Option<&'static str> {
        Some("Combine nested matches into a single match")
    }
}

#[tracing::instrument(skip_all, level = "trace")]
pub fn check_collapsible_match<'db>(
    db: &'db dyn Database,
    item: &ModuleItemId<'db>,
    diagnostics: &mut Vec<PluginDiagnostic<'db>>,
) {
    let function_bodies = get_all_function_bodies(db, item);
    for function_body in function_bodies.iter() {
        let match_exprs = get_all_match_expressions(function_body);
        let arenas = &function_body.arenas;
        for match_expr in match_exprs {
            check_single_collapsible_match(db, &match_expr, arenas, diagnostics);
        }
    }
}

fn check_single_collapsible_match<'db>(
    db: &'db dyn Database,
    match_expr: &ExprMatch<'db>,
    arenas: &Arenas<'db>,
    diagnostics: &mut Vec<PluginDiagnostic<'db>>,
) {
    let arms = &match_expr.arms;

    if arms.len() != 2 {
        return;
    }

    let first_arm = &arms[0];
    let second_arm = &arms[1];

    let first_inner_match = get_inner_match_expression_if_single_one(first_arm, arenas);
    let second_inner_match = get_inner_match_expression_if_single_one(second_arm, arenas);

    // We only support the case, where there is only a single inner match.
    match (first_inner_match, second_inner_match) {
        (Some(_), Some(_)) => {
            return;
        }
        (None, None) => {
            return;
        }
        _ => {}
    }

    if let Some(inner_match) = first_inner_match {
        if check_if_matches_are_simplifiable(db, arenas, first_arm, second_arm, inner_match) {
            diagnostics.push(PluginDiagnostic {
                stable_ptr: match_expr.stable_ptr.untyped(),
                message: CollapsibleMatch.diagnostic_message().to_string(),
                severity: Severity::Warning,
                inner_span: None,
            });
        }
    }

    if let Some(inner_match) = second_inner_match {
        if check_if_matches_are_simplifiable(db, arenas, second_arm, first_arm, inner_match) {
            diagnostics.push(PluginDiagnostic {
                stable_ptr: match_expr.stable_ptr.untyped(),
                message: CollapsibleMatch.diagnostic_message().to_string(),
                severity: Severity::Warning,
                inner_span: None,
            });
        }
    }
}

fn check_if_matches_are_simplifiable<'db>(
    db: &'db dyn Database,
    arenas: &Arenas<'db>,
    arm_with_inner_match: &MatchArm,
    secondary_arm: &MatchArm,
    inner_match: &ExprMatch<'db>,
) -> bool {
    if inner_match.arms.len() != 2 {
        return false;
    }

    // If there is no match of expressions between outer arm and any of the inner arms, we don't go for diagnostics.
    if !check_if_match_arm_has_same_expression_as_inner_match_ars(
        db,
        arenas,
        secondary_arm,
        inner_match,
    ) {
        return false;
    }

    // We don't support multiple patterns in the outer match arm, as it's not a good idea to try to "merge" them with the inner one.
    // It would likely lead to less readable code.
    if arm_with_inner_match.patterns.len() != 1 {
        return false;
    }
    let pattern = &arenas.patterns[arm_with_inner_match.patterns[0]];
    let Some(pattern_variable) = extract_pattern_variable(pattern, arenas) else {
        return false;
    };

    let inner_matched_expr = &arenas.exprs[inner_match.matched_expr];

    let Expr::Var(inner_var_expr) = inner_matched_expr else {
        return false;
    };

    let VarId::Local(inner_local_var_id) = inner_var_expr.var else {
        return false;
    };

    inner_local_var_id == pattern_variable.var.id
}

/// Checks whether the match arm's expression is the same as one of the inner match arms' expressions.
fn check_if_match_arm_has_same_expression_as_inner_match_ars<'db>(
    db: &'db dyn Database,
    arenas: &Arenas<'db>,
    match_arm: &MatchArm,
    inner_match: &ExprMatch<'db>,
) -> bool {
    let second_arm_expression_text = &arenas.exprs[match_arm.expression]
        .stable_ptr()
        .lookup(db)
        .as_syntax_node()
        .get_text_without_trivia(db);

    let inner_first_arm_expression_text = &arenas.exprs[inner_match.arms[0].expression]
        .stable_ptr()
        .lookup(db)
        .as_syntax_node()
        .get_text_without_trivia(db);

    let inner_second_arm_expression_text = &arenas.exprs[inner_match.arms[1].expression]
        .stable_ptr()
        .lookup(db)
        .as_syntax_node()
        .get_text_without_trivia(db);

    inner_first_arm_expression_text == second_arm_expression_text
        || inner_second_arm_expression_text == second_arm_expression_text
}

#[tracing::instrument(skip_all, level = "trace")]
pub fn fix_collapsible_match<'db>(
    db: &'db dyn Database,
    node: SyntaxNode<'db>,
) -> Option<InternalFix<'db>> {
    None
}

/// Gets the inner match expression from a match arm if only the inner match is the only expression.
fn get_inner_match_expression_if_single_one<'db>(
    match_arm: &'db MatchArm,
    arenas: &'db Arenas<'db>,
) -> Option<&'db ExprMatch<'db>> {
    let arm_expression = &arenas.exprs[match_arm.expression];

    match arm_expression {
        Expr::Match(inner_match) => Some(inner_match),
        Expr::Block(block) => match block.statements.len() {
            // If there's not statements, check the tail for match expression.
            0 => {
                if let Some(expr_id) = block.tail
                    && let Expr::Match(inner_match) = &arenas.exprs[expr_id]
                {
                    Some(inner_match)
                } else {
                    None
                }
            }
            // If there's a single statement, check if it's a match expression.
            1 => {
                let first_statement = &block.statements[0];
                if let Statement::Expr(statement_expr) = &arenas.statements[*first_statement]
                    && let Expr::Match(inner_match) = &arenas.exprs[statement_expr.expr]
                {
                    Some(inner_match)
                } else {
                    None
                }
            }
            _ => None,
        },
        _ => None,
    }
}
