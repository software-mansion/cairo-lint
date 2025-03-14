use cairo_lang_defs::ids::ModuleItemId;
use cairo_lang_defs::plugin::PluginDiagnostic;
use cairo_lang_diagnostics::Severity;
use cairo_lang_semantic::db::SemanticGroup;
use cairo_lang_syntax::node::db::SyntaxGroup;
use cairo_lang_syntax::node::kind::SyntaxKind;
use cairo_lang_syntax::node::{
    ast::{ExprIf, ExprMatch},
    SyntaxNode, TypedStablePtr, TypedSyntaxNode,
};

use crate::context::{CairoLintKind, Lint};
use crate::lints::manual::helpers::{
    expr_if_get_var_name_and_err, expr_match_get_var_name_and_err,
};
use crate::lints::manual::{check_manual, check_manual_if, ManualLint};
use crate::queries::{get_all_function_bodies, get_all_if_expressions, get_all_match_expressions};

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

    fn fix(&self, db: &dyn SyntaxGroup, node: SyntaxNode) -> Option<(SyntaxNode, String)> {
        fix_manual_expect(db, node)
    }
}

pub fn check_manual_expect(
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
            if check_manual(db, match_expr, arenas, ManualLint::ManualOptExpect) {
                diagnostics.push(PluginDiagnostic {
                    stable_ptr: match_expr.stable_ptr.untyped(),
                    message: ManualExpect.diagnostic_message().to_owned(),
                    severity: Severity::Warning,
                });
            }

            if check_manual(db, match_expr, arenas, ManualLint::ManualResExpect) {
                diagnostics.push(PluginDiagnostic {
                    stable_ptr: match_expr.stable_ptr.untyped(),
                    message: ManualExpect.diagnostic_message().to_owned(),
                    severity: Severity::Warning,
                });
            }
        }
        for if_expr in if_exprs.iter() {
            if check_manual_if(db, if_expr, arenas, ManualLint::ManualOptExpect) {
                diagnostics.push(PluginDiagnostic {
                    stable_ptr: if_expr.stable_ptr.untyped(),
                    message: ManualExpect.diagnostic_message().to_owned(),
                    severity: Severity::Warning,
                });
            }

            if check_manual_if(db, if_expr, arenas, ManualLint::ManualResExpect) {
                diagnostics.push(PluginDiagnostic {
                    stable_ptr: if_expr.stable_ptr.untyped(),
                    message: ManualExpect.diagnostic_message().to_owned(),
                    severity: Severity::Warning,
                });
            }
        }
    }
}

/// Rewrites a manual implementation of expect
pub fn fix_manual_expect(db: &dyn SyntaxGroup, node: SyntaxNode) -> Option<(SyntaxNode, String)> {
    let fix = match node.kind(db) {
        SyntaxKind::ExprMatch => {
            let expr_match = ExprMatch::from_syntax_node(db, node.clone());

            let (option_var_name, none_arm_err) =
                expr_match_get_var_name_and_err(expr_match, db, 1);

            format!("{}.expect({none_arm_err})", option_var_name.trim_end())
        }
        SyntaxKind::ExprIf => {
            let expr_if = ExprIf::from_syntax_node(db, node.clone());

            let (option_var_name, err) = expr_if_get_var_name_and_err(expr_if, db);

            format!("{}.expect({err})", option_var_name.trim_end())
        }
        _ => panic!("SyntaxKind should be either ExprIf or ExprMatch"),
    };
    Some((node, fix))
}
