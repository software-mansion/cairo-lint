use cairo_lang_defs::ids::ModuleItemId;
use cairo_lang_defs::plugin::PluginDiagnostic;
use cairo_lang_diagnostics::Severity;
use cairo_lang_semantic::db::SemanticGroup;
use cairo_lang_syntax::node::{
    SyntaxNode, TypedStablePtr, TypedSyntaxNode,
    ast::{ExprIf, ExprMatch},
    db::SyntaxGroup,
    kind::SyntaxKind,
};

use crate::{
    context::CairoLintKind,
    corelib::CorelibContext,
    fixer::InternalFix,
    queries::{get_all_function_bodies, get_all_if_expressions, get_all_match_expressions},
};
use crate::{
    context::Lint,
    lints::manual::{
        ManualLint, check_manual, check_manual_if,
        helpers::{expr_if_get_var_name_and_err, expr_match_get_var_name_and_err},
    },
};

pub struct ManualExpectErr;

/// ## What it does
///
/// Checks for manual implementation of `expect_err` method in match and if expressions.
///
/// ## Example
///
/// ```cairo
/// fn main() {
///     let foo: Result<i32> = Result::Err('err');
///     let err = 'this is an err';
///     let _foo = match foo {
///         Result::Ok(_) => core::panic_with_felt252(err),
///         Result::Err(x) => x,
///     };
/// }
/// ```
///
/// Can be rewritten as:
///
/// ```cairo
/// fn main() {
///     let foo: Result<i32> = Result::Err('err');
///     let err = 'this is an err';
///     let _foo = foo.expect_err(err);
/// }
/// ```
impl Lint for ManualExpectErr {
    fn allowed_name(&self) -> &'static str {
        "manual_expect_err"
    }

    fn diagnostic_message(&self) -> &'static str {
        "Manual match for `expect_err` detected. Consider using `expect_err()` instead"
    }

    fn kind(&self) -> CairoLintKind {
        CairoLintKind::ManualExpectErr
    }

    fn has_fixer(&self) -> bool {
        true
    }

    fn fix<'db>(
        &self,
        db: &'db dyn SemanticGroup,
        node: SyntaxNode<'db>,
    ) -> Option<InternalFix<'db>> {
        fix_manual_expect_err(db, node)
    }

    fn fix_message(&self) -> Option<&'static str> {
        Some("Replace manual `expect_err` with `expect_err()` method")
    }
}

#[tracing::instrument(skip_all, level = "trace")]
pub fn check_manual_expect_err<'db>(
    db: &'db dyn SemanticGroup,
    _corelib_context: &CorelibContext<'db>,
    item: &ModuleItemId<'db>,
    diagnostics: &mut Vec<PluginDiagnostic<'db>>,
) {
    let function_bodies = get_all_function_bodies(db, item);
    for function_body in function_bodies.iter() {
        let if_exprs = get_all_if_expressions(function_body);
        let match_exprs = get_all_match_expressions(function_body);
        let arenas = &function_body.arenas;
        for match_expr in match_exprs.iter() {
            if check_manual(db, match_expr, arenas, ManualLint::ManualExpectErr) {
                diagnostics.push(PluginDiagnostic {
                    stable_ptr: match_expr.stable_ptr.untyped(),
                    message: ManualExpectErr.diagnostic_message().to_owned(),
                    severity: Severity::Warning,
                    inner_span: None,
                });
            }
        }
        for if_expr in if_exprs.iter() {
            if check_manual_if(db, if_expr, arenas, ManualLint::ManualExpectErr) {
                diagnostics.push(PluginDiagnostic {
                    stable_ptr: if_expr.stable_ptr.untyped(),
                    message: ManualExpectErr.diagnostic_message().to_owned(),
                    severity: Severity::Warning,
                    inner_span: None,
                });
            }
        }
    }
}

/// Rewrites a manual implementation of expect err
#[tracing::instrument(skip_all, level = "trace")]
pub fn fix_manual_expect_err<'db>(
    db: &'db dyn SyntaxGroup,
    node: SyntaxNode<'db>,
) -> Option<InternalFix<'db>> {
    let fix = match node.kind(db) {
        SyntaxKind::ExprMatch => {
            let expr_match = ExprMatch::from_syntax_node(db, node);

            let (option_var_name, none_arm_err) =
                expr_match_get_var_name_and_err(expr_match, db, 0);

            format!("{}.expect_err({none_arm_err})", option_var_name.trim_end())
        }
        SyntaxKind::ExprIf => {
            let expr_if = ExprIf::from_syntax_node(db, node);

            let (option_var_name, err) = expr_if_get_var_name_and_err(expr_if, db);

            format!("{}.expect_err({err})", option_var_name.trim_end())
        }
        _ => panic!("SyntaxKind should be either ExprIf or ExprMatch"),
    };
    Some(InternalFix {
        node,
        suggestion: fix,
        description: ManualExpectErr.fix_message().unwrap().to_string(),
        import_addition_paths: None,
    })
}
