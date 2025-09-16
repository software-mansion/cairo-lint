use cairo_lang_defs::ids::ModuleItemId;
use cairo_lang_defs::plugin::PluginDiagnostic;
use cairo_lang_diagnostics::Severity;

use cairo_lang_syntax::node::{SyntaxNode, TypedStablePtr};

use crate::context::{CairoLintKind, Lint};

use crate::fixer::InternalFix;
use crate::lints::manual::{ManualLint, check_manual, check_manual_if};
use crate::queries::{get_all_function_bodies, get_all_if_expressions, get_all_match_expressions};

use super::helpers::fix_manual;

use salsa::Database;

pub struct ManualErr;

/// ## What it does
///
/// Checks for manual implementations of `err` in match and if expressions.
///
/// ## Example
///
/// ```cairo
/// fn main() {
///     let foo: Result<i32> = Result::Err('err');
///     let _foo = match foo {
///         Result::Ok(_) => Option::None,
///         Result::Err(x) => Option::Some(x),
///     };
/// }
/// ```
///
/// Can be rewritten as:
///
/// ```cairo
/// fn main() {
///     let foo: Result<i32> = Result::Err('err');
///     let _foo = foo.err();
/// }
/// ```
impl Lint for ManualErr {
    fn allowed_name(&self) -> &'static str {
        "manual_err"
    }

    fn diagnostic_message(&self) -> &'static str {
        "Manual match for `err` detected. Consider using `err()` instead"
    }

    fn kind(&self) -> CairoLintKind {
        CairoLintKind::ManualErr
    }

    fn has_fixer(&self) -> bool {
        true
    }

    fn fix<'db>(&self, db: &'db dyn Database, node: SyntaxNode<'db>) -> Option<InternalFix<'db>> {
        fix_manual_err(db, node)
    }

    fn fix_message(&self) -> Option<&'static str> {
        Some("Replace manual `err` with `err()` method")
    }
}

#[tracing::instrument(skip_all, level = "trace")]
pub fn check_manual_err<'db>(
    db: &'db dyn Database,
    item: &ModuleItemId<'db>,
    diagnostics: &mut Vec<PluginDiagnostic<'db>>,
) {
    let function_bodies = get_all_function_bodies(db, item);
    for function_body in function_bodies.iter() {
        let match_exprs = get_all_match_expressions(function_body);
        let if_exprs = get_all_if_expressions(function_body);
        let arenas = &function_body.arenas;
        for match_expr in match_exprs.iter() {
            if check_manual(db, match_expr, arenas, ManualLint::ManualErr) {
                diagnostics.push(PluginDiagnostic {
                    stable_ptr: match_expr.stable_ptr.untyped(),
                    message: ManualErr.diagnostic_message().to_owned(),
                    severity: Severity::Warning,
                    inner_span: None,
                });
            }
        }
        for if_expr in if_exprs.iter() {
            if check_manual_if(db, if_expr, arenas, ManualLint::ManualErr) {
                diagnostics.push(PluginDiagnostic {
                    stable_ptr: if_expr.stable_ptr.untyped(),
                    message: ManualErr.diagnostic_message().to_owned(),
                    severity: Severity::Warning,
                    inner_span: None,
                });
            }
        }
    }
}

/// Rewrites a manual implementation of err
#[tracing::instrument(skip_all, level = "trace")]
pub fn fix_manual_err<'db>(
    db: &'db dyn Database,
    node: SyntaxNode<'db>,
) -> Option<InternalFix<'db>> {
    Some(InternalFix {
        node,
        suggestion: fix_manual("err", db, node),
        description: ManualErr.fix_message().unwrap().to_string(),
        import_addition_paths: None,
    })
}
