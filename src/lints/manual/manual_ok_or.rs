use cairo_lang_defs::ids::ModuleItemId;
use cairo_lang_defs::plugin::PluginDiagnostic;
use cairo_lang_diagnostics::Severity;
use cairo_lang_semantic::db::SemanticGroup;
use cairo_lang_syntax::node::{
    ast::{ExprIf, ExprMatch},
    db::SyntaxGroup,
    kind::SyntaxKind,
    SyntaxNode, TypedStablePtr, TypedSyntaxNode,
};

use crate::{
    context::CairoLintKind,
    fixes::InternalFix,
    queries::{get_all_function_bodies, get_all_if_expressions, get_all_match_expressions},
};
use crate::{
    context::Lint,
    lints::manual::{
        check_manual, check_manual_if,
        helpers::{expr_if_get_var_name_and_err, expr_match_get_var_name_and_err},
        ManualLint,
    },
};

pub struct ManualOkOr;

/// ## What it does
///
/// Checks for manual implementations of ok_or.
///
/// ## Example
///
/// ```cairo
/// fn main() {
///     let foo: Option<i32> = Option::None;
///     let _foo = match foo {
///         Option::Some(v) => Result::Ok(v),
///         Option::None => Result::Err('this is an err'),
///     };
/// }
/// ```
///
/// Can be rewritten as:
///
/// ```cairo
/// fn main() {
///     let foo: Option<i32> = Option::None;
///     let _foo = foo.ok_or('this is an err');
/// }
/// ```
impl Lint for ManualOkOr {
    fn allowed_name(&self) -> &'static str {
        "manual_ok_or"
    }

    fn diagnostic_message(&self) -> &'static str {
        "Manual match for Option<T> detected. Consider using ok_or instead"
    }

    fn kind(&self) -> CairoLintKind {
        CairoLintKind::ManualOkOr
    }

    fn has_fixer(&self) -> bool {
        true
    }

    fn fix(&self, db: &dyn SemanticGroup, node: SyntaxNode) -> Option<InternalFix> {
        fix_manual_ok_or(db.upcast(), node)
    }

    fn fix_message(&self) -> Option<&'static str> {
        Some("Replace manual conversion with `ok_or()` method")
    }
}

pub fn check_manual_ok_or(
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
            if check_manual(db, match_expr, arenas, ManualLint::ManualOkOr) {
                diagnostics.push(PluginDiagnostic {
                    stable_ptr: match_expr.stable_ptr.untyped(),
                    message: ManualOkOr.diagnostic_message().to_owned(),
                    severity: Severity::Warning,
                    relative_span: None,
                    inner_span: None,
                });
            }
        }
        for if_expr in if_exprs.iter() {
            if check_manual_if(db, if_expr, arenas, ManualLint::ManualOkOr) {
                diagnostics.push(PluginDiagnostic {
                    stable_ptr: if_expr.stable_ptr.untyped(),
                    message: ManualOkOr.diagnostic_message().to_owned(),
                    severity: Severity::Warning,
                    relative_span: None,
                    inner_span: None,
                });
            }
        }
    }
}

/// Rewrites a manual implementation of ok_or
pub fn fix_manual_ok_or(db: &dyn SyntaxGroup, node: SyntaxNode) -> Option<InternalFix> {
    let fix = match node.kind(db) {
        SyntaxKind::ExprMatch => {
            let expr_match = ExprMatch::from_syntax_node(db, node);

            let (option_var_name, none_arm_err) =
                expr_match_get_var_name_and_err(expr_match, db, 1);

            format!("{}.ok_or({none_arm_err})", option_var_name.trim_end())
        }
        SyntaxKind::ExprIf => {
            let expr_if = ExprIf::from_syntax_node(db, node);

            let (option_var_name, err) = expr_if_get_var_name_and_err(expr_if, db);

            format!("{}.ok_or({})", option_var_name.trim_end(), err)
        }
        _ => panic!("SyntaxKind should be either ExprIf or ExprMatch"),
    };
    Some(InternalFix {
        node,
        suggestion: fix,
        description: ManualOkOr.fix_message().unwrap().to_string(),
        import_addition_paths: None,
    })
}
