use cairo_lang_defs::ids::ModuleItemId;
use cairo_lang_defs::plugin::PluginDiagnostic;
use cairo_lang_diagnostics::Severity;
use cairo_lang_semantic::db::SemanticGroup;
use cairo_lang_semantic::{Arenas, Expr, ExprId, ExprLoop, Statement};
use cairo_lang_syntax::node::db::SyntaxGroup;
use cairo_lang_syntax::node::{
    ast::{Expr as AstExpr, ExprLoop as AstExprLoop, OptionElseClause, Statement as AstStatement},
    SyntaxNode, TypedStablePtr, TypedSyntaxNode,
};
use if_chain::if_chain;

use crate::context::{CairoLintKind, Lint};
use crate::helper::{invert_condition, remove_break_from_block, remove_break_from_else_clause};
use crate::queries::{get_all_function_bodies, get_all_loop_expressions};

pub struct LoopForWhile;

/// ## What it does
///
/// Checks for `loop` expressions that contain a conditional `if` statement with break inside that
/// can be simplified to a `while` loop.
///
/// ## Example
///
/// ```cairo
/// fn main() {
///     let mut x: u16 = 0;
///     loop {
///         if x == 10 {
///             break;
///         }
///         x += 1;
///     }
/// }
/// ```
///
/// Can be simplified to:
///
/// ```cairo
/// fn main() {
///     let mut x: u16 = 0;
///     while x != 10 {
///         x += 1;
///     }
/// }
/// ```
impl Lint for LoopForWhile {
    fn allowed_name(&self) -> &'static str {
        "loop_for_while"
    }

    fn diagnostic_message(&self) -> &'static str {
        "you seem to be trying to use `loop`. Consider replacing this `loop` with a `while` \
                                  loop for clarity and conciseness"
    }

    fn kind(&self) -> CairoLintKind {
        CairoLintKind::LoopForWhile
    }

    fn has_fixer(&self) -> bool {
        true
    }

    fn fix(&self, db: &dyn SemanticGroup, node: SyntaxNode) -> Option<(SyntaxNode, String)> {
        fix_loop_break(db.upcast(), node)
    }
}

/// Checks for
/// ```ignore
/// loop {
///     ...
///     if cond {
///         break;
///     }
/// }
/// ```
/// Which can be rewritten as a while loop
/// ```ignore
/// while cond {
///     ...
/// }
/// ```
pub fn check_loop_for_while(
    db: &dyn SemanticGroup,
    item: &ModuleItemId,
    diagnostics: &mut Vec<PluginDiagnostic>,
) {
    let function_bodies = get_all_function_bodies(db, item);
    for function_body in function_bodies.iter() {
        let loop_exprs = get_all_loop_expressions(function_body);
        let arenas = &function_body.arenas;
        for loop_expr in loop_exprs.iter() {
            check_single_loop_for_while(loop_expr, arenas, diagnostics);
        }
    }
}

fn check_single_loop_for_while(
    loop_expr: &ExprLoop,
    arenas: &Arenas,
    diagnostics: &mut Vec<PluginDiagnostic>,
) {
    // Get the else block  expression
    let Expr::Block(block_expr) = &arenas.exprs[loop_expr.body] else {
        return;
    };

    // Checks if the first statement is an if expression that only contains a break instruction.
    if_chain! {
        if let Some(statement) = block_expr.statements.first();
        if let Statement::Expr(ref expr_statement) = arenas.statements[*statement];
        if check_if_contains_break_with_no_return_value(&expr_statement.expr, arenas);
        then {
            diagnostics.push(PluginDiagnostic {
                stable_ptr: loop_expr.stable_ptr.untyped(),
                message: LoopForWhile.diagnostic_message().to_string(),
                severity: Severity::Warning,
                relative_span: None,
                inner_span: None
            });
        }
    }

    // Do the same thing if the if is in the tail of the block
    if_chain! {
        // Loop with single if-else statement will only have a tail expr and the statements will be empty.
        // We check if the tail if statement is a single one. If it's not, we ignore the loop as a whole.
        if block_expr.statements.is_empty();
        if let Some(tail_expr) = block_expr.tail;
        if check_if_contains_break_with_no_return_value(&tail_expr, arenas);
        then {
            diagnostics.push(PluginDiagnostic {
                stable_ptr: loop_expr.stable_ptr.untyped(),
                message: LoopForWhile.diagnostic_message().to_string(),
                severity: Severity::Warning,
                relative_span: None,
                inner_span: None
            });
        }
    }
}

