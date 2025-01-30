use cairo_lang_defs::ids::ModuleItemId;
use cairo_lang_defs::plugin::PluginDiagnostic;
use cairo_lang_diagnostics::Severity;
use cairo_lang_semantic::db::SemanticGroup;
use cairo_lang_semantic::{Arenas, Expr, ExprFunctionCall, ExprFunctionCallArg};
use cairo_lang_syntax::node::TypedStablePtr;
use num_bigint::BigInt;

use super::{function_trait_name_from_fn_id, AND};
use crate::context::{CairoLintKind, Lint};
use crate::lints::{DIV, MUL};
use crate::queries::{get_all_function_bodies, get_all_function_calls};

const ERASING_OPERATION: &str =
    "This operation results in the value being erased (e.g., multiplication by 0). \
                                     Consider replacing the entire expression with 0.";

const ERASING_OPERATION_LINT_NAME: &str = "erasing_op";

pub struct ErasingOperation;

impl Lint for ErasingOperation {
    fn allowed_name(self: &Self) -> &'static str {
        ERASING_OPERATION_LINT_NAME
    }

    fn diagnostic_message(self: &Self) -> &'static str {
        ERASING_OPERATION
    }

    fn kind(self: &Self) -> CairoLintKind {
        CairoLintKind::ErasingOperation
    }
}

pub fn check_erasing_operation(
    db: &dyn SemanticGroup,
    item: &ModuleItemId,
    diagnostics: &mut Vec<PluginDiagnostic>,
) {
    let function_bodies = get_all_function_bodies(db, item);
    for function_body in function_bodies.iter() {
        let function_call_exprs = get_all_function_calls(function_body);
        let arenas = &function_body.arenas;
        for function_call_expr in function_call_exprs.iter() {
            check_single_erasing_operation(db, function_call_expr, arenas, diagnostics);
        }
    }
}

fn check_single_erasing_operation(
    db: &dyn SemanticGroup,
    expr_func: &ExprFunctionCall,
    arenas: &Arenas,
    diagnostics: &mut Vec<PluginDiagnostic>,
) {
    let func = function_trait_name_from_fn_id(db, &expr_func.function);

    let is_erasing_operation = match func.as_str() {
        MUL | AND => is_zero(&expr_func.args[0], arenas) || is_zero(&expr_func.args[1], arenas),
        DIV => is_zero(&expr_func.args[0], arenas),
        _ => false,
    };
    if is_erasing_operation {
        diagnostics.push(PluginDiagnostic {
            stable_ptr: expr_func.stable_ptr.untyped(),
            message: ERASING_OPERATION.to_string(),
            severity: Severity::Warning,
        });
    }
}
fn is_zero(arg: &ExprFunctionCallArg, arenas: &Arenas) -> bool {
    match arg {
        ExprFunctionCallArg::Value(expr) => match &arenas.exprs[*expr] {
            Expr::Literal(val) => val.value == BigInt::ZERO,
            _ => false,
        },
        _ => false,
    }
}
