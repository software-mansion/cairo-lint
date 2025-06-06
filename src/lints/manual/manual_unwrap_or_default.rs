use cairo_lang_defs::ids::ModuleItemId;
use cairo_lang_defs::plugin::PluginDiagnostic;
use cairo_lang_diagnostics::Severity;
use cairo_lang_semantic::db::SemanticGroup;
use cairo_lang_syntax::node::{
    ast::{Condition, Expr},
    db::SyntaxGroup,
    SyntaxNode, TypedStablePtr, TypedSyntaxNode,
};

use crate::{
    context::CairoLintKind,
    fixes::InternalFix,
    queries::{get_all_function_bodies, get_all_if_expressions, get_all_match_expressions},
};
use crate::{
    context::Lint,
    lints::manual::{check_manual, check_manual_if, ManualLint},
};

pub struct ManualUnwrapOrDefault;

/// ## What it does
///
/// Checks for manual unwrapping of an Option or Result.
///
/// ## Example
///
/// ```cairo
/// fn main() {
///     let x: Option<u128> = Option::Some(1038);
///     if let Option::Some(v) = x {
///         v
///     } else {
///         0
///     };
/// }
/// ```
///
/// Can be simplified to:
///
/// ```cairo
/// fn main() {
///     let x: Option<u128> = Option::Some(1038);
///     x.unwrap_or_default();
/// }
/// ```
impl Lint for ManualUnwrapOrDefault {
    fn allowed_name(&self) -> &'static str {
        "manual_unwrap_or_default"
    }

    fn diagnostic_message(&self) -> &'static str {
        "This can be done in one call with `.unwrap_or_default()`"
    }

    fn kind(&self) -> CairoLintKind {
        CairoLintKind::ManualUnwrapOrDefault
    }

    fn has_fixer(&self) -> bool {
        true
    }

    fn fix(&self, db: &dyn SemanticGroup, node: SyntaxNode) -> Option<InternalFix> {
        fix_manual_unwrap_or_default(db.upcast(), node)
    }

    fn fix_message(&self) -> Option<&'static str> {
        Some("Use `unwrap_or_default()` instead of manual pattern")
    }
}

pub fn check_manual_unwrap_or_default(
    db: &dyn SemanticGroup,
    item: &ModuleItemId,
    diagnostics: &mut Vec<PluginDiagnostic>,
) {
    let function_bodies = get_all_function_bodies(db, item);
    for function_body in function_bodies.iter() {
        let if_exprs = get_all_if_expressions(function_body);
        let match_exprs = get_all_match_expressions(function_body);
        let arenas = &function_body.arenas;
        for match_expr in match_exprs.iter() {
            if check_manual(db, match_expr, arenas, ManualLint::ManualUnwrapOrDefault) {
                diagnostics.push(PluginDiagnostic {
                    stable_ptr: match_expr.stable_ptr.untyped(),
                    message: ManualUnwrapOrDefault.diagnostic_message().to_owned(),
                    severity: Severity::Warning,
                    relative_span: None,
                    inner_span: None,
                });
            }
        }
        for if_expr in if_exprs.iter() {
            if check_manual_if(db, if_expr, arenas, ManualLint::ManualUnwrapOrDefault) {
                diagnostics.push(PluginDiagnostic {
                    stable_ptr: if_expr.stable_ptr.untyped(),
                    message: ManualUnwrapOrDefault.diagnostic_message().to_owned(),
                    severity: Severity::Warning,
                    relative_span: None,
                    inner_span: None,
                });
            }
        }
    }
}

/// Rewrites manual unwrap or default to use unwrap_or_default
pub fn fix_manual_unwrap_or_default(db: &dyn SyntaxGroup, node: SyntaxNode) -> Option<InternalFix> {
    // Check if the node is a general expression
    let expr = Expr::from_syntax_node(db, node);

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

    let mut loop_span = node.span(db);
    loop_span.end = node.span_start_without_trivia(db);
    let trivia = node.get_text_of_span(db, loop_span).trim().to_string();
    let trivia = if trivia.is_empty() {
        trivia
    } else {
        format!("{indent}{trivia}\n")
    };
    Some(InternalFix {
        node,
        suggestion: format!(
            "{trivia}{indent}{}.unwrap_or_default()",
            matched_expr.get_text(db).trim_end()
        ),
        description: ManualUnwrapOrDefault.fix_message().unwrap().to_string(),
        import_addition_paths: None,
    })
}
