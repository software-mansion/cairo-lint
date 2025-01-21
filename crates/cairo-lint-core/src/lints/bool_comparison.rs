use cairo_lang_defs::ids::{ModuleItemId, TopLevelLanguageElementId};
use cairo_lang_defs::plugin::PluginDiagnostic;
use cairo_lang_diagnostics::Severity;
use cairo_lang_semantic::db::SemanticGroup;
use cairo_lang_semantic::{Arenas, Expr, ExprFunctionCall, ExprFunctionCallArg};
use cairo_lang_syntax::node::helpers::QueryAttrs;
use cairo_lang_syntax::node::{TypedStablePtr, TypedSyntaxNode};
use if_chain::if_chain;

use crate::queries::{get_all_function_bodies, get_all_function_calls};

pub const BOOL_COMPARISON: &str =
    "Unnecessary comparison with a boolean value. Use the variable directly.";

pub const ALLOWED: [&str; 1] = [LINT_NAME];
pub const LINT_NAME: &str = "bool_comparison";

/// Checks for ` a == true`. Bool comparisons are useless and can be rewritten more clearly.
pub fn check_bool_comparison(
    db: &dyn SemanticGroup,
    item: &ModuleItemId,
    diagnostics: &mut Vec<PluginDiagnostic>,
) {
    let function_bodies = get_all_function_bodies(db, item);
    for function_body in function_bodies.iter() {
        let function_call_exprs = get_all_function_calls(function_body);
        let arenas = &function_body.arenas;
        for function_call_expr in function_call_exprs.iter() {
          check_single_bool_comparison(db, function_call_expr, arenas, diagnostics);
        }
    }
}

fn check_single_bool_comparison(
    db: &dyn SemanticGroup,
    function_call_expr: &ExprFunctionCall,
    arenas: &Arenas,
    diagnostics: &mut Vec<PluginDiagnostic>,
) {
    // Checks if the lint is allowed in an upper scope
    let mut current_node = function_call_expr.stable_ptr.lookup(db.upcast()).as_syntax_node();
    while let Some(node) = current_node.parent() {
        if node.has_attr_with_arg(db.upcast(), "allow", LINT_NAME) {
            return;
        }
        current_node = node;
    }
    // Check if the function call is the bool partial eq function (==).
    if !function_call_expr
        .function
        .full_path(db)
        .contains("core::BoolPartialEq::")
    {
        return;
    }
    // Extract the args of the function call. This function expects snapshots hence we need to
    // destructure that. Also the boolean type in cairo is an enum hence the enum ctor.
    for arg in &function_call_expr.args {
        if_chain! {
            if let ExprFunctionCallArg::Value(expr) = arg;
            if let Expr::Snapshot(snap) = &arenas.exprs[*expr];
            if let Expr::EnumVariantCtor(enum_var) = &arenas.exprs[snap.inner];
            if enum_var.variant.concrete_enum_id.enum_id(db).full_path(db.upcast()) == "core::bool";
            then {
                diagnostics.push(PluginDiagnostic {
                    stable_ptr: function_call_expr.stable_ptr.untyped(),
                    message: BOOL_COMPARISON.to_string(),
                    severity: Severity::Warning,
                });
            }
        }
    }
}
