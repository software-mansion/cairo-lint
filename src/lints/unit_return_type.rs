use cairo_lang_defs::{ids::ModuleItemId, plugin::PluginDiagnostic};
use cairo_lang_diagnostics::Severity;
use cairo_lang_semantic::db::SemanticGroup;
use cairo_lang_syntax::node::{ast::OptionReturnTypeClause, TypedStablePtr};

use crate::{
    context::{CairoLintKind, Lint},
    queries::get_all_checkable_functions,
};

pub struct UnitReturnType;

/// ## What it does
///
/// Detects if the function has a unit return type, which is not needed to be specified.
///
/// ## Example
///
/// ```cairo
/// fn foo() -> () {
///     println!("Hello, world!");
/// }
/// ```
///
/// Can be simplified to just:
///
/// ```cairo
/// fn foo() {
///     println!("Hello, world!");
/// }
/// ```
impl Lint for UnitReturnType {
    fn allowed_name(&self) -> &'static str {
        "unit_return_type"
    }

    fn diagnostic_message(&self) -> &'static str {
        "unnecessary declared unit return type `()`"
    }

    fn kind(&self) -> CairoLintKind {
        CairoLintKind::UnitReturnType
    }
}

pub fn check_unit_return_type(
    db: &dyn SemanticGroup,
    item: &ModuleItemId,
    diagnostics: &mut Vec<PluginDiagnostic>,
) {
    let functions = get_all_checkable_functions(db, item);
    for function in functions {
        let function_signature = db.function_with_body_signature(function).unwrap();
        let return_type = function_signature.return_type;

        // Check if the return type is unit.
        if return_type.is_unit(db) {
            let ast_function_signature = function_signature.stable_ptr.lookup(db);

            // Checks if the return type is explicitly declared as `()`.
            if let OptionReturnTypeClause::ReturnTypeClause(_) = ast_function_signature.ret_ty(db) {
                diagnostics.push(PluginDiagnostic {
                    stable_ptr: function_signature.stable_ptr.untyped(),
                    message: UnitReturnType.diagnostic_message().to_string(),
                    severity: Severity::Warning,
                    relative_span: None,
                });
            }
        }
    }
}
