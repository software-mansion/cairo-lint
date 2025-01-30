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

const MANUAL_OK_OR: &str = "Manual match for Option<T> detected. Consider using ok_or instead";
const MANUAL_OK_OR_LINT_NAME: &str = "manual_ok_or";

pub struct ManualOkOr;

impl Lint for ManualOkOr {
    fn allowed_name(self: &Self) -> &'static str {
        MANUAL_OK_OR_LINT_NAME
    }

    fn diagnostic_message(self: &Self) -> &'static str {
        MANUAL_OK_OR
    }

    fn kind(self: &Self) -> CairoLintKind {
        CairoLintKind::ManualOkOr
    }

    fn has_fixer(self: &Self) -> bool {
        true
    }

    fn fix(self: &Self, db: &dyn SyntaxGroup, node: SyntaxNode) -> Option<(SyntaxNode, String)> {
        fix_manual_ok_or(db, node)
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
                    message: MANUAL_OK_OR.to_owned(),
                    severity: Severity::Warning,
                });
            }
        }
        for if_expr in if_exprs.iter() {
            if check_manual_if(db, if_expr, arenas, ManualLint::ManualOkOr) {
                diagnostics.push(PluginDiagnostic {
                    stable_ptr: if_expr.stable_ptr.untyped(),
                    message: MANUAL_OK_OR.to_owned(),
                    severity: Severity::Warning,
                });
            }
        }
    }
}

/// Rewrites a manual implementation of ok_or
pub fn fix_manual_ok_or(db: &dyn SyntaxGroup, node: SyntaxNode) -> Option<(SyntaxNode, String)> {
    let fix = match node.kind(db) {
        SyntaxKind::ExprMatch => {
            let expr_match = ExprMatch::from_syntax_node(db, node.clone());

            let (option_var_name, none_arm_err) =
                expr_match_get_var_name_and_err(expr_match, db, 1);

            format!("{option_var_name}.ok_or({none_arm_err})")
        }
        SyntaxKind::ExprIf => {
            let expr_if = ExprIf::from_syntax_node(db, node.clone());

            let (option_var_name, err) = expr_if_get_var_name_and_err(expr_if, db);

            format!("{option_var_name}.ok_or({err})")
        }
        _ => panic!("SyntaxKind should be either ExprIf or ExprMatch"),
    };
    Some((node, fix))
}
