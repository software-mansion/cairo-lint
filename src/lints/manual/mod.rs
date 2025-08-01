pub mod helpers;
pub mod manual_assert;
pub mod manual_err;
pub mod manual_expect;
pub mod manual_expect_err;
pub mod manual_is;
pub mod manual_is_empty;
pub mod manual_ok;
pub mod manual_ok_or;
pub mod manual_unwrap_or;
pub mod manual_unwrap_or_default;

use std::fmt::Debug;

use cairo_lang_defs::ids::TopLevelLanguageElementId;
use cairo_lang_semantic::db::SemanticGroup;
use cairo_lang_semantic::{Arenas, Condition, Expr, ExprIf, ExprMatch, Pattern};
use cairo_lang_syntax::node::{TypedStablePtr, ast};
use helpers::{
    check_is_default, func_call_or_block_returns_never,
    if_expr_condition_and_block_match_enum_pattern, if_expr_pattern_matches_tail_var,
    is_destructured_variable_used_and_expected_variant, is_expected_function,
    match_arm_returns_extracted_var,
};
use if_chain::if_chain;

use super::{FALSE, OK, PANIC_WITH_FELT252, TRUE};
use crate::lints::manual::helpers::{
    extract_pattern_variable, extract_tail_or_preserve_expr, is_variable_unused,
};
use crate::lints::{ERR, NONE, SOME};

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum ManualLint {
    ManualOkOr,
    ManualIsSome,
    ManualIsNone,
    ManualExpect,
    ManualUnwrapOrDefault,
    ManualIsOk,
    ManualIsErr,
    ManualOptExpect,
    ManualResExpect,
    ManualOk,
    ManualErr,
    ManualExpectErr,
    ManualUnwrapOr,
    ManualIsEmpty,
}

/// Checks for all the manual lint written as `match`.
/// ```text
/// let res_val: Result<i32> = Result::Err('err');
/// let _a = match res_val {
///     Result::Ok(x) => Option::Some(x),
///     Result::Err(_) => Option::None,
/// };
/// ```
pub fn check_manual<'db>(
    db: &'db dyn SemanticGroup,
    expr_match: &ExprMatch<'db>,
    arenas: &Arenas<'db>,
    manual_lint: ManualLint,
) -> bool {
    if expr_match.arms.len() != 2 {
        return false;
    }

    let ast::Expr::Match(ast_expr_match) = expr_match.stable_ptr.lookup(db) else {
        return false;
    };

    // Check that no arm has more than one statement in its block
    if !ast_expr_match
        .arms(db)
        .elements(db)
        .all(|arm| match arm.expression(db) {
            ast::Expr::Block(block) => block.statements(db).elements(db).len() <= 1,
            _ => true,
        })
    {
        return false;
    }

    let (first_arm, second_arm) = (&expr_match.arms[0], &expr_match.arms[1]);

    let (Pattern::EnumVariant(first_pattern), Pattern::EnumVariant(second_pattern)) = (
        &arenas.patterns[first_arm.patterns[0]],
        &arenas.patterns[second_arm.patterns[0]],
    ) else {
        return false;
    };

    let (first_expr, second_expr) = (
        extract_tail_or_preserve_expr(&arenas.exprs[first_arm.expression], arenas),
        extract_tail_or_preserve_expr(&arenas.exprs[second_arm.expression], arenas),
    );

    let first_enum = first_pattern.variant.id.full_path(db);
    let second_enum = second_pattern.variant.id.full_path(db);

    match (first_enum.as_str(), second_enum.as_str()) {
        (SOME, NONE) => {
            check_syntax_some_arm(
                first_expr,
                &arenas.patterns[first_arm.patterns[0]],
                db,
                arenas,
                manual_lint,
            ) && check_syntax_none_arm(
                second_expr,
                &arenas.patterns[second_arm.patterns[0]],
                db,
                arenas,
                manual_lint,
            )
        }
        (OK, ERR) => {
            check_syntax_ok_arm(
                first_expr,
                &arenas.patterns[first_arm.patterns[0]],
                db,
                arenas,
                manual_lint,
            ) && check_syntax_err_arm(
                second_expr,
                &arenas.patterns[second_arm.patterns[0]],
                db,
                arenas,
                manual_lint,
            )
        }
        _ => false,
    }
}

