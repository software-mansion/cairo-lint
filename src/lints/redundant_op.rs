use super::{ADD, DIV, MUL, SUB};
use crate::context::{CairoLintKind, Lint};
use crate::helper::{is_one, is_zero};
use crate::lints::function_trait_name_from_fn_id;
use crate::queries::{get_all_function_bodies, get_all_function_calls};
use cairo_lang_defs::ids::ModuleItemId;
use cairo_lang_defs::plugin::PluginDiagnostic;
use cairo_lang_diagnostics::Severity;
use cairo_lang_semantic::db::SemanticGroup;
use cairo_lang_semantic::{Arenas, ExprFunctionCall};
use cairo_lang_syntax::node::TypedStablePtr;

pub struct RedundantOperation;

/// ## What it does
///
/// Checks for redundant arithmetic operations like `x + 0`, `x - 0`, `x * 1`, `x / 1`
///
/// ## Example
///
/// ```cairo
/// fn main() {
///     let x = 42;
///     let _y = x * 1;
/// }
/// ```
///
/// Can be simplified to
///
/// ```cairo
/// fn main() {
///     let x = 42;
///     let _y = x;
/// }
/// ```
impl Lint for RedundantOperation {
    fn allowed_name(&self) -> &'static str {
        "redundant_op"
    }

    fn diagnostic_message(&self) -> &'static str {
        "This operation doesn't change the value and can be simplified."
    }

    fn kind(&self) -> CairoLintKind {
        CairoLintKind::RedundantOperation
    }
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
        for function_call_expr in function_call_exprs {
            check_single_redundant_operation(db, &function_call_expr, arenas, diagnostics);
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
    let is_redundant = match func.as_str() {
        ADD => is_zero(&expr_func.args[0], arenas) || is_zero(&expr_func.args[1], arenas),
        SUB => is_zero(&expr_func.args[1], arenas),
        MUL => is_one(&expr_func.args[0], arenas) || is_one(&expr_func.args[1], arenas),
        DIV => is_one(&expr_func.args[1], arenas),
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
