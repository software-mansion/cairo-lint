use cairo_lang_defs::ids::ModuleItemId;
use cairo_lang_defs::plugin::PluginDiagnostic;
use cairo_lang_diagnostics::Severity;
use cairo_lang_semantic::db::SemanticGroup;
use cairo_lang_semantic::{Arenas, Condition, Expr, ExprIf};
use cairo_lang_syntax::node::helpers::QueryAttrs;
use cairo_lang_syntax::node::{TypedStablePtr, TypedSyntaxNode};
use if_chain::if_chain;

use crate::queries::{get_all_function_bodies, get_all_if_expressions};

use super::ensure_no_ref_arg;

pub const DUPLICATE_IF_CONDITION: &str = "Consecutive `if` with the same condition found.";

pub const LINT_NAME: &str = "ifs_same_cond";

pub fn check_duplicate_if_condition(
    db: &dyn SemanticGroup,
    item: &ModuleItemId,
    diagnostics: &mut Vec<PluginDiagnostic>,
) {
  let function_bodies = get_all_function_bodies(db, item);
    for function_body in function_bodies.iter() {
        let if_exprs = get_all_if_expressions(function_body);
        let arenas = &function_body.arenas;
        for if_expr in if_exprs.iter() {
          check_single_duplicate_if_condition(db, if_expr, arenas, diagnostics);
        }
      }
}

fn check_single_duplicate_if_condition(
    db: &dyn SemanticGroup,
    if_expr: &ExprIf,
    arenas: &Arenas,
    diagnostics: &mut Vec<PluginDiagnostic>,
) {
    // Checks if the lint is allowed in any upper scope
    let mut current_node = if_expr.stable_ptr.lookup(db.upcast()).as_syntax_node();
    while let Some(node) = current_node.parent() {
        if node.has_attr_with_arg(db.upcast(), "allow", LINT_NAME) {
            return;
        }
        current_node = node;
    }
    let cond_expr = match &if_expr.condition {
        Condition::BoolExpr(expr_id) => &arenas.exprs[*expr_id],
        Condition::Let(expr_id, _patterns) => &arenas.exprs[*expr_id],
    };

    if_chain! {
        if let Expr::FunctionCall(func_call) = cond_expr;
        if ensure_no_ref_arg(arenas, func_call);
        then {
            return;
        }
    }

    let mut current_block = if_expr.else_block;
    let if_condition_text = cond_expr
        .stable_ptr()
        .lookup(db.upcast())
        .as_syntax_node()
        .get_text_without_trivia(db.upcast());

    while let Some(expr_id) = current_block {
        if let Expr::If(else_if_block) = &arenas.exprs[expr_id] {
            current_block = else_if_block.else_block;
            let else_if_cond = match &else_if_block.condition {
                Condition::BoolExpr(expr_id) => &arenas.exprs[*expr_id],
                Condition::Let(expr_id, _patterns) => &arenas.exprs[*expr_id],
            };

            if_chain! {
                if let Expr::FunctionCall(func_call) = else_if_cond;
                if ensure_no_ref_arg(arenas, func_call);
                then {
                    continue;
                }
            }

            let else_if_condition_text = else_if_cond
                .stable_ptr()
                .lookup(db.upcast())
                .as_syntax_node()
                .get_text_without_trivia(db.upcast());

            if if_condition_text == else_if_condition_text {
                diagnostics.push(PluginDiagnostic {
                    stable_ptr: if_expr.stable_ptr.untyped(),
                    message: DUPLICATE_IF_CONDITION.to_string(),
                    severity: Severity::Warning,
                });
                break;
            }
        } else {
            break;
        }
    }
}
