use cairo_lang_defs::ids::ModuleItemId;
use cairo_lang_defs::plugin::PluginDiagnostic;
use cairo_lang_diagnostics::Severity;
use cairo_lang_semantic::db::SemanticGroup;
use cairo_lang_semantic::{Arenas, Condition, Expr, ExprFunctionCall, ExprFunctionCallArg, ExprIf};
use cairo_lang_syntax::node::{TypedStablePtr, TypedSyntaxNode};
use if_chain::if_chain;

use crate::context::{CairoLintKind, Lint};
use crate::queries::{get_all_function_bodies, get_all_if_expressions};

const DUPLICATE_IF_CONDITION: &str = "Consecutive `if` with the same condition found.";
const DUPLICATE_IF_CONDITION_LINT_NAME: &str = "ifs_same_cond";

pub struct DuplicateIfCondition;

impl Lint for DuplicateIfCondition {
    fn allowed_name(self: &Self) -> &'static str {
        DUPLICATE_IF_CONDITION_LINT_NAME
    }

    fn diagnostic_message(self: &Self) -> &'static str {
        DUPLICATE_IF_CONDITION
    }

    fn kind(self: &Self) -> CairoLintKind {
        CairoLintKind::DuplicateIfCondition
    }
}

pub fn check_duplicate_if_condition(
    db: &dyn SemanticGroup,
    item: &ModuleItemId,
    diagnostics: &mut Vec<PluginDiagnostic>,
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

fn check_single_duplicate_if_condition(
    db: &dyn SemanticGroup,
    if_expr: &ExprIf,
    arenas: &Arenas,
    diagnostics: &mut Vec<PluginDiagnostic>,
) {
    let cond_expr = match &if_expr.condition {
        Condition::BoolExpr(expr_id) => &arenas.exprs[*expr_id],
        Condition::Let(expr_id, _patterns) => &arenas.exprs[*expr_id],
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
        .lookup(db.upcast())
        .as_syntax_node()
        .get_text_without_trivia(db.upcast());

    while let Some(expr_id) = current_block {
        if let Expr::If(else_if_block) = &arenas.exprs[expr_id] {
            current_block = else_if_block.else_block;
            let else_if_cond = match &else_if_block.condition {
                Condition::BoolExpr(expr_id) => &arenas.exprs[*expr_id],
                Condition::Let(expr_id, _patterns) => &arenas.exprs[*expr_id],
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
                .lookup(db.upcast())
                .as_syntax_node()
                .get_text_without_trivia(db.upcast());

            if if_condition_text == else_if_condition_text {
                diagnostics.push(PluginDiagnostic {
                    stable_ptr: if_expr.stable_ptr.untyped(),
                    message: DUPLICATE_IF_CONDITION.to_string(),
                    severity: Severity::Warning,
                });
                break;
            }
        } else {
            break;
        }
    }
}

fn ensure_no_ref_arg(arenas: &Arenas, func_call: &ExprFunctionCall) -> bool {
    func_call.args.iter().any(|arg| match arg {
        ExprFunctionCallArg::Reference(_) => true,
        ExprFunctionCallArg::Value(expr_id) => match &arenas.exprs[*expr_id] {
            Expr::FunctionCall(expr_func) => ensure_no_ref_arg(arenas, expr_func),
            _ => false,
        },
    })
}
