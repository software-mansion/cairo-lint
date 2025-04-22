use crate::helper::indent_snippet;
use cairo_lang_defs::{ids::ModuleItemId, plugin::PluginDiagnostic};
use cairo_lang_diagnostics::Severity;
use cairo_lang_semantic::{db::SemanticGroup, Arenas, Expr, ExprBlock, ExprIf, Statement};
use cairo_lang_syntax::node::{
    ast::{
        BlockOrIf, Condition, Expr as AstExpr, ExprBlock as AstExprBlock, ExprIf as AstExprIf,
        OptionElseClause, Statement as AstStatement,
    },
    db::SyntaxGroup,
    helpers::WrappedArgListHelper,
    SyntaxNode, TypedStablePtr, TypedSyntaxNode,
};
use if_chain::if_chain;
use itertools::Itertools;

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

    fn has_fixer(&self) -> bool {
        true
    }

    fn fix(&self, db: &dyn SemanticGroup, node: SyntaxNode) -> Option<(SyntaxNode, String)> {
        fix_manual_assert(db.upcast(), node)
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
                relative_span: None
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
                relative_span: None
            });
        }
    }
}

pub fn fix_manual_assert(db: &dyn SyntaxGroup, node: SyntaxNode) -> Option<(SyntaxNode, String)> {
    let if_expr = AstExprIf::from_syntax_node(db, node);
    let else_block_option = if_expr.else_clause(db);

    // TODO (wawel37): Handle `if let` case as the `matches!` macro will be implemented inside the corelib.
    let Condition::Expr(condition_expr) = if_expr.condition(db) else {
        return None;
    };

    let condition = condition_expr.expr(db).as_syntax_node().get_text(db);
    let (if_block_panic_args, else_block_panic_args) = get_panic_args_from_diagnosed_node(db, node);
    let contrary_condition = format!("!({})", condition.trim());
    let indent = if_expr
        .if_kw(db)
        .as_syntax_node()
        .get_text(db)
        .chars()
        .take_while(|c| c.is_whitespace())
        .count();

    match (if_block_panic_args, else_block_panic_args) {
        (Some(panic_args), None) => {
            let assert_call = format!(
                "assert!({}, {});\n",
                contrary_condition,
                panic_args
                    .map(|arg| arg.get_text(db).trim().to_string())
                    .join(", ")
            );
            if_chain! {
                if let OptionElseClause::ElseClause(else_clause) = else_block_option;
                if let BlockOrIf::Block(else_block) = else_clause.else_block_or_if(db);
                then {
                  let else_statements = else_block.statements(db).as_syntax_node().get_text(db);
                  return Some((node, indent_snippet(&format!("{} {}", assert_call, else_statements), indent / 4)));
                }
            }
            Some((node, indent_snippet(&assert_call, indent / 4)))
        }
        (None, Some(panic_args)) => {
            let assert_call = format!(
                "assert!({}, {});\n",
                condition.trim(),
                panic_args
                    .map(|arg| arg.get_text(db).trim().to_string())
                    .join(", ")
            );
            let if_statements = if_expr
                .if_block(db)
                .statements(db)
                .as_syntax_node()
                .get_text(db);
            Some((
                node,
                indent_snippet(&format!("{} {}", assert_call, if_statements), indent / 4),
            ))
        }
        (None, None) => {
            panic!("Expected at least one panic argument in the if or else block");
        }
        (Some(_), Some(_)) => None,
    }
}

// Function that returns a tuple where:
// - The first element is an iterator over the panic arguments from the `if` block.
// - The second element is an iterator over the panic arguments from the `else` block.
fn get_panic_args_from_diagnosed_node(
    db: &dyn SyntaxGroup,
    node: SyntaxNode,
) -> (
    Option<impl Iterator<Item = SyntaxNode>>,
    Option<impl Iterator<Item = SyntaxNode>>,
) {
    let if_expr = AstExprIf::from_syntax_node(db, node);
    let if_block = if_expr.if_block(db);
    let else_block_option = if_expr.else_clause(db);

    if_chain! {
        if let OptionElseClause::ElseClause(else_clause) = else_block_option;
        if let BlockOrIf::Block(else_block) = else_clause.else_block_or_if(db);
        then {
            let if_block_panic_args = get_panic_args_from_block(db, if_block);
            let else_block_panic_args = get_panic_args_from_block(db, else_block);
            return (if_block_panic_args, else_block_panic_args)
        }
    }
    (get_panic_args_from_block(db, if_block), None)
}

fn get_panic_args_from_block(
    db: &dyn SyntaxGroup,
    block: AstExprBlock,
) -> Option<impl Iterator<Item = SyntaxNode>> {
    let statements = block.statements(db).elements(db);
    let statement = statements
        .first()
        .expect("Expected at least one statement in the if block");

    let expr = match statement {
        AstStatement::Expr(expr) => expr,
        _ => panic!("Expected the statement to be an expression"),
    };

    let inline_macro = match expr.expr(db) {
        AstExpr::InlineMacro(inline_macro) => inline_macro,
        _ => panic!("Expected the expression to be an inline macro"),
    };

    if inline_macro.path(db).as_syntax_node().get_text(db).trim() != "panic" {
        return None;
    }

    Some(
        inline_macro
            .arguments(db)
            .arg_list(db)
            .expect("Expected arguments in the inline macro")
            .elements(db)
            .into_iter()
            .map(|arg| arg.as_syntax_node()),
    )
}
