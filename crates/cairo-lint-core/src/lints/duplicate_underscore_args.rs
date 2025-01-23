use std::collections::HashSet;

use cairo_lang_defs::{ids::ModuleItemId, plugin::PluginDiagnostic};
use cairo_lang_diagnostics::Severity;
use cairo_lang_semantic::db::SemanticGroup;
use cairo_lang_semantic::items::attribute::SemanticQueryAttrs;

use crate::queries::get_all_checkable_functions;

pub const DUPLICATE_UNDERSCORE_ARGS: &str = "duplicate arguments, having another argument having almost the same name \
                                             makes code comprehension and documentation more difficult";

pub const LINT_NAME: &str = "duplicate_underscore_args";

/// Checks for functions that have the same argument name but prefix with `_`. For example
/// `fn foo(a, _a)`
pub fn check_duplicate_underscore_args(
    db: &dyn SemanticGroup,
    item: &ModuleItemId,
    diagnostics: &mut Vec<PluginDiagnostic>,
) {
    let functions = get_all_checkable_functions(db, item);

    for function in functions {
        if let Ok(true) = function.has_attr_with_arg(db, "allow", LINT_NAME) {
            continue;
        }

        let mut registered_names: HashSet<String> = HashSet::new();
        let params = db.function_with_body_signature(function).unwrap().params;

        for param in params {
            let param_name = param.name.to_string();
            let stripped_name = param_name.strip_prefix('_').unwrap_or(&param_name);

            if !registered_names.insert(stripped_name.to_string()) {
                diagnostics.push(PluginDiagnostic {
                    stable_ptr: param.stable_ptr.0,
                    message: DUPLICATE_UNDERSCORE_ARGS.to_string(),
                    severity: Severity::Warning,
                });
            }
        }
    }
}
