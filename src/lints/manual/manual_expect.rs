use cairo_lang_defs::ids::ModuleItemId;
use cairo_lang_defs::plugin::PluginDiagnostic;
use cairo_lang_diagnostics::Severity;

use cairo_lang_syntax::node::kind::SyntaxKind;
use cairo_lang_syntax::node::{
    SyntaxNode, TypedStablePtr, TypedSyntaxNode,
    ast::{ExprIf, ExprMatch},
};

use crate::context::{CairoLintKind, Lint};

use crate::fixer::InternalFix;
use crate::lints::manual::helpers::{
    MatchOnOption, MatchOnResult, expr_if_get_var_name_and_err, extract_err,
};
use crate::lints::manual::{ManualLint, check_manual, check_manual_if};
use crate::queries::{get_all_function_bodies, get_all_if_expressions, get_all_match_expressions};
use salsa::Database;

pub struct ManualExpect;

/// ## What it does
///
/// Checks for manual implementations of `expect`.
///
/// ## Example
///
/// ```cairo
/// fn main() {
///     let foo: Option::<i32> = Option::None;
///     let _foo = match foo {
///         Option::Some(x) => x,
///         Option::None => core::panic_with_felt252('err'),
///     };
/// }
/// ```
///
/// Can be rewritten as:
///
/// ```cairo
/// fn main() {
///     let foo: Option::<i32> = Option::None;
///     let _foo = foo.expect('err');
/// }
/// ```
impl Lint for ManualExpect {
    fn allowed_name(&self) -> &'static str {
        "manual_expect"
    }

    fn diagnostic_message(&self) -> &'static str {
        "Manual match for expect detected. Consider using `expect()` instead"
    }

    fn kind(&self) -> CairoLintKind {
        CairoLintKind::ManualExpect
    }

    fn has_fixer(&self) -> bool {
        true
    }

    fn fix<'db>(&self, db: &'db dyn Database, node: SyntaxNode<'db>) -> Option<InternalFix<'db>> {
        fix_manual_expect(db, node)
    }

    fn fix_message(&self) -> Option<&'static str> {
        Some("Replace manual `expect` with `expect()` method")
    }
}

#[tracing::instrument(skip_all, level = "trace")]
pub fn check_manual_expect<'db>(
    db: &'db dyn Database,
    item: &ModuleItemId<'db>,
    diagnostics: &mut Vec<PluginDiagnostic<'db>>,
) {
    let function_bodies = get_all_function_bodies(db, item);
    for function_body in function_bodies.iter() {
        let if_exprs = get_all_if_expressions(function_body);
        let match_exprs = get_all_match_expressions(function_body);
        let arenas = &function_body.arenas;
        for match_expr in match_exprs.iter() {
            if check_manual(db, match_expr, arenas, ManualLint::ManualOptExpect) {
                diagnostics.push(PluginDiagnostic {
                    stable_ptr: match_expr.stable_ptr.untyped(),
                    message: ManualExpect.diagnostic_message().to_owned(),
                    severity: Severity::Warning,
                    inner_span: None,
                });
            }

            if check_manual(db, match_expr, arenas, ManualLint::ManualResExpect) {
                diagnostics.push(PluginDiagnostic {
                    stable_ptr: match_expr.stable_ptr.untyped(),
                    message: ManualExpect.diagnostic_message().to_owned(),
                    severity: Severity::Warning,
                    inner_span: None,
                });
            }
        }
        for if_expr in if_exprs.iter() {
            if check_manual_if(db, if_expr, arenas, ManualLint::ManualOptExpect) {
                diagnostics.push(PluginDiagnostic {
                    stable_ptr: if_expr.stable_ptr.untyped(),
                    message: ManualExpect.diagnostic_message().to_owned(),
                    severity: Severity::Warning,
                    inner_span: None,
                });
            }

            if check_manual_if(db, if_expr, arenas, ManualLint::ManualResExpect) {
                diagnostics.push(PluginDiagnostic {
                    stable_ptr: if_expr.stable_ptr.untyped(),
                    message: ManualExpect.diagnostic_message().to_owned(),
                    severity: Severity::Warning,
                    inner_span: None,
                });
            }
        }
    }
}

/// Rewrites a manual implementation of expect
#[tracing::instrument(skip_all, level = "trace")]
pub fn fix_manual_expect<'db>(
    db: &'db dyn Database,
    node: SyntaxNode<'db>,
) -> Option<InternalFix<'db>> {
    let fix = match node.kind(db) {
        SyntaxKind::ExprMatch => {
            let expr_match = ExprMatch::from_syntax_node(db, node);
            let target_expr = expr_match.expr(db).as_syntax_node().get_text(db).trim_end();

            let arm = MatchOnOption::try_new(db, &expr_match)
                .map(|match_on_option| match_on_option.none_arm)
                .or_else(|| {
                    MatchOnResult::try_new(db, &expr_match)
                        .map(|match_on_result| match_on_result.err_arm)
                })
                .expect("Expected a match expression on either Option or Result");

            let err = extract_err(db, &arm);
            format!("{target_expr}.expect({err})")
        }
        SyntaxKind::ExprIf => {
            let expr_if = ExprIf::from_syntax_node(db, node);

            let (option_var_name, err) = expr_if_get_var_name_and_err(expr_if, db);

            format!("{}.expect({err})", option_var_name.trim_end())
        }
        _ => panic!("SyntaxKind should be either ExprIf or ExprMatch"),
    };
    Some(InternalFix {
        node,
        suggestion: fix,
        description: ManualExpect.fix_message().unwrap().to_string(),
        import_addition_paths: None,
    })
}
