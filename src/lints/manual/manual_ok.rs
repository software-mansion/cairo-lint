use cairo_lang_defs::ids::ModuleItemId;
use cairo_lang_defs::plugin::PluginDiagnostic;
use cairo_lang_diagnostics::Severity;
use cairo_lang_semantic::db::SemanticGroup;
use cairo_lang_syntax::node::db::SyntaxGroup;
use cairo_lang_syntax::node::{SyntaxNode, TypedStablePtr};

use crate::context::{CairoLintKind, Lint};
use crate::fixes::InternalFix;
use crate::lints::manual::{check_manual, check_manual_if, ManualLint};
use crate::queries::{get_all_function_bodies, get_all_if_expressions, get_all_match_expressions};

use super::helpers::fix_manual;

pub struct ManualOk;

/// ## What it does
///
/// Checks for manual implementation of `ok` method in match and if expressions.
///
/// ## Example
///
/// ```cairo
/// fn main() {
///     let res_val: Result<i32> = Result::Err('err');
///     let _a = match res_val {
///         Result::Ok(x) => Option::Some(x),
///         Result::Err(_) => Option::None,
///     };
/// }
/// ```
///
/// Can be replaced with:
///
/// ```cairo
/// fn main() {
///     let res_val: Result<i32> = Result::Err('err');
///     let _a = res_val.ok();
/// }
/// ```
impl Lint for ManualOk {
    fn allowed_name(&self) -> &'static str {
        "manual_ok"
    }

    fn diagnostic_message(&self) -> &'static str {
        "Manual match for `ok` detected. Consider using `ok()` instead"
    }

    fn kind(&self) -> CairoLintKind {
        CairoLintKind::ManualOk
    }

    fn has_fixer(&self) -> bool {
        true
    }

    fn fix(&self, db: &dyn SemanticGroup, node: SyntaxNode) -> Option<InternalFix> {
        fix_manual_ok(db.upcast(), node)
    }

    fn fix_message(&self) -> Option<&'static str> {
        Some("Replace manual conversion with `ok()` method")
    }
}

pub fn check_manual_ok(
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
            if check_manual(db, match_expr, arenas, ManualLint::ManualOk) {
                diagnostics.push(PluginDiagnostic {
                    stable_ptr: match_expr.stable_ptr.untyped(),
                    message: ManualOk.diagnostic_message().to_owned(),
                    severity: Severity::Warning,
                    inner_span: None,
                });
            }
        }
        for if_expr in if_exprs.iter() {
            if check_manual_if(db, if_expr, arenas, ManualLint::ManualOk) {
                diagnostics.push(PluginDiagnostic {
                    stable_ptr: if_expr.stable_ptr.untyped(),
                    message: ManualOk.diagnostic_message().to_owned(),
                    severity: Severity::Warning,
                    inner_span: None,
                });
            }
        }
    }
}

/// Rewrites a manual implementation of ok
pub fn fix_manual_ok(db: &dyn SyntaxGroup, node: SyntaxNode) -> Option<InternalFix> {
    Some(InternalFix {
        node,
        suggestion: fix_manual("ok", db, node),
        description: ManualOk.fix_message().unwrap().to_string(),
        import_addition_paths: None,
    })
}
