use cairo_lang_defs::ids::ModuleItemId;
use cairo_lang_defs::plugin::PluginDiagnostic;
use cairo_lang_diagnostics::Severity;
use cairo_lang_semantic::db::SemanticGroup;
use cairo_lang_semantic::{Arenas, Condition, Expr, ExprIf, Pattern, PatternId};
use cairo_lang_syntax::node::db::SyntaxGroup;
use cairo_lang_syntax::node::{
    SyntaxNode, TypedStablePtr, TypedSyntaxNode,
    ast::{Condition as AstCondition, ExprIf as AstExprIf},
};

use crate::LinterGroup;
use crate::context::{CairoLintKind, Lint};
use crate::fixer::InternalFix;
use crate::queries::{get_all_function_bodies, get_all_if_expressions};

pub struct EquatableIfLet;

/// ## What it does
///
/// Checks for `if let` pattern matching that can be replaced by a simple comparison.
///
/// ## Example
///
/// ```cairo
/// if let Some(2) = a {
///     // Code
/// }
/// ```
///
/// Could be replaced by
///
/// ```cairo
/// if a == Some(2) {
///     // Code
/// }
/// ```
impl Lint for EquatableIfLet {
    fn allowed_name(&self) -> &'static str {
        "equatable_if_let"
    }

    fn diagnostic_message(&self) -> &'static str {
        "`if let` pattern used for equatable value. Consider using a simple comparison `==` instead"
    }

    fn kind(&self) -> CairoLintKind {
        CairoLintKind::EquatableIfLet
    }

    fn has_fixer(&self) -> bool {
        true
    }

    fn fix(&self, db: &dyn LinterGroup, node: SyntaxNode) -> Option<InternalFix> {
        fix_equatable_if_let(db.upcast(), node)
    }

    fn fix_message(&self) -> Option<&'static str> {
        Some("Replace `if let` with `==` comparison")
    }
}

#[tracing::instrument(skip_all, level = "trace")]
pub fn check_equatable_if_let(
    db: &dyn LinterGroup,
    item: &ModuleItemId,
    diagnostics: &mut Vec<PluginDiagnostic>,
) {
    let function_bodies = get_all_function_bodies(db, item);
    for function_body in function_bodies.iter() {
        let if_exprs = get_all_if_expressions(function_body);
        let arenas = &function_body.arenas;
        for if_expr in if_exprs.iter() {
            check_single_equatable_if_let(db, if_expr, arenas, diagnostics);
        }
    }
}

fn check_single_equatable_if_let(
    _db: &dyn SemanticGroup,
    if_expr: &ExprIf,
    arenas: &Arenas,
    diagnostics: &mut Vec<PluginDiagnostic>,
) {
    if let Some(Condition::Let(condition_let, patterns)) = &if_expr.conditions.first() {
        // Simple literals and variables
        let expr_is_simple = matches!(
            arenas.exprs[*condition_let],
            Expr::Literal(_) | Expr::StringLiteral(_) | Expr::Var(_)
        );
        let condition_is_simple = is_simple_equality_condition(patterns, arenas);

        if expr_is_simple && condition_is_simple {
            diagnostics.push(PluginDiagnostic {
                stable_ptr: if_expr.stable_ptr.untyped(),
                message: EquatableIfLet.diagnostic_message().to_string(),
                severity: Severity::Warning,
                inner_span: None,
            });
        }
    }
}

fn is_simple_equality_condition(patterns: &[PatternId], arenas: &Arenas) -> bool {
    for pattern in patterns {
        match &arenas.patterns[*pattern] {
            Pattern::Literal(_) | Pattern::StringLiteral(_) => return true,
            Pattern::EnumVariant(pat) => {
                return pat.inner_pattern.is_none_or(|pat_id| {
                    matches!(
                        arenas.patterns[pat_id],
                        Pattern::Literal(_) | Pattern::StringLiteral(_)
                    )
                });
            }
            _ => continue,
        }
    }
    false
}

/// Rewrites a useless `if let` to a simple `if`
#[tracing::instrument(skip_all, level = "trace")]
pub fn fix_equatable_if_let(db: &dyn SyntaxGroup, node: SyntaxNode) -> Option<InternalFix> {
    let expr = AstExprIf::from_syntax_node(db, node);
    let mut conditions = expr.conditions(db).elements(db);
    let condition = conditions.next()?;

    let fixed_condition = match condition {
        AstCondition::Let(condition_let) => {
            format!(
                "{} == {} ",
                condition_let
                    .expr(db)
                    .as_syntax_node()
                    .get_text(db)
                    .trim_end(),
                condition_let
                    .patterns(db)
                    .as_syntax_node()
                    .get_text(db)
                    .trim_end(),
            )
        }
        _ => panic!("Incorrect diagnostic"),
    };

    Some(InternalFix {
        node,
        suggestion: format!(
            "{}{}{}",
            expr.if_kw(db).as_syntax_node().get_text(db),
            fixed_condition,
            expr.if_block(db).as_syntax_node().get_text(db),
        ),
        description: EquatableIfLet.fix_message().unwrap().to_string(),
        import_addition_paths: None,
    })
}