/// Checks the `Option::Some` arm in the match.
fn check_syntax_some_arm<'db>(
    expr: &Expr<'db>,
    pattern: &Pattern<'db>,
    db: &'db dyn SemanticGroup,
    arenas: &Arenas<'db>,
    manual_lint: ManualLint,
) -> bool {
    match manual_lint {
        ManualLint::ManualOkOr => {
            is_destructured_variable_used_and_expected_variant(expr, pattern, db, arenas, OK)
        }
        ManualLint::ManualIsSome => is_expected_variant(expr, db, TRUE),
        ManualLint::ManualIsNone => is_expected_variant(expr, db, FALSE),
        ManualLint::ManualUnwrapOr
        | ManualLint::ManualUnwrapOrDefault
        | ManualLint::ManualOptExpect => match_arm_returns_extracted_var(expr, pattern, arenas),

        _ => false,
    }
}

/// Checks that the variant of the expression is named exactly the provided string.
/// This checks for the full path for example `core::option::Option::Some`
fn is_expected_variant<'db>(
    expr: &Expr<'db>,
    db: &'db dyn SemanticGroup,
    expected_variant: &str,
) -> bool {
    let Some(variant_name) = get_variant_name(expr, db) else {
        return false;
    };
    variant_name == expected_variant
}

/// Returns the variant of the expression is named exactly the provided string.
/// This returns the full path for example `core::option::Option::Some`
fn get_variant_name<'db>(expr: &Expr<'db>, db: &'db dyn SemanticGroup) -> Option<String> {
    let Expr::EnumVariantCtor(maybe_bool) = expr else {
        return None;
    };
    Some(maybe_bool.variant.id.full_path(db))
}

// Checks the `Result::Ok` arm
fn check_syntax_ok_arm<'db>(
    expr: &Expr<'db>,
    pattern: &Pattern<'db>,
    db: &'db dyn SemanticGroup,
    arenas: &Arenas<'db>,
    manual_lint: ManualLint,
) -> bool {
    match manual_lint {
        ManualLint::ManualIsOk => is_expected_variant(expr, db, TRUE),
        ManualLint::ManualIsErr => is_expected_variant(expr, db, FALSE),
        ManualLint::ManualOk => {
            is_destructured_variable_used_and_expected_variant(expr, pattern, db, arenas, SOME)
        }

        ManualLint::ManualErr => is_expected_variant(expr, db, NONE),
        ManualLint::ManualExpectErr => {
            if let Expr::FunctionCall(func_call) = &expr {
                let func_name = func_call.function.full_path(db);
                func_name == PANIC_WITH_FELT252
            } else {
                false
            }
        }
        ManualLint::ManualResExpect
        | ManualLint::ManualUnwrapOr
        | ManualLint::ManualUnwrapOrDefault => {
            match_arm_returns_extracted_var(expr, pattern, arenas)
        }
        _ => false,
    }
}

/// Checks `Option::None` arm
fn check_syntax_none_arm<'db>(
    expr: &Expr<'db>,
    _pattern: &Pattern<'db>,
    db: &'db dyn SemanticGroup,
    arenas: &Arenas<'db>,
    manual_lint: ManualLint,
) -> bool {
    match manual_lint {
        ManualLint::ManualOkOr => is_expected_variant(expr, db, ERR),
        ManualLint::ManualIsSome => is_expected_variant(expr, db, FALSE),
        ManualLint::ManualIsNone => is_expected_variant(expr, db, TRUE),
        ManualLint::ManualOptExpect => {
            if let Expr::FunctionCall(func_call) = &expr {
                let func_name = func_call.function.full_path(db);
                func_name == PANIC_WITH_FELT252
            } else {
                false
            }
        }
        ManualLint::ManualUnwrapOrDefault => check_is_default(db, expr, arenas),
        ManualLint::ManualUnwrapOr => {
            !func_call_or_block_returns_never(expr, db, arenas)
                && !check_is_default(db, expr, arenas)
        }
        _ => false,
    }
}

