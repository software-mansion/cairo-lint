use cairo_lang_defs::ids::{FunctionWithBodyId, ModuleItemId, TopLevelLanguageElementId};
use cairo_lang_defs::plugin::PluginDiagnostic;
use cairo_lang_diagnostics::Severity;
use cairo_lang_semantic::db::SemanticGroup;
use cairo_lang_semantic::{Arenas, Expr, ExprFunctionCall, ExprFunctionCallArg};
use cairo_lang_syntax::node::helpers::QueryAttrs;
use cairo_lang_syntax::node::{TypedStablePtr, TypedSyntaxNode};
use if_chain::if_chain;
use num_bigint::BigInt;

use crate::queries::{get_all_function_bodies, get_all_function_calls};

use super::AND;

pub const BITWISE_FOR_PARITY: &str =
    "You seem to be trying to use `&` for parity check. Consider using `DivRem::div_rem()` instead.";

pub const LINT_NAME: &str = "bitwise_for_parity_check";

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
        for function_call_expr in function_call_exprs.iter() {
            check_single_bitwise_for_parity(db, function_call_expr, arenas, diagnostics);
        }
    }
}

fn check_single_bitwise_for_parity(
    db: &dyn SemanticGroup,
    function_call_expr: &ExprFunctionCall,
    arenas: &Arenas,
    diagnostics: &mut Vec<PluginDiagnostic>,
) {
    // Checks if the lint is allowed in any upper scope
    let mut current_node = function_call_expr.stable_ptr.lookup(db.upcast()).as_syntax_node();
    while let Some(node) = current_node.parent() {
        if node.has_attr_with_arg(db.upcast(), "allow", LINT_NAME) {
            return;
        }
        current_node = node;
    }
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
                message: BITWISE_FOR_PARITY.to_string(),
                severity: Severity::Warning,
            });
        }
    }
}
