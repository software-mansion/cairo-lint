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

pub struct DoubleParens;

/// ## What it does
///
/// Checks for unnecessary double parentheses in expressions.
///
/// ## Example
///
/// ```cairo
/// fn main() -> u32 {
///     ((0))
/// }
/// ```
///
/// Can be simplified to:
///
/// ```cairo
/// fn main() -> u32 {
///     0
/// }
/// ```
impl Lint for DoubleParens {
    fn allowed_name(&self) -> &'static str {
        "double_parens"
    }

    fn diagnostic_message(&self) -> &'static str {
        "unnecessary double parentheses found. Consider removing them."
    }

    fn kind(&self) -> CairoLintKind {
        CairoLintKind::DoubleParens
    }

    fn has_fixer(&self) -> bool {
        true
    }

    fn fix(&self, db: &dyn SemanticGroup, node: SyntaxNode) -> Option<(SyntaxNode, String)> {
        fix_double_parens(db.upcast(), node)
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
            message: DoubleParens.diagnostic_message().to_string(),
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
            &expr.as_syntax_node().get_text(db),
            node.get_text(db)
                .chars()
                .take_while(|c| c.is_whitespace())
                .collect::<String>()
                .len()
                / 4,
        ),
    ))
}
