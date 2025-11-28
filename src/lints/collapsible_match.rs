use cairo_lang_defs::{ids::ModuleItemId, plugin::PluginDiagnostic};
use cairo_lang_diagnostics::Severity;
use cairo_lang_semantic::{Arenas, Expr, ExprMatch, MatchArm, Statement, VarId};
use cairo_lang_syntax::node::{SyntaxNode, TypedStablePtr, TypedSyntaxNode, ast};
use indoc::{formatdoc, indoc};
use salsa::Database;

use crate::{
    context::{CairoLintKind, Lint},
    fixer::InternalFix,
    lints::manual::helpers::extract_pattern_variable,
    queries::{get_all_function_bodies, get_all_match_expressions},
};

pub struct CollapsibleMatch;

/// ## What it does
///
/// Checks for nested `match` statements that can be collapsed into a single `match` statement.
/// Note that this lint is not intended to find all cases where nested match patterns can be merged, but only cases where merging would most likely make the code more readable.
/// ## Example
///
/// ```cairo
/// fn func(opt: Option<Result<u32, felt252>>) {
///     let n = match opt {
///         Some(n) => match n {
///             Ok(n) => n,
///             _ => return,
///         }
///         None => return,
///     };
/// }
/// ```
///
/// Can be collapsed to
///
/// ```cairo
/// fn func(opt: Option<Result<u32, felt252>>) {
///     let n = match opt {
///         Some(Ok(n)) => n,
///         _ => return,
///     };
/// }
/// ```
impl Lint for CollapsibleMatch {
    fn allowed_name(&self) -> &'static str {
        "collapsible_match"
    }

    fn diagnostic_message(&self) -> &'static str {
        "Nested `match` statements can be collapsed into a single `match` statement."
    }

    fn kind(&self) -> CairoLintKind {
        CairoLintKind::CollapsibleMatch
    }

    fn has_fixer(&self) -> bool {
        true
    }

    fn fix<'db>(&self, db: &'db dyn Database, node: SyntaxNode<'db>) -> Option<InternalFix<'db>> {
        fix_collapsible_match(db, node)
    }

    fn fix_message(&self) -> Option<&'static str> {
        Some("Combine nested matches into a single match")
    }
}

#[tracing::instrument(skip_all, level = "trace")]
pub fn check_collapsible_match<'db>(
    db: &'db dyn Database,
    item: &ModuleItemId<'db>,
    diagnostics: &mut Vec<PluginDiagnostic<'db>>,
) {
    let function_bodies = get_all_function_bodies(db, item);
    for function_body in function_bodies.iter() {
        let match_exprs = get_all_match_expressions(function_body);
        let arenas = &function_body.arenas;
        for match_expr in match_exprs {
            check_single_collapsible_match(db, &match_expr, arenas, diagnostics);
        }
    }
}

fn check_single_collapsible_match<'db>(
    db: &'db dyn Database,
    match_expr: &ExprMatch<'db>,
    arenas: &Arenas<'db>,
    diagnostics: &mut Vec<PluginDiagnostic<'db>>,
) {
    let arms = &match_expr.arms;

    if arms.len() != 2 {
        return;
    }

    let first_arm = &arms[0];
    let second_arm = &arms[1];

    let first_inner_match = get_inner_match_expression_if_single_one(first_arm, arenas);
    let second_inner_match = get_inner_match_expression_if_single_one(second_arm, arenas);

    // We only support the case, where there is only a single inner match.
    match (first_inner_match, second_inner_match) {
        (Some(_), Some(_)) => {
            return;
        }
        (None, None) => {
            return;
        }
        _ => {}
    }

    if let Some(inner_match) = first_inner_match {
        if first_arm.patterns.len() != 1 {
            return;
        }
        if check_if_matches_are_simplifiable(db, arenas, first_arm, second_arm, inner_match) {
            diagnostics.push(PluginDiagnostic {
                stable_ptr: match_expr.stable_ptr.untyped(),
                message: CollapsibleMatch.diagnostic_message().to_string(),
                severity: Severity::Warning,
                inner_span: None,
            });
        }
    }

    if let Some(inner_match) = second_inner_match {
        if second_arm.patterns.len() != 1 {
            return;
        }
        if check_if_matches_are_simplifiable(db, arenas, second_arm, first_arm, inner_match) {
            diagnostics.push(PluginDiagnostic {
                stable_ptr: match_expr.stable_ptr.untyped(),
                message: CollapsibleMatch.diagnostic_message().to_string(),
                severity: Severity::Warning,
                inner_span: None,
            });
        }
    }
}

