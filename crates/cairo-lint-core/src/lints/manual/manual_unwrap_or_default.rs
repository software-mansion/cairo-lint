use cairo_lang_defs::ids::ModuleItemId;
use cairo_lang_defs::plugin::PluginDiagnostic;
use cairo_lang_diagnostics::Severity;
use cairo_lang_semantic::db::SemanticGroup;
use cairo_lang_syntax::node::{
    ast::{Condition, Expr},
    db::SyntaxGroup,
    SyntaxNode, TypedStablePtr, TypedSyntaxNode,
};

use crate::{
    context::CairoLintKind,
    queries::{get_all_function_bodies, get_all_if_expressions, get_all_match_expressions},
};
use crate::{
    context::Lint,
    lints::manual::{check_manual, check_manual_if, ManualLint},
};

pub struct ManualUnwrapOrDefault;

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

    fn fix(&self, db: &dyn SyntaxGroup, node: SyntaxNode) -> Option<(SyntaxNode, String)> {
        fix_manual_unwrap_or_default(db, node)
    }
}

pub fn check_manual_unwrap_or_default(
    db: &dyn SemanticGroup,
    item: &ModuleItemId,
    diagnostics: &mut Vec<PluginDiagnostic>,
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
                });
            }
        }
        for if_expr in if_exprs.iter() {
            if check_manual_if(db, if_expr, arenas, ManualLint::ManualUnwrapOrDefault) {
                diagnostics.push(PluginDiagnostic {
                    stable_ptr: if_expr.stable_ptr.untyped(),
                    message: ManualUnwrapOrDefault.diagnostic_message().to_owned(),
                    severity: Severity::Warning,
                });
            }
        }
    }
}

/// Rewrites manual unwrap or default to use unwrap_or_default
pub fn fix_manual_unwrap_or_default(
    db: &dyn SyntaxGroup,
    node: SyntaxNode,
) -> Option<(SyntaxNode, String)> {
    // Check if the node is a general expression
    let expr = Expr::from_syntax_node(db, node.clone());

    let matched_expr = match expr {
        // Handle the case where the expression is a match expression
        Expr::Match(expr_match) => expr_match.expr(db).as_syntax_node(),

        // Handle the case where the expression is an if-let expression
        Expr::If(expr_if) => {
            // Extract the condition from the if-let expression
            let condition = expr_if.condition(db);

            match condition {
                Condition::Let(condition_let) => {
                    // Extract and return the syntax node for the matched expression
                    condition_let.expr(db).as_syntax_node()
                }
                _ => panic!("Expected an `if let` expression."),
            }
        }
        // Handle unsupported expressions
        _ => panic!("The expression cannot be simplified to `.unwrap_or_default()`."),
    };

    let indent = node
        .get_text(db)
        .chars()
        .take_while(|c| c.is_whitespace())
        .collect::<String>();
    Some((
        node,
        format!(
            "{indent}{}.unwrap_or_default()",
            matched_expr.get_text_without_trivia(db)
        ),
    ))
}
