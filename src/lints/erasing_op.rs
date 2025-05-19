use cairo_lang_defs::ids::ModuleItemId;
use cairo_lang_defs::plugin::PluginDiagnostic;
use cairo_lang_diagnostics::Severity;
use cairo_lang_semantic::db::SemanticGroup;
use cairo_lang_semantic::{Arenas, ExprFunctionCall};
use cairo_lang_syntax::node::TypedStablePtr;

use super::{function_trait_name_from_fn_id, AND};
use crate::context::{CairoLintKind, Lint};
use crate::helper::is_zero;
use crate::lints::{DIV, MUL};
use crate::queries::{get_all_function_bodies, get_all_function_calls};

pub struct ErasingOperation;

/// ## What it does
///
/// Checks for operations that result in the value being erased (e.g., multiplication by 0 or 0 being divided by anything).
///
/// ## Example
///
/// ```cairo
/// fn main() {
///     let x = 1;
///     let _y = 0 * x;
///     let _z = 0 / x;
///     let _c = x & 0;
/// }
/// ```
///
/// Could be simplified by replacing the entire expression with 0:
///
/// ```cairo
/// fn main() {
///     let x = 1;
///     let _y = 0;
///     let _z = 0;
///     let _c = 0;
/// }
/// ```
impl Lint for ErasingOperation {
    fn allowed_name(&self) -> &'static str {
        "erasing_op"
    }

    fn diagnostic_message(&self) -> &'static str {
        "This operation results in the value being erased (e.g., multiplication by 0). \
                                     Consider replacing the entire expression with 0."
    }

    fn kind(&self) -> CairoLintKind {
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
        for function_call_expr in function_call_exprs {
            check_single_erasing_operation(db, &function_call_expr, arenas, diagnostics);
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
            message: ErasingOperation.diagnostic_message().to_string(),
            severity: Severity::Warning,
            relative_span: None,
            inner_span: None,
        });
    }
}