fn check_if_matches_are_simplifiable<'db>(
    db: &'db dyn Database,
    arenas: &Arenas<'db>,
    arm_with_inner_match: &MatchArm,
    secondary_arm: &MatchArm,
    inner_match: &ExprMatch<'db>,
) -> bool {
    if inner_match.arms.len() != 2 {
        return false;
    }

    // If there is no match of expressions between outer arm and any of the inner arms, we don't go for diagnostics.
    let Some(inner_duplicated_arm) = get_inner_arm_with_same_expression_as_outer_match_arm(
        db,
        arenas,
        secondary_arm,
        inner_match,
    ) else {
        return false;
    };

    // The arm in the inner match, that should be merged with the outer arm.
    let collapsible_arm = if inner_duplicated_arm == inner_match.arms[0] {
        &inner_match.arms[1]
    } else {
        &inner_match.arms[0]
    };

    if collapsible_arm.patterns.len() != 1 {
        return false;
    }
    let inner_collapsible_pattern = &arenas.patterns[collapsible_arm.patterns[0]];
    if !matches!(
        inner_collapsible_pattern,
        cairo_lang_semantic::Pattern::EnumVariant(_)
    ) {
        return false;
    }

    // We don't support multiple patterns in the outer match arm, as it's not a good idea to try to "merge" them with the inner one.
    // It would likely lead to less readable code.
    if arm_with_inner_match.patterns.len() != 1 {
        return false;
    }
    let pattern = &arenas.patterns[arm_with_inner_match.patterns[0]];

    // This one also ensures that the pattern is an enum variant pattern.
    let Some(pattern_variable) = extract_pattern_variable(pattern, arenas) else {
        return false;
    };

    let inner_matched_expr = &arenas.exprs[inner_match.matched_expr];

    let Expr::Var(inner_var_expr) = inner_matched_expr else {
        return false;
    };

    let VarId::Local(inner_local_var_id) = inner_var_expr.var else {
        return false;
    };

    inner_local_var_id == pattern_variable.var.id
}

fn get_inner_arm_with_same_expression_as_outer_match_arm<'db>(
    db: &'db dyn Database,
    arenas: &Arenas<'db>,
    outer_match_arm: &MatchArm,
    inner_match: &ExprMatch<'db>,
) -> Option<MatchArm> {
    let outer_arm_expression_text = &arenas.exprs[outer_match_arm.expression]
        .stable_ptr()
        .lookup(db)
        .as_syntax_node()
        .get_text_without_trivia(db);

    let inner_first_arm_expression_text = &arenas.exprs[inner_match.arms[0].expression]
        .stable_ptr()
        .lookup(db)
        .as_syntax_node()
        .get_text_without_trivia(db);

    let inner_second_arm_expression_text = &arenas.exprs[inner_match.arms[1].expression]
        .stable_ptr()
        .lookup(db)
        .as_syntax_node()
        .get_text_without_trivia(db);

    if outer_arm_expression_text == inner_first_arm_expression_text {
        Some(inner_match.arms[0].clone())
    } else if outer_arm_expression_text == inner_second_arm_expression_text {
        Some(inner_match.arms[1].clone())
    } else {
        None
    }
}

/// Gets the inner match expression from a match arm if only the inner match is the only expression.
fn get_inner_match_expression_if_single_one<'db>(
    match_arm: &'db MatchArm,
    arenas: &'db Arenas<'db>,
) -> Option<&'db ExprMatch<'db>> {
    let arm_expression = &arenas.exprs[match_arm.expression];

    match arm_expression {
        Expr::Match(inner_match) => Some(inner_match),
        Expr::Block(block) => match block.statements.len() {
            // If there's not statements, check the tail for match expression.
            0 => {
                if let Some(expr_id) = block.tail
                    && let Expr::Match(inner_match) = &arenas.exprs[expr_id]
                {
                    Some(inner_match)
                } else {
                    None
                }
            }
            // If there's a single statement, check if it's a match expression.
            1 => {
                let first_statement = &block.statements[0];
                if let Statement::Expr(statement_expr) = &arenas.statements[*first_statement]
                    && let Expr::Match(inner_match) = &arenas.exprs[statement_expr.expr]
                {
                    Some(inner_match)
                } else {
                    None
                }
            }
            _ => None,
        },
        _ => None,
    }
}