/// Checks `Result::Err` arm
fn check_syntax_err_arm<'db>(
    expr: &Expr<'db>,
    pattern: &Pattern<'db>,
    db: &'db dyn SemanticGroup,
    arenas: &Arenas<'db>,
    manual_lint: ManualLint,
) -> bool {
    match manual_lint {
        ManualLint::ManualIsOk => is_expected_variant(expr, db, FALSE),
        ManualLint::ManualIsErr => is_expected_variant(expr, db, TRUE),
        ManualLint::ManualOk => is_expected_variant(expr, db, NONE),
        ManualLint::ManualErr => {
            is_destructured_variable_used_and_expected_variant(expr, pattern, db, arenas, SOME)
        }
        ManualLint::ManualResExpect => {
            if let Expr::FunctionCall(func_call) = &expr {
                let func_name = func_call.function.full_path(db);
                if func_name != PANIC_WITH_FELT252 {
                    return false;
                }
                let Some(error_pattern_variable) = extract_pattern_variable(pattern, arenas) else {
                    return true;
                };

                return is_variable_unused(db, &error_pattern_variable.var)
                    || error_pattern_variable.name.starts_with("_");
            }
            false
        }
        ManualLint::ManualExpectErr => match_arm_returns_extracted_var(expr, pattern, arenas),
        ManualLint::ManualUnwrapOrDefault => check_is_default(db, expr, arenas),
        ManualLint::ManualUnwrapOr => {
            !func_call_or_block_returns_never(expr, db, arenas)
                && !check_is_default(db, expr, arenas)
        }
        _ => false,
    }
}

/// Checks for manual implementation as `if-let`. For example manual `ok()`
/// ```ignore
/// let _a = if let Result::Ok(x) = res_val {
///     Option::Some(x)
/// } else {
///     Option::None
/// };
/// ```
pub fn check_manual_if<'db>(
    db: &'db dyn SemanticGroup,
    expr: &ExprIf<'db>,
    arenas: &Arenas<'db>,
    manual_lint: ManualLint,
) -> bool {
    if_chain! {
        if let Some(Condition::Let(_condition_let, patterns)) = &expr.conditions.first();
        if let Pattern::EnumVariant(enum_pattern) = &arenas.patterns[patterns[0]];
        then {
            let enum_name = enum_pattern.variant.id.full_path(db);
            match enum_name.as_str() {
                SOME => {
                    let found_if = check_syntax_opt_if(expr, db, arenas, manual_lint);
                    let found_else = check_syntax_opt_else(expr, db, arenas, manual_lint);
                    return found_if && found_else;
                }
                OK => {
                    let found_if = check_syntax_res_if(expr, db, arenas, manual_lint);
                    let found_else = check_syntax_res_else(expr, db, arenas, manual_lint);
                    return found_if && found_else;
                }
                ERR => {
                    let found_if = check_syntax_err_if(expr, db, arenas, manual_lint);
                    let found_else = check_syntax_err_else(expr, db, arenas, manual_lint);
                    return found_if && found_else;
                }
                _ => return false,
            }
        }
    }
    false
}

fn check_syntax_opt_if<'db>(
    expr: &ExprIf<'db>,
    db: &'db dyn SemanticGroup,
    arenas: &Arenas<'db>,
    manual_lint: ManualLint,
) -> bool {
    let Expr::Block(if_block) = &arenas.exprs[expr.if_block] else {
        return false;
    };
    if !if_block.statements.is_empty() {
        return false;
    };
    let Some(tail_expr_id) = if_block.tail else {
        return false;
    };

    match manual_lint {
        ManualLint::ManualOkOr => {
            if_expr_condition_and_block_match_enum_pattern(expr, db, arenas, OK)
        }
        ManualLint::ManualIsSome => is_expected_variant(&arenas.exprs[tail_expr_id], db, TRUE),
        ManualLint::ManualIsNone => is_expected_variant(&arenas.exprs[tail_expr_id], db, FALSE),
        ManualLint::ManualOptExpect => if_expr_pattern_matches_tail_var(expr, arenas),
        ManualLint::ManualUnwrapOrDefault => if_expr_pattern_matches_tail_var(expr, arenas),
        ManualLint::ManualUnwrapOr => if_expr_pattern_matches_tail_var(expr, arenas),
        _ => false,
    }
}

fn check_syntax_res_if<'db>(
    expr: &ExprIf<'db>,
    db: &'db dyn SemanticGroup,
    arenas: &Arenas<'db>,
    manual_lint: ManualLint,
) -> bool {
    let Expr::Block(if_block) = &arenas.exprs[expr.if_block] else {
        return false;
    };
    if !if_block.statements.is_empty() {
        return false;
    };
    let Some(tail_expr_id) = if_block.tail else {
        return false;
    };
    match manual_lint {
        ManualLint::ManualIsOk => is_expected_variant(&arenas.exprs[tail_expr_id], db, TRUE),
        ManualLint::ManualIsErr => is_expected_variant(&arenas.exprs[tail_expr_id], db, FALSE),
        ManualLint::ManualOk => {
            if_expr_condition_and_block_match_enum_pattern(expr, db, arenas, SOME)
        }
        ManualLint::ManualResExpect => if_expr_pattern_matches_tail_var(expr, arenas),
        ManualLint::ManualUnwrapOr => if_expr_pattern_matches_tail_var(expr, arenas),
        ManualLint::ManualUnwrapOrDefault => if_expr_pattern_matches_tail_var(expr, arenas),
        _ => false,
    }
}

