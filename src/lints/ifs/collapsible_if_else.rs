use cairo_lang_defs::ids::ModuleItemId;
use cairo_lang_defs::plugin::PluginDiagnostic;
use cairo_lang_diagnostics::Severity;
use cairo_lang_semantic::{Arenas, Expr, ExprBlock, ExprIf, Statement};
use cairo_lang_syntax::node::db::SyntaxGroup;
use cairo_lang_syntax::node::{
    SyntaxNode, TypedStablePtr, TypedSyntaxNode,
    ast::{
        BlockOrIf, Expr as AstExpr, ExprIf as AstExprIf, OptionElseClause,
        Statement as AstStatement,
    },
};
use if_chain::if_chain;

use crate::LinterGroup;
use crate::context::{CairoLintKind, Lint};
use crate::fixer::InternalFix;
use crate::queries::{get_all_function_bodies, get_all_if_expressions};

pub struct CollapsibleIfElse;

/// ## What it does
///
/// Checks for nested `if` statements inside the `else` statement
/// that can be collapsed into a single `if-else` statement.
///
/// ## Example
///
/// ```cairo
/// fn main() {
///     let x = true;
///     if x {
///         println!("x is true");
///     } else {
///         if !x {
///             println!("x is false");
///         }
///     }
/// }
/// ```
///
/// Can be refactored to:
///
/// ```cairo
/// fn main() {
///     let x = true;
///     if x {
///         println!("x is true");
///     } else if !x {
///         println!("x is false");
///     }
/// }
/// ```
impl Lint for CollapsibleIfElse {
    fn allowed_name(&self) -> &'static str {
        "collapsible_if_else"
    }

    fn diagnostic_message(&self) -> &'static str {
        "Consider using else if instead of else { if ... }"
    }

    fn kind(&self) -> CairoLintKind {
        CairoLintKind::CollapsibleIfElse
    }

    fn has_fixer(&self) -> bool {
        true
    }

    fn fix(&self, db: &dyn LinterGroup, node: SyntaxNode) -> Option<InternalFix> {
        fix_collapsible_if_else(db.upcast(), node)
    }

    fn fix_message(&self) -> Option<&'static str> {
        Some("Use else-if instead of nested if")
    }
}

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
#[tracing::instrument(skip_all, level = "trace")]
pub fn check_collapsible_if_else(
    db: &dyn LinterGroup,
    item: &ModuleItemId,
    diagnostics: &mut Vec<PluginDiagnostic>,
) {
    let function_bodies = get_all_function_bodies(db, item);
    for function_body in function_bodies.iter() {
        let if_exprs = get_all_if_expressions(function_body);
        let arenas = &function_body.arenas;
        for if_expr in if_exprs.iter() {
            check_single_collapsible_if_else(if_expr, arenas, diagnostics);
        }
    }
}

fn check_single_collapsible_if_else(
    if_expr: &ExprIf,
    arenas: &Arenas,
    diagnostics: &mut Vec<PluginDiagnostic>,
) {
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
            message: CollapsibleIfElse.diagnostic_message().to_string(),
            severity: Severity::Warning,
            inner_span: None,
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

/// Transforms nested `if-else` statements into a more compact `if-else if` format.
///
/// Simplifies an expression by converting nested `if-else` structures into a single `if-else
/// if` statement while preserving the original formatting and indentation.
///
/// # Arguments
///
/// * `db` - Reference to the `SyntaxGroup` for syntax tree access.
/// * `node` - The `SyntaxNode` containing the expression.
///
/// # Returns
///
/// A `String` with the refactored `if-else` structure.
#[tracing::instrument(skip_all, level = "trace")]
pub fn fix_collapsible_if_else(db: &dyn SyntaxGroup, node: SyntaxNode) -> Option<InternalFix> {
    let if_expr = AstExprIf::from_syntax_node(db, node);
    let OptionElseClause::ElseClause(else_clause) = if_expr.else_clause(db) else {
        return None;
    };
    if let BlockOrIf::Block(block_expr) = else_clause.else_block_or_if(db)
        && let Some(AstStatement::Expr(statement_expr)) =
            block_expr.statements(db).elements(db).next()
        && let AstExpr::If(if_expr) = statement_expr.expr(db)
    {
        // Construct the new "else if" expression
        let condition = if_expr.conditions(db).as_syntax_node().get_text(db);
        let if_body = if_expr.if_block(db).as_syntax_node().get_text(db);
        let else_body = if_expr.else_clause(db).as_syntax_node().get_text(db);

        // Preserve original indentation
        let original_indent = else_clause
            .as_syntax_node()
            .get_text(db)
            .chars()
            .take_while(|c| c.is_whitespace())
            .collect::<String>();

        return Some(InternalFix {
            node: else_clause.as_syntax_node(),
            suggestion: format!("{original_indent}else if {condition} {if_body} {else_body}"),
            description: CollapsibleIfElse.fix_message().unwrap().to_string(),
            import_addition_paths: None,
        });
    }

    // If we can't transform it, return the original text
    None
}
