use std::sync::Arc;

use cairo_lang_defs::ids::{FunctionWithBodyId, ModuleItemId};
use cairo_lang_semantic::db::SemanticGroup;
use cairo_lang_semantic::{
    Arenas, Expr, ExprFunctionCall, ExprIf, ExprLogicalOperator, ExprLoop, ExprMatch, ExprWhile,
    FunctionBody, Pattern, Statement, StatementBreak,
};
use cairo_lang_syntax::node::ast::Expr as AstExpr;
use cairo_lang_syntax::node::kind::SyntaxKind;
use cairo_lang_syntax::node::TypedStablePtr;
use cairo_lang_syntax::node::TypedSyntaxNode;
use if_chain::if_chain;

pub fn get_all_checkable_functions(
    db: &dyn SemanticGroup,
    item: &ModuleItemId,
) -> Vec<FunctionWithBodyId> {
    match item {
        ModuleItemId::FreeFunction(free_function_id) => {
            vec![FunctionWithBodyId::Free(*free_function_id)]
        }
        ModuleItemId::Impl(impl_id) => db.impl_functions(*impl_id).map_or(vec![], |functions| {
            functions
                .iter()
                .map(|(_, fn_id)| FunctionWithBodyId::Impl(*fn_id))
                .collect()
        }),
        _ => vec![],
    }
}

pub fn get_all_function_bodies(
    db: &dyn SemanticGroup,
    item: &ModuleItemId,
) -> Vec<Arc<FunctionBody>> {
    get_all_checkable_functions(db, item)
        .iter()
        .filter_map(|function| db.function_body(*function).ok())
        .collect()
}

pub fn get_all_parenthesized_expressions(
    db: &dyn SemanticGroup,
    item: &ModuleItemId,
) -> Vec<AstExpr> {
    let function_nodes = match item {
        ModuleItemId::Constant(id) => id
            .stable_ptr(db.upcast())
            .lookup(db.upcast())
            .as_syntax_node(),
        ModuleItemId::FreeFunction(id) => id
            .stable_ptr(db.upcast())
            .lookup(db.upcast())
            .as_syntax_node(),
        ModuleItemId::Impl(id) => id
            .stable_ptr(db.upcast())
            .lookup(db.upcast())
            .as_syntax_node(),
        _ => return vec![],
    }
    .descendants(db.upcast());

    function_nodes
        .filter(|node| node.kind(db.upcast()) == SyntaxKind::ExprParenthesized)
        .map(|node| AstExpr::from_syntax_node(db.upcast(), node))
        .collect()
}

pub fn get_all_match_expressions(function_body: &Arc<FunctionBody>) -> Vec<ExprMatch> {
    function_body
        .arenas
        .exprs
        .iter()
        .filter_map(|(_expression_id, expression)| {
            if let Expr::Match(expr_match) = expression {
                Some(expr_match.clone())
            } else {
                None
            }
        })
        .collect()
}

pub fn get_all_loop_expressions(function_body: &Arc<FunctionBody>) -> Vec<ExprLoop> {
    function_body
        .arenas
        .exprs
        .iter()
        .filter_map(|(_expression_id, expression)| {
            if let Expr::Loop(expr_loop) = expression {
                Some(expr_loop.clone())
            } else {
                None
            }
        })
        .collect()
}

pub fn get_all_function_calls(function_body: &Arc<FunctionBody>) -> Vec<ExprFunctionCall> {
    function_body
        .arenas
        .exprs
        .iter()
        .filter_map(|(_expression_id, expression)| {
            if let Expr::FunctionCall(expr_func) = expression {
                Some(expr_func.clone())
            } else {
                None
            }
        })
        .collect()
}

pub fn get_all_logical_operator_expressions(
    function_body: &Arc<FunctionBody>,
) -> Vec<ExprLogicalOperator> {
    function_body
        .arenas
        .exprs
        .iter()
        .filter_map(|(_expression_id, expression)| {
            if let Expr::LogicalOperator(expr_logical_operator) = expression {
                Some(expr_logical_operator.clone())
            } else {
                None
            }
        })
        .collect()
}

pub fn get_all_if_expressions(function_body: &Arc<FunctionBody>) -> Vec<ExprIf> {
    function_body
        .arenas
        .exprs
        .iter()
        .filter_map(|(_expression_id, expression)| {
            if let Expr::If(expr_if) = expression {
                Some(expr_if.clone())
            } else {
                None
            }
        })
        .collect()
}

pub fn get_all_while_expressions(function_body: &Arc<FunctionBody>) -> Vec<ExprWhile> {
    function_body
        .arenas
        .exprs
        .iter()
        .filter_map(|(_expression_id, expression)| {
            if let Expr::While(expr_while) = expression {
                Some(expr_while.clone())
            } else {
                None
            }
        })
        .collect()
}

pub fn get_all_break_statements(function_body: &Arc<FunctionBody>) -> Vec<StatementBreak> {
    function_body
        .arenas
        .statements
        .iter()
        .filter_map(|(_statement_id, statement)| {
            if let Statement::Break(statement_break) = statement {
                Some(statement_break.clone())
            } else {
                None
            }
        })
        .collect()
}

/// This function checks if the given `if` expression is an assert macro call.
/// It's kind of a hack, but unfortunately compiler expands the `assert!()` macro before any other user macros,
/// so we have to work around it.
pub fn is_assert_macro_call(db: &dyn SemanticGroup, arenas: &Arenas, expr: &ExprIf) -> bool {
    if_chain! {
        if let Expr::Block(ref if_block_expr) = arenas.exprs[expr.if_block];
        if let Statement::Let(ref if_block_let_stmt) = arenas.statements[if_block_expr.statements[0]];
        if let Pattern::Variable(ref if_block_let_stmt_pattern) = arenas.patterns[if_block_let_stmt.pattern];
        if if_block_let_stmt_pattern.name == "__formatter_for_assert_macro__";
        if if_block_let_stmt_pattern.var.ty.short_name(db) == "core::fmt::Formatter";
        then {
          return true;
        }
    }
    false
}
