use cairo_lang_defs::ids::ModuleItemId;
use cairo_lang_defs::plugin::PluginDiagnostic;
use cairo_lang_diagnostics::Severity;
use cairo_lang_semantic::db::SemanticGroup;
use cairo_lang_semantic::{Arenas, ExprMatch, Pattern};
use cairo_lang_syntax::node::ast::{Expr as AstExpr, ExprBlock, ExprListParenthesized, Statement};
use cairo_lang_syntax::node::db::SyntaxGroup;
use cairo_lang_syntax::node::{
    ast::{ExprMatch as AstExprMatch, Pattern as AstPattern},
    SyntaxNode, TypedStablePtr, TypedSyntaxNode,
};
use if_chain::if_chain;

use crate::context::{CairoLintKind, Lint};
use crate::helper::indent_snippet;
use crate::queries::{get_all_function_bodies, get_all_match_expressions};

pub struct DestructMatch;

impl Lint for DestructMatch {
    fn allowed_name(&self) -> &'static str {
        "destruct_match"
    }

    fn diagnostic_message(&self) -> &'static str {
        "you seem to be trying to use `match` for destructuring a single pattern. Consider using `if let`"
    }

    fn kind(&self) -> CairoLintKind {
        CairoLintKind::DestructMatch
    }

    fn has_fixer(&self) -> bool {
        true
    }

    fn fix(&self, db: &dyn SyntaxGroup, node: SyntaxNode) -> Option<(SyntaxNode, String)> {
        fix_destruct_match(db, node)
    }
}

pub struct EqualityMatch;

impl Lint for EqualityMatch {
    fn allowed_name(&self) -> &'static str {
        "equality_match"
    }

    fn diagnostic_message(&self) -> &'static str {
        "you seem to be trying to use `match` for an equality check. Consider using `if`"
    }

    fn kind(&self) -> CairoLintKind {
        CairoLintKind::MatchForEquality
    }
}

/// Checks for matches that do something only in 1 arm and can be rewrote as an `if let`
/// ```ignore
/// let var = Option::Some(1_u32);
/// match var {
///     Option::Some(val) => do_smth(val),
///     _ => (),
/// }
/// ```
/// Which can be rewritten as
/// ```ignore
/// if let Option::Some(val) = var {
///     do_smth(val),
/// }
/// ```
pub fn check_single_matches(
    db: &dyn SemanticGroup,
    item: &ModuleItemId,
    diagnostics: &mut Vec<PluginDiagnostic>,
) {
    let function_bodies = get_all_function_bodies(db, item);
    for function_body in function_bodies.iter() {
        let match_exprs = get_all_match_expressions(function_body);
        let arenas = &function_body.arenas;
        for match_expr in match_exprs.iter() {
            check_single_match(db, match_expr, arenas, diagnostics);
        }
    }
}

fn check_single_match(
    db: &dyn SemanticGroup,
    match_expr: &ExprMatch,
    arenas: &Arenas,
    diagnostics: &mut Vec<PluginDiagnostic>,
) {
    let arms = &match_expr.arms;
    let mut is_single_armed = false;
    let mut is_complete = false;
    let mut is_destructuring = false;

    // If the match isn't of unit type it means that both branches return something so it can't be a
    // single match
    if arms.len() != 2 || !match_expr.ty.is_unit(db) {
        return;
    }

    let first_arm = &arms[0];
    let second_arm = &arms[1];
    let mut enum_len = None;
    if let Some(pattern) = first_arm.patterns.first() {
        match &arenas.patterns[*pattern] {
            // If the first arm is `_ => ...` the enum is wrong
            Pattern::Otherwise(_) => return,
            // Get the number of variants in the enum to know if it's comprehensive or not
            Pattern::EnumVariant(enum_pat) => {
                enum_len = Some(
                    db.enum_variants(enum_pat.variant.concrete_enum_id.enum_id(db))
                        .unwrap()
                        .len(),
                );
                // If there's an enum pattern it's a destructuring match
                is_destructuring = enum_pat.inner_pattern.is_some();
            }
            Pattern::Struct(_) => {
                // If it's a struct pattern it's a destructuring match
                is_destructuring = true;
            }
            _ => (),
        };
    };
    if let Some(pattern) = second_arm.patterns.first() {
        match &arenas.patterns[*pattern] {
            // If the second arm is `_ => ...` the match is comprehensive
            Pattern::Otherwise(_) => {
                is_complete = true;
            }
            Pattern::EnumVariant(_) => {
                // And if the 2nd arm is an enum variant check that the number of variants in the enum is 2.
                if enum_len == Some(2) {
                    is_complete = true;
                }
            }
            _ => (),
        };

        // Checks that the second arm doesn't do anything
        is_single_armed = is_expr_unit(
            arenas.exprs[second_arm.expression]
                .stable_ptr()
                .lookup(db.upcast()),
            db.upcast(),
        ) && is_complete;
    };

    match (is_single_armed, is_destructuring) {
        (true, false) => diagnostics.push(PluginDiagnostic {
            stable_ptr: match_expr.stable_ptr.into(),
            message: EqualityMatch.diagnostic_message().to_string(),
            severity: Severity::Warning,
        }),
        (true, true) => diagnostics.push(PluginDiagnostic {
            stable_ptr: match_expr.stable_ptr.into(),
            message: DestructMatch.diagnostic_message().to_string(),
            severity: Severity::Warning,
        }),
        (_, _) => (),
    }
}

