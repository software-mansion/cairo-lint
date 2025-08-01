use cairo_lang_defs::ids::ModuleItemId;
use cairo_lang_defs::plugin::PluginDiagnostic;
use cairo_lang_diagnostics::Severity;
use cairo_lang_semantic::db::SemanticGroup;
use cairo_lang_semantic::{Arenas, Condition, Expr, ExprIf, Statement};
use cairo_lang_syntax::node::db::SyntaxGroup;
use cairo_lang_syntax::node::{
    SyntaxNode, TypedStablePtr, TypedSyntaxNode,
    ast::{Expr as AstExpr, ExprIf as AstExprIf, OptionElseClause, Statement as AstStatement},
};
use if_chain::if_chain;

use crate::context::{CairoLintKind, Lint};
use crate::corelib::CorelibContext;
use crate::fixer::InternalFix;
use crate::helper::indent_snippet;
use crate::queries::{get_all_function_bodies, get_all_if_expressions, is_assert_macro_call};

pub struct CollapsibleIf;

/// ## What it does
///
/// Checks for nested `if` statements that can be collapsed into a single `if` statement.
///
/// ## Example
///
/// ```cairo
/// fn main() {
///     let x = true;
///     let y = true;
///     let z = false;
///
///     if x || z {
///         if y && z {
///             println!("Hello");
///         }
///     }
/// }
/// ```
///
/// Can be collapsed to
///
/// ```cairo
/// fn main() {
///     let x = true;
///     let y = true;
///     let z = false;
///     if (x || z) && (y && z) {
///         println!("Hello");
///     }
/// }
/// ```
impl Lint for CollapsibleIf {
    fn allowed_name(&self) -> &'static str {
        "collapsible_if"
    }

    fn diagnostic_message(&self) -> &'static str {
        "Each `if`-statement adds one level of nesting, which makes code look more complex than it really is."
    }

    fn kind(&self) -> CairoLintKind {
        CairoLintKind::CollapsibleIf
    }

    fn has_fixer(&self) -> bool {
        true
    }

    fn fix<'db>(
        &self,
        db: &'db dyn SemanticGroup,
        node: SyntaxNode<'db>,
    ) -> Option<InternalFix<'db>> {
        fix_collapsible_if(db, node)
    }

    fn fix_message(&self) -> Option<&'static str> {
        Some("Combine nested ifs into a single condition")
    }
}

#[tracing::instrument(skip_all, level = "trace")]
pub fn check_collapsible_if<'db>(
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
            check_single_collapsible_if(db, if_expr, arenas, diagnostics);
        }
    }
}

fn check_single_collapsible_if<'db>(
    db: &'db dyn SemanticGroup,
    if_expr: &ExprIf<'db>,
    arenas: &Arenas<'db>,
    diagnostics: &mut Vec<PluginDiagnostic<'db>>,
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

        // Skip cases where the outer or inner `if` is an `if let`, as they aren't collapsible.
        if !matches!(if_expr.conditions.first(), Some(Condition::Let(..))) && !matches!(inner_if_expr.conditions.first(), Some(Condition::Let(..)));

        // We check whether the if inner `if` statement comes from an assert macro call.
        // If it does, we don't warn about collapsible ifs.
        if !is_assert_macro_call(db, arenas, inner_if_expr);

        // Check if any of the ifs (outer and inner) have an else block, if it's the case, don't return any diagnostics.
        if inner_if_expr.else_block.is_none() && if_expr.else_block.is_none();
        then {
            diagnostics.push(PluginDiagnostic {
                stable_ptr: if_expr.stable_ptr.untyped(),
                message: CollapsibleIf.diagnostic_message().to_string(),
                severity: Severity::Warning,
                inner_span: None
            });

            return;
        }
    }

    // Case where the outer if only has a tail.
    if_chain! {
        if if_block.statements.is_empty();
        if let Some(tail) = if_block.tail;

        // Check that the tail expression is a if
        if let Expr::If(ref inner_if_expr) = arenas.exprs[tail];

        // Skip cases where the outer or inner `if` is an `if let`, as they aren't collapsible.
        if !matches!(if_expr.conditions.first(), Some(Condition::Let(..))) && !matches!(inner_if_expr.conditions.first(), Some(Condition::Let(..)));

        // Check if any of the ifs (outer and inner) have an else block, if it's the case, don't return any diagnostics.
        if if_expr.else_block.is_none() && inner_if_expr.else_block.is_none();
        then {
            diagnostics.push(PluginDiagnostic {
                stable_ptr: if_expr.stable_ptr.untyped(),
                message: CollapsibleIf.diagnostic_message().to_string(),
                severity: Severity::Warning,
                inner_span: None,
            });
        }
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
#[tracing::instrument(skip_all, level = "trace")]
pub fn fix_collapsible_if<'db>(
    db: &'db dyn SyntaxGroup,
    node: SyntaxNode<'db>,
) -> Option<InternalFix<'db>> {
    let expr_if = AstExprIf::from_syntax_node(db, node);
    let outer_condition = expr_if.conditions(db).as_syntax_node().get_text(db);
    let if_block = expr_if.if_block(db);

    let mut statements = if_block.statements(db).elements(db);
    if statements.len() != 1 {
        return None;
    }

    if let Some(AstStatement::Expr(inner_expr_stmt)) = statements.next() {
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

            let inner_condition = inner_if_expr.conditions(db).as_syntax_node().get_text(db);
            let combined_condition = format!(
                "({}) && ({})",
                outer_condition.trim(),
                inner_condition.trim()
            );
            let inner_if_block = inner_if_expr.if_block(db).as_syntax_node().get_text(db);

            let indent = expr_if
                .if_kw(db)
                .as_syntax_node()
                .get_text(db)
                .chars()
                .take_while(|c| c.is_whitespace())
                .count();

            return Some(InternalFix {
                node,
                suggestion: indent_snippet(
                    &format!("if {combined_condition} {inner_if_block}"),
                    indent / 4,
                ),
                description: CollapsibleIf.fix_message().unwrap().to_string(),
                import_addition_paths: None,
            });
        }
    }
    None
}
