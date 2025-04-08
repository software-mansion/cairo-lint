use cairo_lang_defs::{ids::ModuleItemId, plugin::PluginDiagnostic};
use cairo_lang_diagnostics::Severity;
use cairo_lang_semantic::{db::SemanticGroup, Arenas, Expr, ExprIf, Statement};
use cairo_lang_syntax::node::TypedStablePtr;
use if_chain::if_chain;

use crate::{
    context::{CairoLintKind, Lint},
    helper::PANIC_PATH,
    queries::{get_all_function_bodies, get_all_if_expressions},
};

pub struct ManualAssert;

impl Lint for ManualAssert {
    fn allowed_name(&self) -> &'static str {
        "manual_assert"
    }

    fn diagnostic_message(&self) -> &'static str {
        "Manual assert detected. Consider using assert!() macro instead."
    }

    fn kind(&self) -> crate::context::CairoLintKind {
        CairoLintKind::ManualAssert
    }
}

pub fn check_manual_assert(
    db: &dyn SemanticGroup,
    item: &ModuleItemId,
    diagnostics: &mut Vec<PluginDiagnostic>,
) {
    let function_bodies = get_all_function_bodies(db, item);
    for function_body in function_bodies.iter() {
        let if_exprs = get_all_if_expressions(function_body);
        let arenas = &function_body.arenas;
        for if_expr in if_exprs.iter() {
            check_single_manual_assert(db, if_expr, arenas, diagnostics);
        }
    }
}

fn check_single_manual_assert(
    db: &dyn SemanticGroup,
    if_expr: &ExprIf,
    arenas: &Arenas,
    diagnostics: &mut Vec<PluginDiagnostic>,
) {
    let Expr::Block(ref if_block) = arenas.exprs[if_expr.if_block] else {
        return;
    };

    // If there's an else block we ignore it.
    if let Some(_) = if_expr.else_block {
        return;
    };

    // Without tail.
    if_chain! {
        if if_block.statements.len() == 1;
        if let Statement::Expr(ref inner_expr_stmt) = arenas.statements[if_block.statements[0]];
        if let Expr::FunctionCall(ref expr_func_call) = arenas.exprs[inner_expr_stmt.expr];
        if expr_func_call.function.full_path(db) == PANIC_PATH;
        then {
            diagnostics.push(PluginDiagnostic {
                stable_ptr: if_expr.stable_ptr.untyped(),
                message: ManualAssert.diagnostic_message().to_string(),
                severity: Severity::Warning,
            });
            return;
        }
    }

    // With tail.
    if_chain! {
        if if_block.statements.len() == 0;
        if let Some(expr_id) = if_block.tail;
        if let Expr::FunctionCall(ref expr_func_call) = arenas.exprs[expr_id];
        if expr_func_call.function.full_path(db) == PANIC_PATH;
        then {
            diagnostics.push(PluginDiagnostic {
                stable_ptr: if_expr.stable_ptr.untyped(),
                message: ManualAssert.diagnostic_message().to_string(),
                severity: Severity::Warning,
            });
        }
    }
}
