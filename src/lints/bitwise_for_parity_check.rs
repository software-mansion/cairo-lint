use cairo_lang_defs::ids::{FunctionWithBodyId, ModuleItemId, TopLevelLanguageElementId};
use cairo_lang_defs::plugin::PluginDiagnostic;
use cairo_lang_diagnostics::Severity;
use cairo_lang_semantic::db::SemanticGroup;
use cairo_lang_semantic::{Arenas, Expr, ExprFunctionCall, ExprFunctionCallArg};
use cairo_lang_syntax::node::TypedStablePtr;
use if_chain::if_chain;
use num_bigint::BigInt;

use crate::context::{CairoLintKind, Lint};
use crate::queries::{get_all_function_bodies, get_all_function_calls};

use super::AND;

pub struct BitwiseForParity;

/// ## What it does
///
/// Checks for `x & 1` which is unoptimized in cairo and could be replaced by `x % 1`.
///
/// ## Example
///
/// ```cairo
/// fn main() {
///     let _a = 200_u32 & 1;
/// }
/// ```
impl Lint for BitwiseForParity {
    fn allowed_name(&self) -> &'static str {
        "bitwise_for_parity_check"
    }

    fn diagnostic_message(&self) -> &'static str {
        "You seem to be trying to use `&` for parity check. Consider using `DivRem::div_rem()` instead."
    }

    fn kind(&self) -> CairoLintKind {
        CairoLintKind::BitwiseForParityCheck
    }
}

/// Checks for `x & 1` which is unoptimized in cairo and can be replaced by `x % 1`
pub fn check_bitwise_for_parity(
    db: &dyn SemanticGroup,
    item: &ModuleItemId,
    diagnostics: &mut Vec<PluginDiagnostic>,
) {
    let function_bodies = get_all_function_bodies(db, item);
    for function_body in function_bodies.iter() {
        let function_call_exprs = get_all_function_calls(function_body);
        let arenas = &function_body.arenas;
        for function_call_expr in function_call_exprs {
            check_single_bitwise_for_parity(db, &function_call_expr, arenas, diagnostics);
        }
    }
}

fn check_single_bitwise_for_parity(
    db: &dyn SemanticGroup,
    function_call_expr: &ExprFunctionCall,
    arenas: &Arenas,
    diagnostics: &mut Vec<PluginDiagnostic>,
) {
    let Ok(Some(func_id)) = function_call_expr.function.get_concrete(db).body(db) else {
        return;
    };
    // Get the trait function id of the function (if there's none it means it cannot be a call to
    // `bitand`)
    let trait_fn_id = match func_id.function_with_body_id(db) {
        FunctionWithBodyId::Impl(func) => db.impl_function_trait_function(func).unwrap(),
        FunctionWithBodyId::Trait(func) => func,
        _ => return,
    };

    // From the trait function id get the trait name and check if it's the corelib `BitAnd`
    if_chain! {
        if trait_fn_id.full_path(db.upcast()) == AND;
        if let ExprFunctionCallArg::Value(val) = function_call_expr.args[1];
        // Checks if the rhs is 1
        if let Expr::Literal(lit) = &arenas.exprs[val];
        if lit.value == BigInt::from(1u8);
        then {
            diagnostics.push(PluginDiagnostic {
                stable_ptr: function_call_expr.stable_ptr.untyped(),
                message: BitwiseForParity.diagnostic_message().to_string(),
                severity: Severity::Warning,
                relative_span: None,
                inner_span: None,
            });
        }
    }
}
