use cairo_lang_defs::ids::ModuleItemId;
use cairo_lang_defs::plugin::PluginDiagnostic;
use cairo_lang_diagnostics::Severity;
use cairo_lang_semantic::db::SemanticGroup;
use cairo_lang_semantic::{Arenas, Expr, ExprIf, Statement};
use cairo_lang_syntax::node::db::SyntaxGroup;
use cairo_lang_syntax::node::{
    ast::{Expr as AstExpr, ExprIf as AstExprIf, OptionElseClause, Statement as AstStatement},
    SyntaxNode, TypedStablePtr, TypedSyntaxNode,
};
use if_chain::if_chain;

use crate::context::{CairoLintKind, Lint};
use crate::helper::indent_snippet;
use crate::queries::{get_all_function_bodies, get_all_if_expressions};

const COLLAPSIBLE_IF: &str =
    "Each `if`-statement adds one level of nesting, which makes code look more complex than it really is.";
const COLLAPSIBLE_IF_LINT_NAME: &str = "collapsible_if";

pub struct CollapsibleIf;

impl Lint for CollapsibleIf {
    fn allowed_name(self: &Self) -> &'static str {
        COLLAPSIBLE_IF_LINT_NAME
    }

    fn diagnostic_message(self: &Self) -> &'static str {
        COLLAPSIBLE_IF
    }

    fn kind(self: &Self) -> CairoLintKind {
        CairoLintKind::CollapsibleIf
    }

    fn has_fixer(self: &Self) -> bool {
        true
    }

    fn fix(self: &Self, db: &dyn SyntaxGroup, node: SyntaxNode) -> Option<(SyntaxNode, String)> {
        fix_collapsible_if(db, node)
    }
}

/// Checks for
/// ```ignore
/// if cond {
///     if second_cond {
///         ...
///     }
/// }
/// ```
/// This can be collapsed to
/// ```ignore
/// if cond && second_cond {
///     ...
/// }
/// ```
pub fn check_collapsible_if(
    db: &dyn SemanticGroup,
    item: &ModuleItemId,
    diagnostics: &mut Vec<PluginDiagnostic>,
) {
    let function_bodies = get_all_function_bodies(db, item);
    for function_body in function_bodies.iter() {
        let if_exprs = get_all_if_expressions(function_body);
        let arenas = &function_body.arenas;
        for if_expr in if_exprs.iter() {
            check_single_collapsible_if(db, if_expr, arenas, diagnostics);
        }
    }
}

fn check_single_collapsible_if(
    _db: &dyn SemanticGroup,
    if_expr: &ExprIf,
    arenas: &Arenas,
    diagnostics: &mut Vec<PluginDiagnostic>,
) {
    let Expr::Block(ref if_block) = arenas.exprs[if_expr.if_block] else {
        return;
    };

    // TODO: Check if if block can contain only 1 statement without tail
    // Case where the if block only contains a statement and no tail
    if_chain! {
        if if_block.statements.len() == 1;
        if if_block.tail.is_none();
        // If the inner statement is an expression
        if let Statement::Expr(ref inner_expr_stmt) = arenas.statements[if_block.statements[0]];
        // And this expression is an if expression
        if let Expr::If(ref inner_if_expr) = arenas.exprs[inner_expr_stmt.expr];
        then {
            // Check if any of the ifs (outter and inner) have an else block, if it's the case don't diagnostic
            if inner_if_expr.else_block.is_some() || if_expr.else_block.is_some() {
                return;
            }

            diagnostics.push(PluginDiagnostic {
                stable_ptr: if_expr.stable_ptr.untyped(),
                message: COLLAPSIBLE_IF.to_string(),
                severity: Severity::Warning,
            });
            return;
        }
    }

    // Case where the outter if only has a tail.
    if if_block.tail.is_some_and(|tail| {
        // Check that the tail expression is a if
        let Expr::If(ref inner_if_expr) = arenas.exprs[tail] else {
            return false;
        };
        // Check if any of the ifs (outter and inner) have an else block, if it's the case don't diagnostic
        if_expr.else_block.is_none() && inner_if_expr.else_block.is_none()
    }) && if_block.statements.is_empty()
    {
        diagnostics.push(PluginDiagnostic {
            stable_ptr: if_expr.stable_ptr.untyped(),
            message: COLLAPSIBLE_IF.to_string(),
            severity: Severity::Warning,
        });
    }
}

/// Attempts to fix a collapsible if-statement by combining its conditions.
/// This function detects nested `if` statements where the inner `if` can be collapsed
/// into the outer one by combining their conditions with `&&`. It reconstructs the
/// combined condition and the inner block, preserving the indentation and formatting.
///
/// # Arguments
///
/// * `db` - A reference to the `SyntaxGroup`, which provides access to the syntax tree.
/// * `node` - A `SyntaxNode` representing the outer `if` statement that might be collapsible.
///
/// # Returns
///
/// A `String` containing the fixed code with the combined conditions if a collapsible
/// `if` is found. If no collapsible `if` is detected, the original text of the node is
/// returned.
pub fn fix_collapsible_if(db: &dyn SyntaxGroup, node: SyntaxNode) -> Option<(SyntaxNode, String)> {
    let expr_if = AstExprIf::from_syntax_node(db, node.clone());
    let outer_condition = expr_if
        .condition(db)
        .as_syntax_node()
        .get_text_without_trivia(db);
    let if_block = expr_if.if_block(db);

    let statements = if_block.statements(db).elements(db);
    if statements.len() != 1 {
        return None;
    }

    if let Some(AstStatement::Expr(inner_expr_stmt)) = statements.first() {
        if let AstExpr::If(inner_if_expr) = inner_expr_stmt.expr(db) {
            match inner_if_expr.else_clause(db) {
                OptionElseClause::Empty(_) => {}
                OptionElseClause::ElseClause(_) => {
                    return None;
                }
            }

            match expr_if.else_clause(db) {
                OptionElseClause::Empty(_) => {}
                OptionElseClause::ElseClause(_) => {
                    return None;
                }
            }

            let inner_condition = inner_if_expr
                .condition(db)
                .as_syntax_node()
                .get_text_without_trivia(db);
            let combined_condition = format!("({}) && ({})", outer_condition, inner_condition);
            let inner_if_block = inner_if_expr.if_block(db).as_syntax_node().get_text(db);

            let indent = expr_if
                .if_kw(db)
                .as_syntax_node()
                .get_text(db)
                .chars()
                .take_while(|c| c.is_whitespace())
                .count();
            return Some((
                node,
                indent_snippet(
                    &format!("if {} {}", combined_condition, inner_if_block,),
                    indent / 4,
                ),
            ));
        }
    }
    None
}
