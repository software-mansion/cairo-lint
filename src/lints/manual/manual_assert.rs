use crate::{corelib::CorelibContext, fixer::InternalFix, helper::indent_snippet};
use cairo_lang_defs::{ids::ModuleItemId, plugin::PluginDiagnostic};
use cairo_lang_diagnostics::Severity;
use cairo_lang_semantic::{Arenas, Expr, ExprBlock, ExprIf, Statement, db::SemanticGroup};
use cairo_lang_syntax::node::{
    SyntaxNode, TypedStablePtr, TypedSyntaxNode,
    ast::{
        BlockOrIf, Condition, ElseClause, Expr as AstExpr, ExprBlock as AstExprBlock,
        ExprIf as AstExprIf, OptionElseClause, Statement as AstStatement, WrappedTokenTree,
    },
    db::SyntaxGroup,
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

    fn fix<'db>(
        &self,
        db: &'db dyn SemanticGroup,
        node: SyntaxNode<'db>,
    ) -> Option<InternalFix<'db>> {
        fix_manual_assert(db, node)
    }

    fn fix_message(&self) -> Option<&'static str> {
        Some("Replace manual assert with `assert!` macro")
    }
}

#[tracing::instrument(skip_all, level = "trace")]
pub fn check_manual_assert<'db>(
    db: &'db dyn SemanticGroup,
    _corelib_context: &CorelibContext<'db>,
    item: &ModuleItemId<'db>,
    diagnostics: &mut Vec<PluginDiagnostic<'db>>,
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

fn check_single_manual_assert<'db>(
    db: &'db dyn SemanticGroup,
    if_expr: &ExprIf<'db>,
    arenas: &Arenas<'db>,
    diagnostics: &mut Vec<PluginDiagnostic<'db>>,
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

fn check_single_condition_block<'db>(
    db: &'db dyn SemanticGroup,
    condition_block_expr: &ExprBlock<'db>,
    if_expr: &ExprIf<'db>,
    arenas: &Arenas<'db>,
    diagnostics: &mut Vec<PluginDiagnostic<'db>>,
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
                inner_span: None
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
                inner_span: None
            });
        }
    }
}

