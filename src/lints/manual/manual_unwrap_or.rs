use cairo_lang_defs::ids::ModuleItemId;
use cairo_lang_defs::plugin::PluginDiagnostic;
use cairo_lang_diagnostics::Severity;
use cairo_lang_semantic::db::SemanticGroup;
use cairo_lang_syntax::node::TypedStablePtr;

use crate::{
    context::CairoLintKind,
    queries::{get_all_function_bodies, get_all_if_expressions, get_all_match_expressions},
};
use crate::{
    context::Lint,
    lints::manual::{check_manual, check_manual_if, ManualLint},
};

pub struct ManualUnwrapOr;

/// ## What it does
///
/// Finds patterns that reimplement `Option::unwrap_or` or `Result::unwrap_or`.
///
/// ## Example
///
/// ```cairo
/// let foo: Option<i32> = None;
/// match foo {
///     Some(v) => v,
///     None => 1,
/// };
/// ```
///
/// Can be simplified to:
///
/// ```cairo
/// let foo: Option<i32> = None;
/// foo.unwrap_or(1);
/// ```
impl Lint for ManualUnwrapOr {
    fn allowed_name(&self) -> &'static str {
        "manual_unwrap_or"
    }

    fn diagnostic_message(&self) -> &'static str {
        "Manual `unwrap_or` detected. Consider using `unwrap_or()` instead.`"
    }

    fn kind(&self) -> CairoLintKind {
        CairoLintKind::ManualUnwrapOr
    }
}

pub fn check_manual_unwrap_or(
    db: &dyn SemanticGroup,
    item: &ModuleItemId,
    diagnostics: &mut Vec<PluginDiagnostic>,
) {
    let function_bodies = get_all_function_bodies(db, item);
    for function_body in function_bodies {
        let if_exprs = get_all_if_expressions(&function_body);
        let match_exprs = get_all_match_expressions(&function_body);
        let arenas = &function_body.arenas;
        for match_expr in match_exprs {
            if check_manual(db, &match_expr, arenas, ManualLint::ManualUnwrapOr) {
                diagnostics.push(PluginDiagnostic {
                    stable_ptr: match_expr.stable_ptr.untyped(),
                    message: ManualUnwrapOr.diagnostic_message().to_owned(),
                    severity: Severity::Warning,
                    relative_span: None,
                });
            }
        }
        for if_expr in if_exprs {
            if check_manual_if(db, &if_expr, arenas, ManualLint::ManualUnwrapOr) {
                diagnostics.push(PluginDiagnostic {
                    stable_ptr: if_expr.stable_ptr.untyped(),
                    message: ManualUnwrapOr.diagnostic_message().to_owned(),
                    severity: Severity::Warning,
                    relative_span: None,
                });
            }
        }
    }
}
