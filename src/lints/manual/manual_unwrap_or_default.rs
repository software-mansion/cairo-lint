use cairo_lang_defs::ids::ModuleItemId;
use cairo_lang_defs::plugin::PluginDiagnostic;
use cairo_lang_diagnostics::Severity;
use cairo_lang_syntax::node::{
    SyntaxNode, TypedStablePtr, TypedSyntaxNode,
    ast::{self},
};

use crate::{
    context::CairoLintKind,
    fixer::InternalFix,
    queries::{get_all_function_bodies, get_all_if_expressions, get_all_match_expressions},
};
use crate::{
    context::Lint,
    lints::manual::{ManualLint, check_manual, check_manual_if},
};
use salsa::Database;

pub struct ManualUnwrapOrDefault;

/// ## What it does
///
/// Checks for manual unwrapping of an Option or Result.
///
/// ## Example
///
/// ```cairo
/// fn main() {
///     let x: Option<u128> = Option::Some(1038);
///     if let Option::Some(v) = x {
///         v
///     } else {
///         0
///     };
/// }
/// ```
///
/// Can be simplified to:
///
/// ```cairo
/// fn main() {
///     let x: Option<u128> = Option::Some(1038);
///     x.unwrap_or_default();
/// }
/// ```
impl Lint for ManualUnwrapOrDefault {
    fn allowed_name(&self) -> &'static str {
        "manual_unwrap_or_default"
    }

    fn diagnostic_message(&self) -> &'static str {
        "This can be done in one call with `.unwrap_or_default()`"
    }

    fn kind(&self) -> CairoLintKind {
        CairoLintKind::ManualUnwrapOrDefault
    }

    fn has_fixer(&self) -> bool {
        true
    }

    fn fix<'db>(&self, db: &'db dyn Database, node: SyntaxNode<'db>) -> Option<InternalFix<'db>> {
        fix_manual_unwrap_or_default(db, node)
    }

    fn fix_message(&self) -> Option<&'static str> {
        Some("Use `unwrap_or_default()` instead of manual pattern")
    }
}

#[tracing::instrument(skip_all, level = "trace")]
pub fn check_manual_unwrap_or_default<'db>(
    db: &'db dyn Database,
    item: &ModuleItemId<'db>,
    diagnostics: &mut Vec<PluginDiagnostic<'db>>,
) {
    let function_bodies = get_all_function_bodies(db, item);
    for function_body in function_bodies.iter() {
        let if_exprs = get_all_if_expressions(function_body);
        let match_exprs = get_all_match_expressions(function_body);
        let arenas = &function_body.arenas;

        for match_expr in match_exprs.iter() {
            if check_manual(db, match_expr, arenas, ManualLint::ManualUnwrapOrDefault) {
                diagnostics.push(PluginDiagnostic {
                    stable_ptr: match_expr.stable_ptr.untyped(),
                    message: ManualUnwrapOrDefault.diagnostic_message().to_owned(),
                    severity: Severity::Warning,
                    inner_span: None,
                });
            }
        }

        for if_expr in if_exprs.iter() {
            if check_manual_if(db, if_expr, arenas, ManualLint::ManualUnwrapOrDefault) {
                diagnostics.push(PluginDiagnostic {
                    stable_ptr: if_expr.stable_ptr.untyped(),
                    message: ManualUnwrapOrDefault.diagnostic_message().to_owned(),
                    severity: Severity::Warning,
                    inner_span: None,
                });
            }
        }
    }
}

#[tracing::instrument(skip_all, level = "trace")]
pub fn fix_manual_unwrap_or_default<'db>(
    db: &'db dyn Database,
    node: SyntaxNode<'db>,
) -> Option<InternalFix<'db>> {
    let expr = ast::Expr::from_syntax_node(db, node);

    let matched_expr = match &expr {
        ast::Expr::Match(expr_match) => expr_match.expr(db).as_syntax_node(),
        ast::Expr::If(expr_if) => {
            let mut conditions = expr_if.conditions(db).elements(db);
            let condition = conditions
                .next()
                .expect("Expected at least one condition in `if` expression.");
            match condition {
                ast::Condition::Let(condition_let) => condition_let.expr(db).as_syntax_node(),
                _ => panic!("Expected an `if let` expression."),
            }
        }
        _ => panic!("The expression is expected to be either a `match` or an `if` statement."),
    };

    // If the expression is part of a `let` statement (e.g., `let x = match a { ... }`),
    // we need to take the parent node to capture the entire statement, not just the `match` expression.
    let (expression, target_node) = if let ast::Statement::Let(parent_node) =
        ast::Statement::from_syntax_node(db, node.parent(db).unwrap())
    {
        let mut expr = parent_node
            .as_syntax_node()
            .get_text_without_trivia(db)
            .replace(
                parent_node.rhs(db).as_syntax_node().get_text(db),
                &format!("{}.unwrap_or_default()", matched_expr.get_text(db).trim()),
            );

        // Since `get_text_without_trivia` removes trailing whitespace and newlines,
        // we explicitly add a newline to ensure expression maintains correct formatting.
        expr.push('\n');

        (expr, parent_node.as_syntax_node())
    } else {
        (
            format!("{}.unwrap_or_default()", matched_expr.get_text(db).trim()),
            node,
        )
    };

    let indent = target_node
        .get_text(db)
        .chars()
        .take_while(|c| c.is_whitespace())
        .collect::<String>();

    let comments = extract_comments(db, target_node, &indent);

    Some(InternalFix {
        node: target_node,
        suggestion: format!("{comments}{indent}{expression}"),
        description: ManualUnwrapOrDefault.fix_message().unwrap().to_string(),
        import_addition_paths: None,
    })
}

// Extracts comments from the node's text and formats them with the given indentation.
fn extract_comments<'db>(db: &'db dyn Database, node: SyntaxNode<'db>, indent: &str) -> String {
    let text = node.get_text(db);
    let comments_lines = text
        .lines()
        .filter_map(|line| extract_comment_only(line).map(|comment| format!("{indent}{comment}")))
        .collect::<Vec<_>>();

    if !comments_lines.is_empty() {
        let mut comments = comments_lines.join("\n");
        if !comments.ends_with('\n') {
            comments.push('\n');
        }
        comments
    } else {
        String::new()
    }
}

// Extracts only the comment from a line, e.g. `let x = 5; // comment` -> `// comment`
fn extract_comment_only(line: &str) -> Option<String> {
    line.find("//").map(|idx| line[idx..].trim().to_string())
}
