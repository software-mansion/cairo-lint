use cairo_lang_defs::{ids::ModuleItemId, plugin::PluginDiagnostic};
use cairo_lang_diagnostics::Severity;
use cairo_lang_semantic::{db::SemanticGroup, Arenas, Expr, ExprBlock, ExprIf, Statement};
use cairo_lang_syntax::node::TypedStablePtr;
use if_chain::if_chain;

use crate::{
    context::{CairoLintKind, Lint},
    helper::is_panic_expr,
    queries::{get_all_function_bodies, get_all_if_expressions},
};

pub struct ManualAssert;

/// ## What it does
///
/// Checks for manual implementations of `assert` macro in `if` expressions.
///
/// ## Example
///
/// ```cairo
/// fn main() {
///     let a = 5;
///     if a == 5 {
///         panic!("a shouldn't be equal to 5");
///     }
/// }
/// ```
///
/// Can be rewritten as:
///
/// ```cairo
/// fn main() {
///     let a = 5;
///     assert!(a != 5, "a shouldn't be equal to 5");
/// }
/// ```
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

    check_single_condition_block(db, if_block, if_expr, arenas, diagnostics);

    if_chain! {
      if let Some(else_block) = if_expr.else_block;
      if let Expr::Block(ref else_block) = arenas.exprs[else_block];
      then {
        check_single_condition_block(db, else_block, if_expr, arenas, diagnostics);
      }
    }
}

fn check_single_condition_block(
    db: &dyn SemanticGroup,
    condition_block_expr: &ExprBlock,
    if_expr: &ExprIf,
    arenas: &Arenas,
    diagnostics: &mut Vec<PluginDiagnostic>,
) {
    // Without tail.
    if_chain! {
        if !condition_block_expr.statements.is_empty();
        if let Statement::Expr(ref inner_expr_stmt) = arenas.statements[condition_block_expr.statements[0]];
        if is_panic_expr(db, arenas, inner_expr_stmt.expr);
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
        if condition_block_expr.statements.is_empty();
        if let Some(expr_id) = condition_block_expr.tail;
        if is_panic_expr(db, arenas, expr_id);
        then {
            diagnostics.push(PluginDiagnostic {
                stable_ptr: if_expr.stable_ptr.untyped(),
                message: ManualAssert.diagnostic_message().to_string(),
                severity: Severity::Warning,
            });
        }
    }
}
