use std::collections::HashSet;

use cairo_lang_defs::{ids::ModuleItemId, plugin::PluginDiagnostic};
use cairo_lang_diagnostics::Severity;
use cairo_lang_semantic::db::SemanticGroup;

use crate::context::{CairoLintKind, Lint};
use crate::queries::get_all_checkable_functions;

pub struct DuplicateUnderscoreArgs;

/// ## What it does
///
/// Checks for functions that have the same argument name but prefix with `_`.
///
/// ## Example
///
/// This code will raise a warning because it can be difficult to differentiate between `test` and `_test`.
///
/// ```cairo
/// fn foo(test: u32, _test: u32) {}
/// ```
impl Lint for DuplicateUnderscoreArgs {
    fn allowed_name(&self) -> &'static str {
        "duplicate_underscore_args"
    }

    fn diagnostic_message(&self) -> &'static str {
        "duplicate arguments, having another argument having almost the same name \
                                             makes code comprehension and documentation more difficult"
    }

    fn kind(&self) -> CairoLintKind {
        CairoLintKind::DuplicateUnderscoreArgs
    }
}

pub fn check_duplicate_underscore_args(
    db: &dyn SemanticGroup,
    item: &ModuleItemId,
    diagnostics: &mut Vec<PluginDiagnostic>,
) {
    let functions = get_all_checkable_functions(db, item);

    for function in functions {
        let mut registered_names: HashSet<String> = HashSet::new();
        let params = db.function_with_body_signature(function).unwrap().params;

        for param in params {
            let param_name = param.name.to_string();
            let stripped_name = param_name.strip_prefix('_').unwrap_or(&param_name);

            if !registered_names.insert(stripped_name.to_string()) {
                diagnostics.push(PluginDiagnostic {
                    stable_ptr: param.stable_ptr.0,
                    message: DuplicateUnderscoreArgs.diagnostic_message().to_string(),
                    severity: Severity::Warning,
                    relative_span: None,
                    inner_span: None,
                });
            }
        }
    }
}