#[tracing::instrument(skip_all, level = "trace")]
pub fn fix_collapsible_match<'db>(
    db: &'db dyn Database,
    node: SyntaxNode<'db>,
) -> Option<InternalFix<'db>> {
    let ast_expr = ast::Expr::from_syntax_node(db, node);
    let ast::Expr::Match(match_expr) = &ast_expr else {
        panic!("Expr should be a Match expression");
    };

    let mut match_arms = match_expr.arms(db).elements(db);

    let match_expr_text = match_expr.expr(db).as_syntax_node().get_text(db).trim();

    assert!(match_arms.len() == 2, "Match should have exactly two arms");

    let first_arm = &match_arms.next().unwrap();
    let second_arm = &match_arms.next().unwrap();

    if let Some(inner_match) = get_inner_match_syntax_expression(db, first_arm) {
        return Some(InternalFix {
            node,
            suggestion: get_collapsed_match(
                db,
                match_expr_text,
                first_arm,
                second_arm,
                &inner_match,
            ),
            description: CollapsibleMatch.fix_message().unwrap().to_string(),
            import_addition_paths: None,
        });
    }

    if let Some(inner_match) = get_inner_match_syntax_expression(db, second_arm) {
        return Some(InternalFix {
            node,
            suggestion: get_collapsed_match(
                db,
                match_expr_text,
                second_arm,
                first_arm,
                &inner_match,
            ),
            description: CollapsibleMatch.fix_message().unwrap().to_string(),
            import_addition_paths: None,
        });
    }

    None
}

fn get_collapsed_match<'db>(
    db: &'db dyn Database,
    match_expr: &'db str,
    // The arm which contains the inner match.
    main_arm: &ast::MatchArm<'db>,
    // The arm which contains the redundant expression.
    secondary_arm: &ast::MatchArm<'db>,
    inner_match: &ast::ExprMatch<'db>,
) -> String {
    let mut inner_arms = inner_match.arms(db).elements(db);
    assert!(
        inner_arms.len() == 2,
        "Inner match should have exactly two arms"
    );
    let redundant_arm_code = secondary_arm
        .expression(db)
        .as_syntax_node()
        .get_text_without_trivia(db);

    let inner_first_arm = inner_arms.next().unwrap();
    let inner_second_arm = inner_arms.next().unwrap();

    let inner_arm_to_collapse = if inner_first_arm
        .expression(db)
        .as_syntax_node()
        .get_text_without_trivia(db)
        == redundant_arm_code
    {
        inner_second_arm
    } else {
        inner_first_arm
    };

    assert!(
        inner_arm_to_collapse.patterns(db).elements(db).len() == 1,
        "Inner arm to collapse should have exactly one pattern"
    );

    let inner_pattern = &inner_arm_to_collapse
        .patterns(db)
        .elements(db)
        .next()
        .unwrap();

    let outer_pattern_to_collapse = &main_arm.patterns(db).elements(db).next().unwrap();
    let ast::Pattern::Enum(enum_pattern) = outer_pattern_to_collapse else {
        panic!("Outer pattern should be an enum pattern");
    };

    let ast::OptionPatternEnumInnerPattern::PatternEnumInnerPattern(pattern_variable) =
        enum_pattern.pattern(db)
    else {
        panic!("Outer pattern should include a variable inside")
    };

    let collapsible_expression = inner_arm_to_collapse
        .expression(db)
        .as_syntax_node()
        .get_text(db);

    let outer_pattern_path = enum_pattern.path(db).as_syntax_node();

    let collapsed_pattern = format!(
        "{}{}{}{}",
        outer_pattern_path.get_text(db).trim(),
        pattern_variable
            .lparen(db)
            .as_syntax_node()
            .get_text(db)
            .trim(),
        inner_pattern.as_syntax_node().get_text(db).trim(),
        pattern_variable
            .rparen(db)
            .as_syntax_node()
            .get_text(db)
            .trim()
    );

    let redundant_arm_code_string = redundant_arm_code.to_string(db);

    formatdoc! {
      r#"
        match {match_expr} {{
            {collapsed_pattern} => {collapsible_expression},
            _ => {redundant_arm_code_string},
        }}
      "#,
    }
}

fn get_inner_match_syntax_expression<'db>(
    db: &'db dyn Database,
    match_arm: &ast::MatchArm<'db>,
) -> Option<ast::ExprMatch<'db>> {
    let arm_expression = match_arm.expression(db);
    if let ast::Expr::Match(inner_match) = &arm_expression {
        return Some(inner_match.clone());
    } else if let ast::Expr::Block(block) = &arm_expression {
        let statements = block.statements(db).elements(db).collect::<Vec<_>>();
        if statements.len() == 1 {
            let first_statement = &statements[0];
            if let ast::Statement::Expr(statement_expr) = first_statement {
                let statement_expression = statement_expr.expr(db);
                if let ast::Expr::Match(inner_match) = &statement_expression {
                    return Some(inner_match.clone());
                }
            }
        }
    }
    None
}
