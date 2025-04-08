use cairo_lang_defs::ids::ModuleItemId;
use cairo_lang_defs::plugin::PluginDiagnostic;
use cairo_lang_diagnostics::Severity;
use cairo_lang_semantic::db::SemanticGroup;
use cairo_lang_syntax::node::db::SyntaxGroup;
use cairo_lang_syntax::node::{SyntaxNode, TypedStablePtr};

use crate::context::{CairoLintKind, Lint};
use crate::lints::manual::{check_manual, check_manual_if, ManualLint};
use crate::queries::{get_all_function_bodies, get_all_if_expressions, get_all_match_expressions};

use super::helpers::fix_manual;

pub struct ManualIsSome;

/// ## What it does
///
/// Checks for manual implementations of `is_some`.
///
/// ## Example
///
/// ```cairo
/// fn main() {
///     let foo: Option<i32> = Option::None;
///     let _foo = match foo {
///         Option::Some(_) => true,
///         Option::None => false,
///     };
/// }
/// ```
///
/// Can be rewritten as:
///
/// ```cairo
/// fn main() {
///     let foo: Option<i32> = Option::None;
///     let _foo = foo.is_some();
/// }
/// ```
impl Lint for ManualIsSome {
    fn allowed_name(&self) -> &'static str {
        "manual_is_some"
    }

    fn diagnostic_message(&self) -> &'static str {
        "Manual match for `is_some` detected. Consider using `is_some()` instead"
    }

    fn kind(&self) -> CairoLintKind {
        CairoLintKind::ManualIsSome
    }

    fn has_fixer(&self) -> bool {
        true
    }

    fn fix(&self, db: &dyn SemanticGroup, node: SyntaxNode) -> Option<(SyntaxNode, String)> {
        fix_manual_is_some(db.upcast(), node)
    }
}

pub struct ManualIsNone;

/// ## What it does
///
/// Checks for manual implementations of `is_none`.
///
/// ## Example
///
/// ```cairo
/// fn main() {
///     let foo: Option<i32> = Option::None;
///     let _foo = match foo {
///         Option::Some(_) => false,
///         Option::None => true,
///     };
/// }
/// ```
///
/// Can be rewritten as:
///
/// ```cairo
/// fn main() {
///     let foo: Option<i32> = Option::None;
///     let _foo = foo.is_none();
/// }
/// ```
impl Lint for ManualIsNone {
    fn allowed_name(&self) -> &'static str {
        "manual_is_none"
    }

    fn diagnostic_message(&self) -> &'static str {
        "Manual match for `is_none` detected. Consider using `is_none()` instead"
    }

    fn kind(&self) -> CairoLintKind {
        CairoLintKind::ManualIsNone
    }

    fn has_fixer(&self) -> bool {
        true
    }

    fn fix(&self, db: &dyn SemanticGroup, node: SyntaxNode) -> Option<(SyntaxNode, String)> {
        fix_manual_is_none(db.upcast(), node)
    }
}

pub struct ManualIsOk;

/// ## What it does
///
/// Checks for manual implementations of `is_ok`.
///
/// ## Example
///
/// ```cairo
/// fn main() {
///     let res_val: Result<i32> = Result::Err('err');
///     let _a = match res_val {
///         Result::Ok(_) => true,
///         Result::Err(_) => false
///     };
/// }
/// ```
///
/// Can be rewritten as:
///
/// ```cairo
/// fn main() {
///     let res_val: Result<i32> = Result::Err('err');
///     let _a = res_val.is_ok();
/// }
/// ```
impl Lint for ManualIsOk {
    fn allowed_name(&self) -> &'static str {
        "manual_is_ok"
    }

    fn diagnostic_message(&self) -> &'static str {
        "Manual match for `is_ok` detected. Consider using `is_ok()` instead"
    }

    fn kind(&self) -> CairoLintKind {
        CairoLintKind::ManualIsOk
    }

    fn has_fixer(&self) -> bool {
        true
    }

    fn fix(&self, db: &dyn SemanticGroup, node: SyntaxNode) -> Option<(SyntaxNode, String)> {
        fix_manual_is_ok(db.upcast(), node)
    }
}

pub struct ManualIsErr;

