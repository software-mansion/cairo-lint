use cairo_lang_defs::diagnostic_utils::StableLocation;
use cairo_lang_defs::ids::ModuleItemId;
use cairo_lang_defs::plugin::PluginDiagnostic;
use cairo_lang_diagnostics::Severity;
use cairo_lang_filesystem::db::get_originating_location;
use cairo_lang_semantic::db::SemanticGroup;
use cairo_lang_semantic::ExprFunctionCall;
use cairo_lang_syntax::node::helpers::QueryAttrs;
use cairo_lang_syntax::node::{TypedStablePtr, TypedSyntaxNode};
use if_chain::if_chain;

use crate::queries::{get_all_function_bodies, get_all_function_calls};

pub const PANIC_IN_CODE: &str = "Leaving `panic` in the code is discouraged.";
const PANIC: &str = "core::panics::panic";
pub const LINT_NAME: &str = "panic";

/// Checks for panic usage.
pub fn check_panic_usage(
    db: &dyn SemanticGroup,
    item: &ModuleItemId,
    diagnostics: &mut Vec<PluginDiagnostic>,
) {
    let function_bodies = get_all_function_bodies(db, item);
    for function_body in function_bodies.iter() {
        let function_call_exprs = get_all_function_calls(function_body);
        for function_call_expr in function_call_exprs.iter() {
            check_single_panic_usage(db, function_call_expr, diagnostics);
        }
    }
}

fn check_single_panic_usage(
    db: &dyn SemanticGroup,
    function_call_expr: &ExprFunctionCall,
    diagnostics: &mut Vec<PluginDiagnostic>,
) {
    // Checks if the lint is allowed in an upper scope
    let mut current_node = function_call_expr
        .stable_ptr
        .lookup(db.upcast())
        .as_syntax_node();
    let init_node = current_node.clone();
    while let Some(node) = current_node.parent() {
        if node.has_attr_with_arg(db.upcast(), "allow", LINT_NAME) {
            return;
        }
        current_node = node;
    }

    // If the function is not the panic function from the corelib return
    if function_call_expr.function.full_path(db) != PANIC {
        return;
    }

    // Get the origination location of this panic as there is a `panic!` macro that gerates virtual
    // files
    let initial_file_id =
        StableLocation::new(function_call_expr.stable_ptr.untyped()).file_id(db.upcast());
    let (file_id, span) = get_originating_location(
        db.upcast(),
        initial_file_id,
        function_call_expr
            .stable_ptr
            .lookup(db.upcast())
            .as_syntax_node()
            .span(db.upcast()),
        None,
    );
    // If the panic comes from a real file (macros generate code in new virtual files)
    if initial_file_id == file_id {
        diagnostics.push(PluginDiagnostic {
            stable_ptr: init_node.stable_ptr(),
            message: PANIC_IN_CODE.to_owned(),
            severity: Severity::Warning,
        });
    } else {
        // If the originating location is a different file get the syntax node that generated the
        // code that contains a panic.
        if_chain! {
            if let Some(text_position) = span.position_in_file(db.upcast(), file_id);
            if let Ok(file_node) = db.file_syntax(file_id);
            then {
                let syntax_node = file_node.lookup_position(db.upcast(), text_position.start);
                // Checks if the lint is allowed in the original file
                let mut current_node = syntax_node.clone();
                while let Some(node) = current_node.parent() {
                    if node.has_attr_with_arg(db.upcast(), "allow", LINT_NAME) {
                        return;
                    }
                    current_node = node;
                }
                diagnostics.push(PluginDiagnostic {
                    stable_ptr: syntax_node.stable_ptr(),
                    message: PANIC_IN_CODE.to_owned(),
                    severity: Severity::Warning,
                });
            }
        }
    }
}