fn check_if_contains_break_with_no_return_value(expr: &ExprId, arenas: &Arenas) -> bool {
    if_chain! {
        // Is an if expression
        if let Expr::If(ref if_expr) = arenas.exprs[*expr];
        // Get the block
        if let Expr::Block(ref if_block) = arenas.exprs[if_expr.if_block];
        // Get the first statement of the if
        if let Some(inner_stmt) = if_block.statements.first();
        // Is it a break statement
        if let Statement::Break(break_expr) = &arenas.statements[*inner_stmt];
        then {
            // If break also has a return value like `break 1;` then it's not a simple break.
            return break_expr.expr_option.is_none();
        }
    }
    false
}

/// Converts a `loop` with a conditionally-breaking `if` statement into a `while` loop.
///
/// This function transforms loops that have a conditional `if` statement
/// followed by a `break` into a `while` loop, which can simplify the logic
/// and improve readability.
///
/// # Arguments
///
/// * `db` - Reference to the `SyntaxGroup` for syntax tree access.
/// * `node` - The `SyntaxNode` representing the loop expression.
///
/// # Returns
///
/// A `String` containing the transformed loop as a `while` loop, preserving
/// the original formatting and indentation.
///
/// # Example
///
/// ```
/// let mut x = 0;
/// loop {
///     if x > 5 {
///         break;
///     }
///     x += 1;
/// }
/// ```
///
/// Would be converted to:
///
/// ```
/// let mut x = 0;
/// while x <= 5 {
///     x += 1;
/// }
/// ```
pub fn fix_loop_break(db: &dyn SyntaxGroup, node: SyntaxNode) -> Option<(SyntaxNode, String)> {
    let loop_expr = AstExprLoop::from_syntax_node(db, node);
    let indent = node
        .get_text(db)
        .chars()
        .take_while(|c| c.is_whitespace())
        .collect::<String>();
    let mut condition_text = String::new();
    let mut loop_body = String::new();

    let mut loop_span = node.span(db);
    loop_span.end = node.span_start_without_trivia(db);
    let trivia = node.get_text_of_span(db, loop_span).trim().to_string();
    let trivia = if trivia.is_empty() {
        trivia
    } else {
        format!("{indent}{trivia}\n")
    };

    if let Some(AstStatement::Expr(expr_statement)) =
        loop_expr.body(db).statements(db).elements(db).first()
    {
        if let AstExpr::If(if_expr) = expr_statement.expr(db) {
            condition_text = invert_condition(
                &if_expr
                    .condition(db)
                    .as_syntax_node()
                    .get_text_without_trivia(db),
            );

            loop_body.push_str(&remove_break_from_block(db, if_expr.if_block(db), &indent));

            if let OptionElseClause::ElseClause(else_clause) = if_expr.else_clause(db) {
                loop_body.push_str(&remove_break_from_else_clause(db, else_clause, &indent));
            }
        }
    }

    for statement in loop_expr
        .body(db)
        .statements(db)
        .elements(db)
        .iter()
        .skip(1)
    {
        loop_body.push_str(&format!(
            "{}    {}\n",
            indent,
            statement.as_syntax_node().get_text(db).trim()
        ));
    }

    Some((
        node,
        format!(
            "{trivia}{}while {} {{\n{}{}}}\n",
            indent, condition_text, loop_body, indent
        ),
    ))
}