/// ## What it does
///
/// Checks for manual implementations of `is_err`.
///
/// ## Example
///
/// ```cairo
/// fn main() {
///     let res_val: Result<i32> = Result::Err('err');
///     let _a = match res_val {
///         Result::Ok(_) => false,
///         Result::Err(_) => true
///     };
/// }
/// ```
///
/// Can be rewritten as:
///
/// ```cairo
/// fn main() {
///     let res_val: Result<i32> = Result::Err('err');
///     let _a = res_val.is_err();
/// }
/// ```
impl Lint for ManualIsErr {
    fn allowed_name(&self) -> &'static str {
        "manual_is_err"
    }

    fn diagnostic_message(&self) -> &'static str {
        "Manual match for `is_err` detected. Consider using `is_err()` instead"
    }

    fn kind(&self) -> CairoLintKind {
        CairoLintKind::ManualIsErr
    }

    fn has_fixer(&self) -> bool {
        true
    }

    fn fix(&self, db: &dyn SemanticGroup, node: SyntaxNode) -> Option<(SyntaxNode, String)> {
        fix_manual_is_err(db.upcast(), node)
    }
}

pub fn check_manual_is(
    db: &dyn SemanticGroup,
    item: &ModuleItemId,
    diagnostics: &mut Vec<PluginDiagnostic>,
) {
    let function_bodies = get_all_function_bodies(db, item);
    for function_body in function_bodies.iter() {
        let if_exprs = get_all_if_expressions(function_body);
        let match_exprs = get_all_match_expressions(function_body);
        let arenas = &function_body.arenas;
        for match_expr in match_exprs.iter() {
            if check_manual(db, match_expr, arenas, ManualLint::ManualIsSome) {
                diagnostics.push(PluginDiagnostic {
                    stable_ptr: match_expr.stable_ptr.untyped(),
                    message: ManualIsSome.diagnostic_message().to_owned(),
                    severity: Severity::Warning,
                });
            }
            if check_manual(db, match_expr, arenas, ManualLint::ManualIsNone) {
                diagnostics.push(PluginDiagnostic {
                    stable_ptr: match_expr.stable_ptr.untyped(),
                    message: ManualIsNone.diagnostic_message().to_owned(),
                    severity: Severity::Warning,
                });
            }
            if check_manual(db, match_expr, arenas, ManualLint::ManualIsOk) {
                diagnostics.push(PluginDiagnostic {
                    stable_ptr: match_expr.stable_ptr.untyped(),
                    message: ManualIsOk.diagnostic_message().to_owned(),
                    severity: Severity::Warning,
                });
            }
            if check_manual(db, match_expr, arenas, ManualLint::ManualIsErr) {
                diagnostics.push(PluginDiagnostic {
                    stable_ptr: match_expr.stable_ptr.untyped(),
                    message: ManualIsErr.diagnostic_message().to_owned(),
                    severity: Severity::Warning,
                });
            }
        }
        for if_expr in if_exprs.iter() {
            if check_manual_if(db, if_expr, arenas, ManualLint::ManualIsSome) {
                diagnostics.push(PluginDiagnostic {
                    stable_ptr: if_expr.stable_ptr.untyped(),
                    message: ManualIsSome.diagnostic_message().to_owned(),
                    severity: Severity::Warning,
                });
            }
            if check_manual_if(db, if_expr, arenas, ManualLint::ManualIsNone) {
                diagnostics.push(PluginDiagnostic {
                    stable_ptr: if_expr.stable_ptr.untyped(),
                    message: ManualIsNone.diagnostic_message().to_owned(),
                    severity: Severity::Warning,
                });
            }
            if check_manual_if(db, if_expr, arenas, ManualLint::ManualIsOk) {
                diagnostics.push(PluginDiagnostic {
                    stable_ptr: if_expr.stable_ptr.untyped(),
                    message: ManualIsOk.diagnostic_message().to_owned(),
                    severity: Severity::Warning,
                });
            }
            if check_manual_if(db, if_expr, arenas, ManualLint::ManualIsErr) {
                diagnostics.push(PluginDiagnostic {
                    stable_ptr: if_expr.stable_ptr.untyped(),
                    message: ManualIsErr.diagnostic_message().to_owned(),
                    severity: Severity::Warning,
                });
            }
        }
    }
}

/// Rewrites a manual implementation of is_some
pub fn fix_manual_is_some(db: &dyn SyntaxGroup, node: SyntaxNode) -> Option<(SyntaxNode, String)> {
    Some((node, fix_manual("is_some", db, node)))
}

// Rewrites a manual implementation of is_none
pub fn fix_manual_is_none(db: &dyn SyntaxGroup, node: SyntaxNode) -> Option<(SyntaxNode, String)> {
    Some((node, fix_manual("is_none", db, node)))
}

/// Rewrites a manual implementation of is_ok
pub fn fix_manual_is_ok(db: &dyn SyntaxGroup, node: SyntaxNode) -> Option<(SyntaxNode, String)> {
    Some((node, fix_manual("is_ok", db, node)))
}

/// Rewrites a manual implementation of is_err
pub fn fix_manual_is_err(db: &dyn SyntaxGroup, node: SyntaxNode) -> Option<(SyntaxNode, String)> {
    Some((node, fix_manual("is_err", db, node)))
}
