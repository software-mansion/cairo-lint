use cairo_lang_defs::ids::ModuleItemId;
use cairo_lang_defs::plugin::PluginDiagnostic;
use cairo_lang_diagnostics::Severity;
use cairo_lang_semantic::db::SemanticGroup;
use cairo_lang_semantic::{Arenas, ExprIf, ExprMatch};
use cairo_lang_syntax::node::TypedStablePtr;

use crate::lints::manual::{check_manual, check_manual_if, ManualLint};
use crate::queries::{get_all_function_bodies, get_all_if_expressions, get_all_match_expressions};

pub const MANUAL_OK_OR: &str = "Manual match for Option<T> detected. Consider using ok_or instead";
pub const LINT_NAME: &str = "manual_ok_or";

pub fn check_manual_ok_or(
    db: &dyn SemanticGroup,
    item: &ModuleItemId,
    diagnostics: &mut Vec<PluginDiagnostic>,
) {
    let function_bodies = get_all_function_bodies(db, item);
    for function_body in function_bodies.iter() {
        let match_exprs = get_all_match_expressions(function_body);
        let arenas = &function_body.arenas;
        for match_expr in match_exprs.iter() {
            if check_manual(db, match_expr, arenas, ManualLint::ManualOkOr, LINT_NAME) {
                diagnostics.push(PluginDiagnostic {
                    stable_ptr: match_expr.stable_ptr.untyped(),
                    message: MANUAL_OK_OR.to_owned(),
                    severity: Severity::Warning,
                });
            }
        }
    }
}

pub fn check_manual_if_ok_or(
    db: &dyn SemanticGroup,
    item: &ModuleItemId,
    diagnostics: &mut Vec<PluginDiagnostic>,
) {
    let function_bodies = get_all_function_bodies(db, item);
    for function_body in function_bodies.iter() {
        let if_exprs = get_all_if_expressions(function_body);
        let arenas = &function_body.arenas;
        for if_expr in if_exprs.iter() {
            if check_manual_if(db, if_expr, arenas, ManualLint::ManualOkOr, LINT_NAME) {
                diagnostics.push(PluginDiagnostic {
                    stable_ptr: if_expr.stable_ptr.untyped(),
                    message: MANUAL_OK_OR.to_owned(),
                    severity: Severity::Warning,
                });
            }
        }
    }
}
