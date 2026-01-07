use std::collections::HashSet;

use crate::context::{CairoLintKind, Lint};
use cairo_lang_defs::{ids::ModuleItemId, plugin::PluginDiagnostic};
use cairo_lang_diagnostics::Severity;
use cairo_lang_semantic::items::function_with_body::FunctionWithBodySemantic;

use crate::queries::get_all_checkable_functions;
use salsa::Database;

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

#[tracing::instrument(skip_all, level = "trace")]
pub fn check_duplicate_underscore_args<'db>(
    db: &'db dyn Database,
    item: &ModuleItemId<'db>,
    diagnostics: &mut Vec<PluginDiagnostic<'db>>,
) {
    let functions = get_all_checkable_functions(db, item);

    for function in functions {
        let mut registered_names: HashSet<String> = HashSet::new();
        let params = db
            .function_with_body_signature(function)
            .cloned()
            .unwrap()
            .params;

        for param in params {
            let name_string = param.name.to_string(db);
            let stripped_name = name_string.strip_prefix('_').unwrap_or(&name_string);

            if !registered_names.insert(stripped_name.to_string()) {
                diagnostics.push(PluginDiagnostic {
                    stable_ptr: param.stable_ptr.0,
                    message: DuplicateUnderscoreArgs.diagnostic_message().to_string(),
                    severity: Severity::Warning,
                    inner_span: None,
                    error_code: None,
                });
            }
        }
    }
}
