use cairo_lang_defs::ids::ModuleItemId;
use cairo_lang_defs::plugin::PluginDiagnostic;
use cairo_lang_diagnostics::Severity;
use cairo_lang_semantic::db::SemanticGroup;
use cairo_lang_semantic::{Arenas, Expr, ExprBlock, ExprIf, Statement};
use cairo_lang_syntax::node::helpers::QueryAttrs;
use cairo_lang_syntax::node::{TypedStablePtr, TypedSyntaxNode};
use if_chain::if_chain;

use crate::queries::{get_all_function_bodies, get_all_if_expressions};

pub const COLLAPSIBLE_IF_ELSE: &str = "Consider using else if instead of else { if ... }";
pub const LINT_NAME: &str = "collapsible_if_else";

/// Checks for
/// ```ignore
/// if cond {
///     ...
/// } else {
///     if second_cond {
///         ...
///     }
/// }
/// ```
/// This can be collapsed to:
/// ```ignore
/// if cond {
///     ...
/// } else if second_cond {
///     ...
/// }
/// ```
pub fn check_collapsible_if_else(
    db: &dyn SemanticGroup,
    item: &ModuleItemId,
    diagnostics: &mut Vec<PluginDiagnostic>,
) {
    let function_bodies = get_all_function_bodies(db, item);
    for function_body in function_bodies.iter() {
        let if_exprs = get_all_if_expressions(function_body);
        let arenas = &function_body.arenas;
        for if_expr in if_exprs.iterk() {
            check_single_collapsible_if_else(db, if_expr, arenas, diagnostics);
        }
    }
}

fn check_single_collapsible_if_else(
    db: &dyn SemanticGroup,
    if_expr: &ExprIf,
    arenas: &Arenas,
    diagnostics: &mut Vec<PluginDiagnostic>,
) {
    let mut current_node = if_expr.stable_ptr.lookup(db.upcast()).as_syntax_node();
    while let Some(node) = current_node.parent() {
        if node.has_attr_with_arg(db.upcast(), "allow", LINT_NAME) {
            return;
        }
        current_node = node;
    }
    // Extract the expression from the ElseClause
    let Some(else_block) = if_expr.else_block else {
        return;
    };

    let Expr::Block(block_expr) = &arenas.exprs[else_block] else {
        return;
    };
    // Check if the expression is a block (not else if)
    let is_if = is_only_statement_if(block_expr, arenas);

    if is_if {
        diagnostics.push(PluginDiagnostic {
            stable_ptr: if_expr.stable_ptr.untyped(),
            message: COLLAPSIBLE_IF_ELSE.to_string(),
            severity: Severity::Warning,
        });
    }
}

fn is_only_statement_if(block_expr: &ExprBlock, arenas: &Arenas) -> bool {
    if block_expr.statements.len() == 1 && block_expr.tail.is_none() {
        if_chain! {
            if let Statement::Expr(statement_expr) = &arenas.statements[block_expr.statements[0]];
            if matches!(arenas.exprs[statement_expr.expr], Expr::If(_));
            then {
                return true;
            } else {
                return false;
            }
        }
    }

    if_chain! {
        if let Some(tail) = block_expr.tail;
        if block_expr.statements.is_empty();
        then {
            return matches!(arenas.exprs[tail], Expr::If(_));
        }
    }

    false
}
