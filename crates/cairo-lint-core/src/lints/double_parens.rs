use cairo_lang_defs::ids::ModuleItemId;
use cairo_lang_defs::plugin::PluginDiagnostic;
use cairo_lang_diagnostics::Severity;
use cairo_lang_semantic::db::SemanticGroup;
use cairo_lang_syntax::node::ast::Expr;
use cairo_lang_syntax::node::helpers::QueryAttrs;
use cairo_lang_syntax::node::{TypedStablePtr, TypedSyntaxNode};

use crate::queries::get_all_parenthesized_expressions;

pub const DOUBLE_PARENS: &str = "unnecessary double parentheses found. Consider removing them.";

pub const LINT_NAME: &str = "double_parens";

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
    let mut current_node = parens_expr.as_syntax_node();
    while let Some(node) = current_node.parent() {
        if node.has_attr_with_arg(db.upcast(), "allow", LINT_NAME) {
            return;
        }
        current_node = node;
    }
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
