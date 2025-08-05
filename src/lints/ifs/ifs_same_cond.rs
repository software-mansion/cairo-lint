use cairo_lang_defs::ids::ModuleItemId;
use cairo_lang_defs::plugin::PluginDiagnostic;
use cairo_lang_diagnostics::Severity;
use cairo_lang_semantic::db::SemanticGroup;
use cairo_lang_semantic::{Arenas, Condition, Expr, ExprFunctionCall, ExprFunctionCallArg, ExprIf};
use cairo_lang_syntax::node::{TypedStablePtr, TypedSyntaxNode};
use if_chain::if_chain;

use crate::context::{CairoLintKind, Lint};

use crate::LinterGroup;
use crate::queries::{get_all_function_bodies, get_all_if_expressions};

pub struct DuplicateIfCondition;

/// ## What it does
///
/// Checks for consecutive `if` expressions with the same condition.
///
/// ## Example
///
/// ```cairo
/// fn main() {
///     let a = 1;
///     let b = 1;
///     if a == b {
///         println!("a is equal to b");
///     } else if a == b {
///         println!("a is equal to b");
///     }
/// }
/// ```
///
/// Could be rewritten as just:
///
/// ```cairo
/// fn main() {
///     let a = 1;
///     let b = 1;
///     if a == b {
///         println!("a is equal to b");
///     }
/// }
/// ```
impl Lint for DuplicateIfCondition {
    fn allowed_name(&self) -> &'static str {
        "ifs_same_cond"
    }

    fn diagnostic_message(&self) -> &'static str {
        "Consecutive `if` with the same condition found."
    }

    fn kind(&self) -> CairoLintKind {
        CairoLintKind::DuplicateIfCondition
    }
}

#[tracing::instrument(skip_all, level = "trace")]
pub fn check_duplicate_if_condition<'db>(
    db: &'db dyn LinterGroup,
    item: &ModuleItemId<'db>,
    diagnostics: &mut Vec<PluginDiagnostic<'db>>,
) {
    let function_bodies = get_all_function_bodies(db, item);
    for function_body in function_bodies.iter() {
        let if_exprs = get_all_if_expressions(function_body);
        let arenas = &function_body.arenas;
        for if_expr in if_exprs.iter() {
            check_single_duplicate_if_condition(db, if_expr, arenas, diagnostics);
        }
    }
}

fn check_single_duplicate_if_condition<'db>(
    db: &'db dyn SemanticGroup,
    if_expr: &ExprIf<'db>,
    arenas: &Arenas<'db>,
    diagnostics: &mut Vec<PluginDiagnostic<'db>>,
) {
    let cond_expr = match &if_expr.conditions.first() {
        Some(Condition::BoolExpr(expr_id)) => &arenas.exprs[*expr_id],
        Some(Condition::Let(expr_id, _patterns)) => &arenas.exprs[*expr_id],
        _ => return,
    };

    if_chain! {
        if let Expr::FunctionCall(func_call) = cond_expr;
        if ensure_no_ref_arg(arenas, func_call);
        then {
            return;
        }
    }

    let mut current_block = if_expr.else_block;
    let if_condition_text = cond_expr
        .stable_ptr()
        .lookup(db)
        .as_syntax_node()
        .get_text(db);

    while let Some(expr_id) = current_block {
        if let Expr::If(else_if_block) = &arenas.exprs[expr_id] {
            current_block = else_if_block.else_block;
            let else_if_cond = match &else_if_block.conditions.first() {
                Some(Condition::BoolExpr(expr_id)) => &arenas.exprs[*expr_id],
                Some(Condition::Let(expr_id, _patterns)) => &arenas.exprs[*expr_id],
                _ => continue,
            };

            if_chain! {
                if let Expr::FunctionCall(func_call) = else_if_cond;
                if ensure_no_ref_arg(arenas, func_call);
                then {
                    continue;
                }
            }

            let else_if_condition_text = else_if_cond
                .stable_ptr()
                .lookup(db)
                .as_syntax_node()
                .get_text(db);

            if if_condition_text == else_if_condition_text {
                diagnostics.push(PluginDiagnostic {
                    stable_ptr: if_expr.stable_ptr.untyped(),
                    message: DuplicateIfCondition.diagnostic_message().to_string(),
                    severity: Severity::Warning,
                    inner_span: None,
                });
                break;
            }
        } else {
            break;
        }
    }
}

fn ensure_no_ref_arg<'db>(arenas: &Arenas<'db>, func_call: &ExprFunctionCall<'db>) -> bool {
    func_call.args.iter().any(|arg| match arg {
        ExprFunctionCallArg::Reference(_) => true,
        ExprFunctionCallArg::Value(expr_id) => match &arenas.exprs[*expr_id] {
            Expr::FunctionCall(expr_func) => ensure_no_ref_arg(arenas, expr_func),
            _ => false,
        },
    })
}
