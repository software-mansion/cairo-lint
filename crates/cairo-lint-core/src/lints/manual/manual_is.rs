use cairo_lang_defs::ids::ModuleItemId;
use cairo_lang_defs::plugin::PluginDiagnostic;
use cairo_lang_diagnostics::Severity;
use cairo_lang_semantic::db::SemanticGroup;
use cairo_lang_semantic::{Arenas, ExprIf, ExprMatch};
use cairo_lang_syntax::node::TypedStablePtr;

use crate::lints::manual::{check_manual, check_manual_if, ManualLint};
use crate::queries::{get_all_function_bodies, get_all_if_expressions, get_all_match_expressions};

pub const MANUAL_IS_SOME: &str =
    "Manual match for `is_some` detected. Consider using `is_some()` instead";
pub(crate) mod some {
    pub(crate) const LINT_NAME: &str = "manual_is_some";
}
pub(crate) mod none {
    pub(crate) const LINT_NAME: &str = "manual_is_none";
}
pub(crate) mod ok {
    pub(crate) const LINT_NAME: &str = "manual_is_ok";
}
pub(crate) mod err {
    pub(crate) const LINT_NAME: &str = "manual_is_err";
}

pub const MANUAL_IS_NONE: &str =
    "Manual match for `is_none` detected. Consider using `is_none()` instead";
pub const MANUAL_IS_OK: &str =
    "Manual match for `is_ok` detected. Consider using `is_ok()` instead";
pub const MANUAL_IS_ERR: &str =
    "Manual match for `is_err` detected. Consider using `is_err()` instead";

pub fn check_manual_is(
    db: &dyn SemanticGroup,
    item: &ModuleItemId,
    diagnostics: &mut Vec<PluginDiagnostic>,
) {
    let function_bodies = get_all_function_bodies(db, item);
    for function_body in function_bodies.iter() {
        let match_exprs = get_all_match_expressions(function_body);
        let arenas = &function_body.arenas;
        for match_expr in match_exprs.iter() {
            if check_manual(
                db,
                match_expr,
                arenas,
                ManualLint::ManualIsSome,
                some::LINT_NAME,
            ) {
                diagnostics.push(PluginDiagnostic {
                    stable_ptr: match_expr.stable_ptr.untyped(),
                    message: MANUAL_IS_SOME.to_owned(),
                    severity: Severity::Warning,
                });
            }
            if check_manual(
                db,
                match_expr,
                arenas,
                ManualLint::ManualIsNone,
                none::LINT_NAME,
            ) {
                diagnostics.push(PluginDiagnostic {
                    stable_ptr: match_expr.stable_ptr.untyped(),
                    message: MANUAL_IS_NONE.to_owned(),
                    severity: Severity::Warning,
                });
            }
            if check_manual(
                db,
                match_expr,
                arenas,
                ManualLint::ManualIsOk,
                ok::LINT_NAME,
            ) {
                diagnostics.push(PluginDiagnostic {
                    stable_ptr: match_expr.stable_ptr.untyped(),
                    message: MANUAL_IS_OK.to_owned(),
                    severity: Severity::Warning,
                });
            }
            if check_manual(
                db,
                match_expr,
                arenas,
                ManualLint::ManualIsErr,
                err::LINT_NAME,
            ) {
                diagnostics.push(PluginDiagnostic {
                    stable_ptr: match_expr.stable_ptr.untyped(),
                    message: MANUAL_IS_ERR.to_owned(),
                    severity: Severity::Warning,
                });
            }
        }
    }
}

pub fn check_manual_if_is(
    db: &dyn SemanticGroup,
    item: &ModuleItemId,
    diagnostics: &mut Vec<PluginDiagnostic>,
) {
    let function_bodies = get_all_function_bodies(db, item);
    for function_body in function_bodies.iter() {
        let if_exprs = get_all_if_expressions(function_body);
        let arenas = &function_body.arenas;
        for if_expr in if_exprs.iter() {
            if check_manual_if(
                db,
                if_expr,
                arenas,
                ManualLint::ManualIsSome,
                some::LINT_NAME,
            ) {
                diagnostics.push(PluginDiagnostic {
                    stable_ptr: if_expr.stable_ptr.untyped(),
                    message: MANUAL_IS_SOME.to_owned(),
                    severity: Severity::Warning,
                });
            }
            if check_manual_if(
                db,
                if_expr,
                arenas,
                ManualLint::ManualIsNone,
                none::LINT_NAME,
            ) {
                diagnostics.push(PluginDiagnostic {
                    stable_ptr: if_expr.stable_ptr.untyped(),
                    message: MANUAL_IS_NONE.to_owned(),
                    severity: Severity::Warning,
                });
            }
            if check_manual_if(db, if_expr, arenas, ManualLint::ManualIsOk, ok::LINT_NAME) {
                diagnostics.push(PluginDiagnostic {
                    stable_ptr: if_expr.stable_ptr.untyped(),
                    message: MANUAL_IS_OK.to_owned(),
                    severity: Severity::Warning,
                });
            }
            if check_manual_if(db, if_expr, arenas, ManualLint::ManualIsErr, err::LINT_NAME) {
                diagnostics.push(PluginDiagnostic {
                    stable_ptr: if_expr.stable_ptr.untyped(),
                    message: MANUAL_IS_ERR.to_owned(),
                    severity: Severity::Warning,
                });
            }
        }
    }
}