fn check_syntax_err_if<'db>(
    expr: &ExprIf<'db>,
    db: &'db dyn SemanticGroup,
    arenas: &Arenas<'db>,
    manual_lint: ManualLint,
) -> bool {
    match manual_lint {
        ManualLint::ManualErr => {
            if_expr_condition_and_block_match_enum_pattern(expr, db, arenas, SOME)
        }
        ManualLint::ManualExpectErr => if_expr_pattern_matches_tail_var(expr, arenas),
        _ => false,
    }
}

fn check_syntax_opt_else<'db>(
    expr: &ExprIf<'db>,
    db: &'db dyn SemanticGroup,
    arenas: &Arenas<'db>,
    manual_lint: ManualLint,
) -> bool {
    let expr_block = match expr.else_block {
        Some(block) => {
            let Expr::Block(ref block) = arenas.exprs[block] else {
                return false;
            };
            block
        }
        None => return false,
    };
    if !expr_block.statements.is_empty() {
        return false;
    };
    let Some(tail_expr_id) = expr_block.tail else {
        return false;
    };
    let tail_expr = &arenas.exprs[tail_expr_id];

    match manual_lint {
        ManualLint::ManualOkOr => is_expected_variant(&arenas.exprs[tail_expr_id], db, ERR),
        ManualLint::ManualIsSome => is_expected_variant(&arenas.exprs[tail_expr_id], db, FALSE),
        ManualLint::ManualIsNone => is_expected_variant(&arenas.exprs[tail_expr_id], db, TRUE),
        ManualLint::ManualOptExpect => is_expected_function(tail_expr, db, PANIC_WITH_FELT252),
        ManualLint::ManualUnwrapOrDefault => check_is_default(db, tail_expr, arenas),
        ManualLint::ManualUnwrapOr => {
            !check_is_default(db, tail_expr, arenas)
                && !func_call_or_block_returns_never(tail_expr, db, arenas)
        }
        _ => false,
    }
}

fn check_syntax_res_else<'db>(
    expr: &ExprIf<'db>,
    db: &'db dyn SemanticGroup,
    arenas: &Arenas<'db>,
    manual_lint: ManualLint,
) -> bool {
    let expr_block = match expr.else_block {
        Some(block) => {
            let Expr::Block(ref block) = arenas.exprs[block] else {
                return false;
            };
            block
        }
        None => return false,
    };
    if !expr_block.statements.is_empty() {
        return false;
    };
    let Some(tail_expr_id) = expr_block.tail else {
        return false;
    };

    let tail_expr = &arenas.exprs[tail_expr_id];

    match manual_lint {
        ManualLint::ManualIsOk => is_expected_variant(tail_expr, db, FALSE),
        ManualLint::ManualIsErr => is_expected_variant(tail_expr, db, TRUE),
        ManualLint::ManualOk => is_expected_variant(tail_expr, db, NONE),
        ManualLint::ManualResExpect => is_expected_function(tail_expr, db, PANIC_WITH_FELT252),
        ManualLint::ManualUnwrapOr => {
            !check_is_default(db, tail_expr, arenas)
                && !func_call_or_block_returns_never(tail_expr, db, arenas)
        }
        ManualLint::ManualUnwrapOrDefault => check_is_default(db, tail_expr, arenas),
        _ => false,
    }
}

fn check_syntax_err_else<'db>(
    expr: &ExprIf<'db>,
    db: &'db dyn SemanticGroup,
    arenas: &Arenas<'db>,
    manual_lint: ManualLint,
) -> bool {
    let expr_block = match expr.else_block {
        Some(block) => {
            let Expr::Block(ref block) = arenas.exprs[block] else {
                return false;
            };
            block
        }
        None => return false,
    };
    if !expr_block.statements.is_empty() {
        return false;
    };
    let Some(tail_expr_id) = expr_block.tail else {
        return false;
    };
    match manual_lint {
        ManualLint::ManualErr => is_expected_variant(&arenas.exprs[tail_expr_id], db, NONE),
        ManualLint::ManualExpectErr => {
            is_expected_function(&arenas.exprs[tail_expr_id], db, PANIC_WITH_FELT252)
        }
        _ => false,
    }
}
