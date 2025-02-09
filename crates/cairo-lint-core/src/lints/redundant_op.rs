use crate::context::{CairoLintKind, Lint};
use crate::lints::function_trait_name_from_fn_id;
use crate::queries::{get_all_function_bodies, get_all_function_calls};
use cairo_lang_defs::ids::ModuleItemId;
use cairo_lang_defs::plugin::PluginDiagnostic;
use cairo_lang_diagnostics::Severity;
use cairo_lang_semantic::db::SemanticGroup;
use cairo_lang_semantic::{Arenas, Expr, ExprFunctionCall, ExprFunctionCallArg};
use cairo_lang_syntax::node::TypedStablePtr;
use num_bigint::BigInt;

pub struct RedundantOperation;

impl Lint for RedundantOperation {
    fn allowed_name(&self) -> &'static str {
        "redundant_op"
    }

    fn diagnostic_message(&self) -> &'static str {
        "This operation doesn't change the value and can be simplified."
    }

    fn kind(&self) -> crate::context::CairoLintKind {
        CairoLintKind::RedundantOperation
    }
}

fn is_zero(arg: &ExprFunctionCallArg, arenas: &Arenas) -> bool {
    matches!(
        arg,
        ExprFunctionCallArg::Value(expr) if matches!(&arenas.exprs[*expr], Expr::Literal(val) if val.value == BigInt::from(0))
    )
}

fn is_one(arg: &ExprFunctionCallArg, arenas: &Arenas) -> bool {
    matches!(
        arg,
        ExprFunctionCallArg::Value(expr) if matches!(&arenas.exprs[*expr], Expr::Literal(val) if val.value == BigInt::from(1))
    )
}

pub fn check_redundant_operation(
    db: &dyn SemanticGroup,
    item: &ModuleItemId,
    diagnostics: &mut Vec<PluginDiagnostic>,
) {
    let function_bodies = get_all_function_bodies(db, item);
    for function_body in function_bodies.iter() {
        let function_call_exprs = get_all_function_calls(function_body);
        let arenas = &function_body.arenas;
        for function_call_expr in function_call_exprs.iter() {
            check_single_redundant_operation(db, function_call_expr, arenas, diagnostics);
        }
    }
}

fn check_single_redundant_operation(
    db: &dyn SemanticGroup,
    expr_func: &ExprFunctionCall,
    arenas: &Arenas,
    diagnostics: &mut Vec<PluginDiagnostic>,
) {
    let func = function_trait_name_from_fn_id(db, &expr_func.function);
    let op = func.split("::").last().unwrap_or("");

    let is_redundant = match op {
        "add" => is_zero(&expr_func.args[0], arenas) || is_zero(&expr_func.args[1], arenas),
        "sub" => is_zero(&expr_func.args[1], arenas),
        "mul" => is_one(&expr_func.args[0], arenas) || is_one(&expr_func.args[1], arenas),
        "div" => is_one(&expr_func.args[1], arenas),
        _ => false,
    };

    if is_redundant {
        diagnostics.push(PluginDiagnostic {
            stable_ptr: expr_func.stable_ptr.untyped(),
            message: RedundantOperation.diagnostic_message().to_string(),
            severity: Severity::Warning,
        });
    }
}
