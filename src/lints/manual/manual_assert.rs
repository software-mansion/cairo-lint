use cairo_lang_defs::{ids::ModuleItemId, plugin::PluginDiagnostic};
use cairo_lang_diagnostics::Severity;
use cairo_lang_semantic::{db::SemanticGroup, Arenas, Expr, ExprIf, Statement};
use cairo_lang_syntax::node::{
    ast::{Condition, Expr as AstExpr, ExprIf as AstExprIf, Statement as AstStatement},
    db::SyntaxGroup,
    helpers::WrappedArgListHelper,
    SyntaxNode, TypedStablePtr, TypedSyntaxNode,
};
use if_chain::if_chain;

use crate::{
    context::{CairoLintKind, Lint},
    helper::is_panic_expr,
    queries::{get_all_function_bodies, get_all_if_expressions},
};

pub struct ManualAssert;

/// ## What it does
///
/// Checks for manual implementations of `assert` macro in an if expressions.
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

    // If there's an else block we ignore it.
    if if_expr.else_block.is_some() {
        return;
    };

    // if_chain! {
    //     if if_block.statements.len() == 1;
    //     if let Statement::Expr(ref inner_expr_stmt) = arenas.statements[if_block.statements[0]];
    //     if is_panic_expr(db, arenas, inner_expr_stmt.expr);
    //     then {
    //         println!("inner_expr_stmt: {:?}", inner_expr_stmt);
    //     }
    // }

    // Without tail.
    if_chain! {
        if if_block.statements.len() == 1;
        if let Statement::Expr(ref inner_expr_stmt) = arenas.statements[if_block.statements[0]];
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
        if if_block.statements.is_empty();
        if let Some(expr_id) = if_block.tail;
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

pub fn fix_manual_assert(db: &dyn SyntaxGroup, node: SyntaxNode) -> Option<(SyntaxNode, String)> {
    let if_expr = AstExprIf::from_syntax_node(db, node);
    let panic_args = get_panic_args_from_diagnosed_node(db, node);
    let condition = if_expr.condition(db);

    // TODO (wawel37): Handle `if let` case as the `matches!` macro will be implemented inside the corelib.
    let Condition::Expr(condition_expr) = condition else {
        return None;
    };

    // println!("condition: {}", condition.as_syntax_node().get_text(db));
    None
}

fn get_panic_args_from_diagnosed_node(
    db: &dyn SyntaxGroup,
    node: SyntaxNode,
) -> impl Iterator<Item = SyntaxNode> {
    let if_expr = AstExprIf::from_syntax_node(db, node);
    let if_block = if_expr.if_block(db);

    let statements = if_block.statements(db).elements(db);
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

    inline_macro
        .arguments(db)
        .arg_list(db)
        .expect("Expected arguments in the inline macro")
        .elements(db)
        .into_iter()
        .map(|arg| arg.as_syntax_node())
}
