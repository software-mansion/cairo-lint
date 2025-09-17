use cairo_lang_defs::ids::{FunctionWithBodyId, ModuleItemId};
use cairo_lang_semantic::items::function_with_body::FunctionWithBodySemantic;
use cairo_lang_semantic::items::imp::ImplSemantic;
use cairo_lang_semantic::items::trt::TraitSemantic;
use cairo_lang_semantic::{
    Arenas, Condition, Expr, ExprFunctionCall, ExprIf, ExprLogicalOperator, ExprLoop, ExprMatch,
    ExprWhile, FunctionBody, Pattern, Statement, StatementBreak,
};
use cairo_lang_syntax::node::TypedSyntaxNode;
use cairo_lang_syntax::node::ast::ExprParenthesized;
use cairo_lang_syntax::node::kind::SyntaxKind;
use cairo_lang_syntax::node::{SyntaxNode, TypedStablePtr};
use if_chain::if_chain;
use itertools::chain;

use crate::helper::{ASSERT_FORMATTER_NAME, ASSERT_PATH};
use salsa::Database;

#[tracing::instrument(skip_all, level = "trace")]
pub fn get_all_checkable_functions<'db>(
    db: &'db dyn Database,
    item: &ModuleItemId<'db>,
) -> Vec<FunctionWithBodyId<'db>> {
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
        ModuleItemId::Trait(trait_id) => {
            db.trait_functions(*trait_id).map_or(vec![], |functions| {
                functions
                    .iter()
                    .map(|(_, trait_fn_id)| FunctionWithBodyId::Trait(*trait_fn_id))
                    .collect()
            })
        }
        _ => vec![],
    }
}

#[tracing::instrument(skip_all, level = "trace")]
pub fn get_all_function_bodies<'db>(
    db: &'db dyn Database,
    item: &ModuleItemId<'db>,
) -> Vec<&'db FunctionBody<'db>> {
    get_all_checkable_functions(db, item)
        .iter()
        .filter_map(|function| db.function_body(*function).ok())
        .collect()
}

#[tracing::instrument(skip_all, level = "trace")]
pub fn get_all_function_bodies_with_ids<'db>(
    db: &'db dyn Database,
    item: &ModuleItemId<'db>,
) -> Vec<(FunctionWithBodyId<'db>, &'db FunctionBody<'db>)> {
    get_all_checkable_functions(db, item)
        .iter()
        .filter_map(|&id| {
            let body = db.function_body(id).ok()?;
            Some((id, body))
        })
        .collect()
}

#[tracing::instrument(skip_all, level = "trace")]
pub fn get_all_parenthesized_expressions<'db>(
    db: &'db dyn Database,
    item: &ModuleItemId<'db>,
) -> Vec<ExprParenthesized<'db>> {
    let node = match item {
        ModuleItemId::Constant(id) => id.stable_ptr(db).lookup(db).as_syntax_node(),
        ModuleItemId::FreeFunction(id) => id.stable_ptr(db).lookup(db).as_syntax_node(),
        ModuleItemId::Impl(id) => id.stable_ptr(db).lookup(db).as_syntax_node(),
        // Trait can have a default function impl.
        ModuleItemId::Trait(id) => id.stable_ptr(db).lookup(db).as_syntax_node(),
        _ => return vec![],
    };
    let function_nodes = node.descendants(db);

    function_nodes
        .filter(|node| node.kind(db) == SyntaxKind::ExprParenthesized)
        .map(|node| ExprParenthesized::from_syntax_node(db, node))
        .collect()
}

#[tracing::instrument(skip_all, level = "trace")]
pub fn get_all_match_expressions<'db>(
    function_body: &'db FunctionBody<'db>,
) -> Vec<ExprMatch<'db>> {
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

#[tracing::instrument(skip_all, level = "trace")]
pub fn get_all_loop_expressions<'db>(function_body: &'db FunctionBody<'db>) -> Vec<ExprLoop<'db>> {
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

#[tracing::instrument(skip_all, level = "trace")]
pub fn get_all_function_calls<'db>(
    function_body: &'db FunctionBody<'db>,
) -> impl Iterator<Item = ExprFunctionCall<'db>> {
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
}

#[tracing::instrument(skip_all, level = "trace")]
pub fn get_all_logical_operator_expressions<'db>(
    function_body: &'db FunctionBody<'db>,
) -> Vec<ExprLogicalOperator<'db>> {
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

#[tracing::instrument(skip_all, level = "trace")]
pub fn get_all_if_expressions<'db>(function_body: &'db FunctionBody<'db>) -> Vec<ExprIf<'db>> {
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

#[tracing::instrument(skip_all, level = "trace")]
pub fn get_all_conditions<'db>(function_body: &'db FunctionBody<'db>) -> Vec<Condition> {
    let if_expr_conditions = get_all_if_expressions(function_body)
        .into_iter()
        .flat_map(|if_expr| if_expr.conditions.clone());
    let while_expr_conditions = get_all_while_expressions(function_body)
        .into_iter()
        .map(|while_expr| while_expr.condition.clone());
    chain!(if_expr_conditions, while_expr_conditions,).collect()
}

#[tracing::instrument(skip_all, level = "trace")]
pub fn get_all_while_expressions<'db>(
    function_body: &'db FunctionBody<'db>,
) -> Vec<ExprWhile<'db>> {
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

#[tracing::instrument(skip_all, level = "trace")]
pub fn get_all_break_statements<'db>(
    function_body: &'db FunctionBody<'db>,
) -> Vec<StatementBreak<'db>> {
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
pub fn is_assert_macro_call<'db>(
    db: &'db dyn Database,
    arenas: &Arenas<'db>,
    expr: &ExprIf<'db>,
) -> bool {
    if_chain! {
        if let Expr::Block(ref if_block_expr) = arenas.exprs[expr.if_block];
        if let Statement::Let(ref if_block_let_stmt) = arenas.statements[if_block_expr.statements[0]];
        if let Pattern::Variable(ref if_block_let_stmt_pattern) = arenas.patterns[if_block_let_stmt.pattern];
        if if_block_let_stmt_pattern.name == ASSERT_FORMATTER_NAME;
        if if_block_let_stmt_pattern.var.ty.short_name(db) == ASSERT_PATH;
        then {
          return true;
        }
    }
    false
}

/// Gets rid of all the trivia (whitespaces, newlines etc.)
/// It makes predetermined token sequences easily comparable without counting in formatting caveats
pub fn syntax_node_to_str_without_all_nested_trivia<'db>(
    db: &'db dyn Database,
    syntax_node: SyntaxNode<'db>,
) -> String {
    syntax_node
        .tokens(db)
        .fold(String::new(), |mut acc, terminal| {
            acc.push_str(terminal.get_text_without_trivia(db));
            acc
        })
}
