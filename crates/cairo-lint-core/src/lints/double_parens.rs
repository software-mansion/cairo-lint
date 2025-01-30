use cairo_lang_defs::ids::ModuleItemId;
use cairo_lang_defs::plugin::PluginDiagnostic;
use cairo_lang_diagnostics::Severity;
use cairo_lang_semantic::db::SemanticGroup;
use cairo_lang_syntax::node::ast::Expr;
use cairo_lang_syntax::node::db::SyntaxGroup;
use cairo_lang_syntax::node::{SyntaxNode, TypedStablePtr, TypedSyntaxNode};

use crate::context::{CairoLintKind, Lint};
use crate::helper::indent_snippet;
use crate::queries::get_all_parenthesized_expressions;

const DOUBLE_PARENS: &str = "unnecessary double parentheses found. Consider removing them.";
const DOUBLE_PARENS_LINT_NAME: &str = "double_parens";

pub struct DoubleParens;

impl Lint for DoubleParens {
    fn allowed_name(self: &Self) -> &'static str {
        DOUBLE_PARENS_LINT_NAME
    }

    fn diagnostic_message(self: &Self) -> &'static str {
        DOUBLE_PARENS
    }

    fn kind(self: &Self) -> CairoLintKind {
        CairoLintKind::DoubleParens
    }

    fn fix(self: &Self, db: &dyn SyntaxGroup, node: SyntaxNode) -> Option<(SyntaxNode, String)> {
        fix_double_parens(db, node)
    }
}

pub fn check_double_parens(
    db: &dyn SemanticGroup,
    item: &ModuleItemId,
    diagnostics: &mut Vec<PluginDiagnostic>,
) {
    let parenthesized_exprs = get_all_parenthesized_expressions(db, item);
    for parens_expr in parenthesized_exprs.iter() {
        check_single_double_parens(db, parens_expr, diagnostics);
    }
}

fn check_single_double_parens(
    db: &dyn SemanticGroup,
    parens_expr: &Expr,
    diagnostics: &mut Vec<PluginDiagnostic>,
) {
    let is_double_parens = if let Expr::Parenthesized(parenthesized_expr) = parens_expr {
        matches!(
            parenthesized_expr.expr(db.upcast()),
            Expr::Parenthesized(_) | Expr::Tuple(_)
        )
    } else {
        false
    };

    if is_double_parens {
        diagnostics.push(PluginDiagnostic {
            stable_ptr: parens_expr.stable_ptr().untyped(),
            message: DOUBLE_PARENS.to_string(),
            severity: Severity::Warning,
        });
    }
}

/// Removes unnecessary double parentheses from a syntax node.
///
/// Simplifies an expression by stripping extra layers of parentheses while preserving
/// the original formatting and indentation.
///
/// # Arguments
///
/// * `db` - Reference to the `SyntaxGroup` for syntax tree access.
/// * `node` - The `SyntaxNode` containing the expression.
///
/// # Returns
///
/// A `String` with the simplified expression.
///
/// # Example
///
/// Input: `((x + y))`
/// Output: `x + y`
pub fn fix_double_parens(db: &dyn SyntaxGroup, node: SyntaxNode) -> Option<(SyntaxNode, String)> {
    let mut expr = Expr::from_syntax_node(db, node.clone());

    while let Expr::Parenthesized(inner_expr) = expr {
        expr = inner_expr.expr(db);
    }

    Some((
        node.clone(),
        indent_snippet(
            &expr.as_syntax_node().get_text_without_trivia(db),
            node.get_text(db)
                .chars()
                .take_while(|c| c.is_whitespace())
                .collect::<String>()
                .len()
                / 4,
        ),
    ))
}
