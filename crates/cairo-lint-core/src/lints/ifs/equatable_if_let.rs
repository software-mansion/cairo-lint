use cairo_lang_defs::ids::ModuleItemId;
use cairo_lang_defs::plugin::PluginDiagnostic;
use cairo_lang_diagnostics::Severity;
use cairo_lang_semantic::db::SemanticGroup;
use cairo_lang_semantic::{Arenas, Condition, Expr, ExprIf, Pattern, PatternId};
use cairo_lang_syntax::node::db::SyntaxGroup;
use cairo_lang_syntax::node::{
    ast::{Condition as AstCondition, ExprIf as AstExprIf},
    SyntaxNode, TypedStablePtr, TypedSyntaxNode,
};

use crate::context::{CairoLintKind, Lint};
use crate::queries::{get_all_function_bodies, get_all_if_expressions};

const EQUATABLE_IF_LET: &str =
    "`if let` pattern used for equatable value. Consider using a simple comparison `==` instead";
const EQUATABLE_IF_LET_LINT_NAME: &str = "equatable_if_let";

pub struct EquatableIfLet;

impl Lint for EquatableIfLet {
    fn allowed_name(&self) -> &'static str {
        EQUATABLE_IF_LET_LINT_NAME
    }

    fn diagnostic_message(&self) -> &'static str {
        EQUATABLE_IF_LET
    }

    fn kind(&self) -> CairoLintKind {
        CairoLintKind::EquatableIfLet
    }

    fn has_fixer(&self) -> bool {
        true
    }

    fn fix(&self, db: &dyn SyntaxGroup, node: SyntaxNode) -> Option<(SyntaxNode, String)> {
        fix_equatable_if_let(db, node)
    }
}

/// Checks for
/// ```ignore
/// if let Some(2) = a {
///     ...
/// }
/// ```
/// Which can be replaced by
/// ```ignore
/// if a == Some(2) {
///    ...
/// }
/// ````
pub fn check_equatable_if_let(
    db: &dyn SemanticGroup,
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
    if let Condition::Let(condition_let, patterns) = &if_expr.condition {
        // Simple literals and variables
        let expr_is_simple = matches!(
            arenas.exprs[*condition_let],
            Expr::Literal(_) | Expr::StringLiteral(_) | Expr::Var(_)
        );
        let condition_is_simple = is_simple_equality_condition(patterns, arenas);

        if expr_is_simple && condition_is_simple {
            diagnostics.push(PluginDiagnostic {
                stable_ptr: if_expr.stable_ptr.untyped(),
                message: EQUATABLE_IF_LET.to_string(),
                severity: Severity::Warning,
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
                })
            }
            _ => continue,
        }
    }
    false
}

/// Rewrites a useless `if let` to a simple `if`
pub fn fix_equatable_if_let(
    db: &dyn SyntaxGroup,
    node: SyntaxNode,
) -> Option<(SyntaxNode, String)> {
    let expr = AstExprIf::from_syntax_node(db, node.clone());
    let condition = expr.condition(db);

    let fixed_condition = match condition {
        AstCondition::Let(condition_let) => {
            format!(
                "{} == {} ",
                condition_let
                    .expr(db)
                    .as_syntax_node()
                    .get_text_without_trivia(db),
                condition_let
                    .patterns(db)
                    .as_syntax_node()
                    .get_text_without_trivia(db),
            )
        }
        _ => panic!("Incorrect diagnostic"),
    };

    Some((
        node,
        format!(
            "{}{}{}",
            expr.if_kw(db).as_syntax_node().get_text(db),
            fixed_condition,
            expr.if_block(db).as_syntax_node().get_text(db),
        ),
    ))
}