/// Is a tuple expression the unit type.
fn is_expr_list_parenthesised_unit(expr: &ExprListParenthesized, db: &dyn SyntaxGroup) -> bool {
    expr.expressions(db).elements(db).is_empty()
}

/// Is the block empty `{}` or `{ () }` but it shouldn't contain a comment.
fn is_block_expr_unit_without_comment(block_expr: &ExprBlock, db: &dyn SyntaxGroup) -> bool {
    let statements = block_expr.statements(db).elements(db);
    // Check if the block is empty and there's no comment in it
    if statements.is_empty()
        && block_expr
            .rbrace(db)
            .leading_trivia(db)
            .node
            .get_text(db)
            .trim()
            .is_empty()
    {
        return true;
    }

    // If there's statement checks that it's `()` without comment
    if_chain! {
        if statements.len() == 1;
        if let Statement::Expr(statement_expr) = &statements[0];
        if let AstExpr::Tuple(tuple_expr) = statement_expr.expr(db);
        then {
            let tuple_node = tuple_expr.as_syntax_node();
            if tuple_node.span(db).start != tuple_node.span_start_without_trivia(db) {
                return false;
            }
            return is_expr_list_parenthesised_unit(&tuple_expr, db);
        }
    }
    false
}

/// Checks that either the expression is `()` or `{ }` or `{ () }` but none of them should contain a
/// comment.
pub fn is_expr_unit(expr: AstExpr, db: &dyn SyntaxGroup) -> bool {
    match expr {
        AstExpr::Block(block_expr) => is_block_expr_unit_without_comment(&block_expr, db),
        AstExpr::Tuple(tuple_expr) => is_expr_list_parenthesised_unit(&tuple_expr, db),
        _ => false,
    }
}

/// Fixes a destructuring match by converting it to an if-let expression.
///
/// This method handles matches with two arms, where one arm is a wildcard (_)
/// and the other is either an enum or struct pattern.
///
/// # Arguments
///
/// * `db` - A reference to the SyntaxGroup
/// * `node` - The SyntaxNode representing the match expression
///
/// # Returns
///
/// A `String` containing the if-let expression that replaces the match.
///
/// # Panics
///
/// Panics if the diagnostic is incorrect (i.e., the match doesn't have the expected structure).
pub fn fix_destruct_match(db: &dyn SyntaxGroup, node: SyntaxNode) -> Option<(SyntaxNode, String)> {
    let match_expr = AstExprMatch::from_syntax_node(db, node.clone());
    let arms = match_expr.arms(db).elements(db);
    let first_arm = &arms[0];
    let second_arm = &arms[1];
    let (pattern, first_expr) = match (
        &first_arm.patterns(db).elements(db)[0],
        &second_arm.patterns(db).elements(db)[0],
    ) {
        (AstPattern::Underscore(_), AstPattern::Enum(pat)) => (pat.as_syntax_node(), second_arm),
        (AstPattern::Enum(pat), AstPattern::Underscore(_)) => (pat.as_syntax_node(), first_arm),
        (AstPattern::Underscore(_), AstPattern::Struct(pat)) => (pat.as_syntax_node(), second_arm),
        (AstPattern::Struct(pat), AstPattern::Underscore(_)) => (pat.as_syntax_node(), first_arm),
        (AstPattern::Enum(pat1), AstPattern::Enum(pat2)) => {
            if is_expr_unit(second_arm.expression(db), db) {
                (pat1.as_syntax_node(), first_arm)
            } else {
                (pat2.as_syntax_node(), second_arm)
            }
        }
        (_, _) => panic!("Incorrect diagnostic"),
    };
    let mut pattern_span = pattern.span(db);
    pattern_span.end = pattern.span_start_without_trivia(db);
    let indent = node
        .get_text(db)
        .chars()
        .take_while(|c| c.is_whitespace())
        .collect::<String>();
    let trivia = pattern.clone().get_text_of_span(db, pattern_span);
    Some((
        node,
        indent_snippet(
            &format!(
                "{trivia}{indent}if let {} = {} {{\n{}\n}}",
                pattern.get_text_without_trivia(db),
                match_expr
                    .expr(db)
                    .as_syntax_node()
                    .get_text_without_trivia(db),
                first_expr
                    .expression(db)
                    .as_syntax_node()
                    .get_text_without_trivia(db),
            ),
            indent.len() / 4,
        ),
    ))
}