#[tracing::instrument(skip_all, level = "trace")]
pub fn fix_manual_assert<'db>(
    db: &'db dyn SyntaxGroup,
    node: SyntaxNode<'db>,
) -> Option<InternalFix<'db>> {
    let if_expr = AstExprIf::from_syntax_node(db, node);
    let else_block_option = if_expr.else_clause(db);
    let is_else_if = is_else_if_expr(db, node);
    let prefix = if is_else_if { "{\n" } else { "" };
    let suffix = if is_else_if { "}\n" } else { "" };

    let indent = if is_else_if {
        // Extracting parent `if` node indentation.
        node.parent(db)
            .expect("Expected parent `else if` node")
            .parent(db)
            .expect("Expected parent `if` node")
            .get_text(db)
            .chars()
            .take_while(|c| c.is_whitespace())
            .count()
            + 4
    } else {
        node.get_text(db)
            .chars()
            .take_while(|c| c.is_whitespace())
            .count()
    };

    // TODO (wawel37): Handle `if let` case as the `matches!` macro will be implemented inside the corelib.
    let mut conditions = if_expr.conditions(db).elements(db);
    let condition = conditions.next()?;
    let Condition::Expr(condition_expr) = condition else {
        return None;
    };

    let condition = condition_expr.expr(db).as_syntax_node().get_text(db);
    let (if_block_panic_args, else_block_panic_args) = get_panic_args_from_diagnosed_node(db, node);
    let contrary_condition = format!("!({})", condition.trim());

    match (if_block_panic_args, else_block_panic_args) {
        (Some(panic_args), None) => {
            let assert_call = format!(
                "assert!({}, {});\n",
                contrary_condition,
                panic_args
                    .iter()
                    .map(|arg| {
                        let arg_text = arg.get_text(db).trim().to_string();
                        if arg_text == "," {
                            return format!("{arg_text} ");
                        }
                        arg_text
                    })
                    .join("")
            );
            if let OptionElseClause::ElseClause(else_clause) = else_block_option {
                // Else is just a block (not `else if`).
                if let BlockOrIf::Block(else_block) = else_clause.else_block_or_if(db) {
                    let else_statements = else_block.statements(db).as_syntax_node().get_text(db);
                    return Some(InternalFix {
                        node,
                        suggestion: format!(
                            "{prefix}{}",
                            indent_snippet(
                                &format!("{assert_call} {else_statements}{suffix}"),
                                indent / 4,
                            )
                        ),
                        description: ManualAssert.fix_message().unwrap().to_string(),
                        import_addition_paths: None,
                    });
                }

                // Else is an `else if` expression.
                if let BlockOrIf::If(else_if) = else_clause.else_block_or_if(db) {
                    return Some(InternalFix {
                        node,
                        suggestion: format!(
                            "{prefix}{}",
                            indent_snippet(
                                &format!(
                                    "{} {}{suffix}",
                                    assert_call,
                                    else_if.as_syntax_node().get_text(db)
                                ),
                                indent / 4,
                            )
                        ),
                        description: ManualAssert.fix_message().unwrap().to_string(),
                        import_addition_paths: None,
                    });
                }
            }
            // If there is no else block, just return the assert call.
            Some(InternalFix {
                node,
                suggestion: format!(
                    "{prefix}{}",
                    indent_snippet(&format!("{prefix}{assert_call}{suffix}"), indent / 4)
                ),
                description: ManualAssert.fix_message().unwrap().to_string(),
                import_addition_paths: None,
            })
        }
        (None, Some(panic_args)) => {
            let assert_call = format!(
                "assert!({}, {});\n",
                condition.trim(),
                panic_args
                    .iter()
                    .map(|arg| {
                        let arg_text = arg.get_text(db).trim().to_string();
                        if arg_text == "," {
                            return format!("{arg_text} ");
                        }
                        arg_text
                    })
                    .join("")
            );
            let if_statements = if_expr
                .if_block(db)
                .statements(db)
                .as_syntax_node()
                .get_text(db);
            Some(InternalFix {
                node,
                suggestion: format!(
                    "{prefix}{}",
                    indent_snippet(
                        &format!("{assert_call} {if_statements}{suffix}"),
                        indent / 4,
                    )
                ),
                description: ManualAssert.fix_message().unwrap().to_string(),
                import_addition_paths: None,
            })
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
fn get_panic_args_from_diagnosed_node<'db>(
    db: &'db dyn SyntaxGroup,
    node: SyntaxNode<'db>,
) -> (Option<Vec<SyntaxNode<'db>>>, Option<Vec<SyntaxNode<'db>>>) {
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

fn get_panic_args_from_block<'db>(
    db: &'db dyn SyntaxGroup,
    block: AstExprBlock<'db>,
) -> Option<Vec<SyntaxNode<'db>>> {
    let mut statements = block.statements(db).elements(db);
    let statement = statements
        .next()
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

    let args = match inline_macro.arguments(db).subtree(db) {
        WrappedTokenTree::Parenthesized(arg_list) => arg_list.tokens(db),
        WrappedTokenTree::Bracketed(arg_list) => arg_list.tokens(db),
        WrappedTokenTree::Braced(arg_list) => arg_list.tokens(db),
        WrappedTokenTree::Missing(_) => panic!("Expected arguments in the inline macro"),
    };

    Some(args.elements(db).map(|arg| arg.as_syntax_node()).collect())
}

// Checks if the given node is an `else if` expression.
fn is_else_if_expr<'db>(db: &'db dyn SyntaxGroup, node: SyntaxNode<'db>) -> bool {
    if_chain! {
        if let Some(else_clause) = node.parent_of_type::<ElseClause>(db);
        if let BlockOrIf::If(child_if) = else_clause.else_block_or_if(db);
        if child_if.as_syntax_node().long(db) == node.long(db);
        then {
            return true;
        }
    }
    false
}
