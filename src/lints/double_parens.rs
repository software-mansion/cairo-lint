use crate::context::{CairoLintKind, Lint};
use crate::fixes::InternalFix;
use crate::helper::indent_snippet;
use crate::queries::get_all_parenthesized_expressions;
use cairo_lang_defs::ids::ModuleItemId;
use cairo_lang_defs::plugin::PluginDiagnostic;
use cairo_lang_diagnostics::Severity;
use cairo_lang_semantic::db::SemanticGroup;
use cairo_lang_syntax::node::ast::Expr;
use cairo_lang_syntax::node::db::SyntaxGroup;
use cairo_lang_syntax::node::kind::SyntaxKind;
use cairo_lang_syntax::node::{SyntaxNode, TypedStablePtr, TypedSyntaxNode};

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

    fn fix(&self, db: &dyn SemanticGroup, node: SyntaxNode) -> Option<InternalFix> {
        fix_double_parens(db.upcast(), node)
    }

    fn fix_message(&self) -> Option<&'static str> {
        Some("Remove nested parentheses")
    }
}

#[tracing::instrument(skip_all, level = "trace")]
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
            stable_ptr: parens_expr.stable_ptr(db.upcast()).untyped(),
            message: DoubleParens.diagnostic_message().to_string(),
            severity: Severity::Warning,
            inner_span: None,
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
#[tracing::instrument(skip_all, level = "trace")]
pub fn fix_double_parens(db: &dyn SyntaxGroup, node: SyntaxNode) -> Option<InternalFix> {
    let mut expr = Expr::from_syntax_node(db, node);

    // When the parent expression is binary or unary, we may want to keep the last parenthesis,
    // as it can affect the meaning of the expression.
    let leave_last_parens = node.parent(db).is_some_and(|parent| {
        matches!(
            parent.kind(db),
            SyntaxKind::ExprBinary | SyntaxKind::ExprUnary
        )
    });

    while let Expr::Parenthesized(inner_expr) = &expr {
        let sub_expr = inner_expr.expr(db);

        expr = match sub_expr {
            // Preserve parentheses if the next expression is a binary operation and they might be needed.
            Expr::Binary(_) if leave_last_parens => break,

            // In all other cases, when the next expression is not binary
            // and the parent node does not indicate parentheses are needed,
            // we can proceed without them.
            _ => sub_expr,
        };
    }

    let indented_snippet = indent_snippet(
        &expr.as_syntax_node().get_text(db),
        node.get_text(db)
            .chars()
            .take_while(|c| c.is_whitespace())
            .collect::<String>()
            .len()
            / 4,
    );

    let end_whitespaces = node
        .get_text(db)
        .chars()
        .rev()
        .take_while(|c| c.is_whitespace())
        .collect::<String>();

    Some(InternalFix {
        node,
        suggestion: format!("{indented_snippet}{end_whitespaces}"),
        description: DoubleParens.fix_message().unwrap().to_string(),
        import_addition_paths: None,
    })
}
