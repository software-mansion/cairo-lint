pub mod helpers;
pub mod manual_assert;
pub mod manual_err;
pub mod manual_expect;
pub mod manual_expect_err;
pub mod manual_is;
pub mod manual_ok;
pub mod manual_ok_or;
pub mod manual_unwrap_or;
pub mod manual_unwrap_or_default;

use std::fmt::Debug;

use cairo_lang_defs::ids::TopLevelLanguageElementId;
use cairo_lang_semantic::db::SemanticGroup;
use cairo_lang_semantic::{Arenas, Condition, Expr, ExprId, ExprIf, ExprMatch, MatchArm, Pattern};
use helpers::{
    check_is_default, func_call_or_block_returns_never,
    if_expr_condition_and_block_match_enum_pattern, if_expr_pattern_matches_tail_var,
    is_destructured_variable_used_and_expected_variant, is_expected_function,
    match_arm_returns_extracted_var, match_with_single_statement_or_empty, pattern_check_enum_arg,
};
use if_chain::if_chain;

use super::{FALSE, OK, PANIC_WITH_FELT252, TRUE};
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
}

/// Checks for all the manual lint written as `match`.
/// ```ignore
/// let res_val: Result<i32> = Result::Err('err');
/// let _a = match res_val {
///     Result::Ok(x) => Option::Some(x),
///     Result::Err(_) => Option::None,
/// };
/// ```
pub fn check_manual(
    db: &dyn SemanticGroup,
    expr_match: &ExprMatch,
    arenas: &Arenas,
    manual_lint: ManualLint,
) -> bool {
    // All the manual lints are for options and results which only have 2 variants
    if expr_match.arms.len() != 2 {
        return false;
    }

    let first_arm = &expr_match.arms[0];
    let second_arm = &expr_match.arms[1];

    let found_first_arm = match &arenas.patterns[first_arm.patterns[0]] {
        Pattern::EnumVariant(enum_pattern) => {
            // Checks if we are in the option case or result.
            let enum_name = enum_pattern.variant.id.full_path(db.upcast());
            match enum_name.as_str() {
                SOME => check_syntax_some_arm(first_arm, db, arenas, manual_lint),
                OK => check_syntax_ok_arm(first_arm, db, arenas, manual_lint),
                _ => return false,
            }
        }
        _ => return false,
    };

    let found_second_arm = match &arenas.patterns[second_arm.patterns[0]] {
        Pattern::EnumVariant(enum_pattern) => {
            // Checks if we are in the option case or result.
            let enum_name = enum_pattern.variant.id.full_path(db.upcast());
            match enum_name.as_str() {
                NONE => check_syntax_none_arm(expr_match, db, arenas, manual_lint),
                ERR => check_syntax_err_arm(expr_match, db, arenas, manual_lint),
                _ => return false,
            }
        }
        _ => return false,
    };

    found_first_arm && found_second_arm
}

/// Checks the `Option::Some` arm in the match.
fn check_syntax_some_arm(
    arm: &MatchArm,
    db: &dyn SemanticGroup,
    arenas: &Arenas,
    manual_lint: ManualLint,
) -> bool {
    match manual_lint {
        ManualLint::ManualOkOr => is_destructured_variable_used_and_expected_variant(
            &arenas.exprs[arm.expression],
            &arenas.patterns[arm.patterns[0]],
            db,
            arenas,
            OK,
        ),
        ManualLint::ManualIsSome => is_expected_variant(&arm.expression, arenas, db, TRUE),
        ManualLint::ManualIsNone => is_expected_variant(&arm.expression, arenas, db, FALSE),
        ManualLint::ManualOptExpect => {
            let Expr::Var(expr_var) = &arenas.exprs[arm.expression] else {
                return false;
            };
            pattern_check_enum_arg(&arenas.patterns[arm.patterns[0]], &expr_var.var, arenas)
        }
        ManualLint::ManualUnwrapOr | ManualLint::ManualUnwrapOrDefault => {
            match_arm_returns_extracted_var(arm, arenas)
        }

        _ => false,
    }
}

/// Checks that the variant of the expression is named exactly the provided string.
/// This checks for the full path for example `core::option::Option::Some`
fn is_expected_variant(
    expr: &ExprId,
    arenas: &Arenas,
    db: &dyn SemanticGroup,
    expected_variant: &str,
) -> bool {
    let Some(variant_name) = get_variant_name(expr, arenas, db) else {
        return false;
    };
    variant_name == expected_variant
}

/// Returns the variant of the expression is named exactly the provided string.
/// This returns the full path for example `core::option::Option::Some`
fn get_variant_name(expr: &ExprId, arenas: &Arenas, db: &dyn SemanticGroup) -> Option<String> {
    let Expr::EnumVariantCtor(maybe_bool) = &arenas.exprs[*expr] else {
        return None;
    };
    Some(maybe_bool.variant.id.full_path(db.upcast()))
}

// Checks the `Result::Ok` arm
fn check_syntax_ok_arm(
    arm: &MatchArm,
    db: &dyn SemanticGroup,
    arenas: &Arenas,
    manual_lint: ManualLint,
) -> bool {
    match manual_lint {
        ManualLint::ManualIsOk => is_expected_variant(&arm.expression, arenas, db, TRUE),
        ManualLint::ManualIsErr => is_expected_variant(&arm.expression, arenas, db, FALSE),
        ManualLint::ManualOk => is_destructured_variable_used_and_expected_variant(
            &arenas.exprs[arm.expression],
            &arenas.patterns[arm.patterns[0]],
            db,
            arenas,
            SOME,
        ),

        ManualLint::ManualErr => is_expected_variant(&arm.expression, arenas, db, NONE),
        ManualLint::ManualResExpect => {
            let Expr::Var(expr_var) = &arenas.exprs[arm.expression] else {
                return false;
            };
            pattern_check_enum_arg(&arenas.patterns[arm.patterns[0]], &expr_var.var, arenas)
        }
        ManualLint::ManualExpectErr => {
            if let Expr::FunctionCall(func_call) = &arenas.exprs[arm.expression] {
                let func_name = func_call.function.full_path(db);
                func_name == PANIC_WITH_FELT252
            } else {
                false
            }
        }
        ManualLint::ManualUnwrapOr | ManualLint::ManualUnwrapOrDefault => {
            match_arm_returns_extracted_var(arm, arenas)
        }
        _ => false,
    }
}

/// Checks `Option::None` arm
fn check_syntax_none_arm(
    expr_match: &ExprMatch,
    db: &dyn SemanticGroup,
    arenas: &Arenas,
    manual_lint: ManualLint,
) -> bool {
    let arm = &expr_match.arms[1];

    match manual_lint {
        ManualLint::ManualOkOr => is_expected_variant(&arm.expression, arenas, db, ERR),
        ManualLint::ManualIsSome => is_expected_variant(&arm.expression, arenas, db, FALSE),
        ManualLint::ManualIsNone => is_expected_variant(&arm.expression, arenas, db, TRUE),
        ManualLint::ManualOptExpect => {
            if let Expr::FunctionCall(func_call) = &arenas.exprs[arm.expression] {
                let func_name = func_call.function.full_path(db);
                func_name == PANIC_WITH_FELT252
            } else {
                false
            }
        }
        ManualLint::ManualUnwrapOrDefault => {
            check_is_default(db, &arenas.exprs[arm.expression], arenas)
        }
        ManualLint::ManualUnwrapOr => {
            let expr = &arenas.exprs[arm.expression];

            match_with_single_statement_or_empty(expr_match, db, 1)
                && !func_call_or_block_returns_never(expr, db, arenas)
                && !check_is_default(db, expr, arenas)
        }
        _ => false,
    }
}

/// Checks `Result::Err` arm
fn check_syntax_err_arm(
    expr_match: &ExprMatch,
    db: &dyn SemanticGroup,
    arenas: &Arenas,
    manual_lint: ManualLint,
) -> bool {
    let arm = &expr_match.arms[1];

    match manual_lint {
        ManualLint::ManualIsOk => is_expected_variant(&arm.expression, arenas, db, FALSE),
        ManualLint::ManualIsErr => is_expected_variant(&arm.expression, arenas, db, TRUE),
        ManualLint::ManualOk => is_expected_variant(&arm.expression, arenas, db, NONE),
        ManualLint::ManualErr => is_destructured_variable_used_and_expected_variant(
            &arenas.exprs[arm.expression],
            &arenas.patterns[arm.patterns[0]],
            db,
            arenas,
            SOME,
        ),
        ManualLint::ManualResExpect => {
            if let Expr::FunctionCall(func_call) = &arenas.exprs[arm.expression] {
                let func_name = func_call.function.full_path(db);
                func_name == PANIC_WITH_FELT252
            } else {
                false
            }
        }
        ManualLint::ManualExpectErr => {
            let Expr::Var(return_err_var) = &arenas.exprs[arm.expression] else {
                return false;
            };
            pattern_check_enum_arg(
                &arenas.patterns[arm.patterns[0]],
                &return_err_var.var,
                arenas,
            )
        }

        ManualLint::ManualUnwrapOr => {
            let expr = &arenas.exprs[arm.expression];

            match_with_single_statement_or_empty(expr_match, db, 1)
                && !func_call_or_block_returns_never(expr, db, arenas)
                && !check_is_default(db, expr, arenas)
        }

        ManualLint::ManualUnwrapOrDefault => {
            check_is_default(db, &arenas.exprs[arm.expression], arenas)
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
pub fn check_manual_if(
    db: &dyn SemanticGroup,
    expr: &ExprIf,
    arenas: &Arenas,
    manual_lint: ManualLint,
) -> bool {
    if_chain! {
        if let Condition::Let(_condition_let, patterns) = &expr.condition;
        if let Pattern::EnumVariant(enum_pattern) = &arenas.patterns[patterns[0]];
        then {
            let enum_name = enum_pattern.variant.id.full_path(db.upcast());
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

fn check_syntax_opt_if(
    expr: &ExprIf,
    db: &dyn SemanticGroup,
    arenas: &Arenas,
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
        ManualLint::ManualIsSome => is_expected_variant(&tail_expr_id, arenas, db, TRUE),
        ManualLint::ManualIsNone => is_expected_variant(&tail_expr_id, arenas, db, FALSE),
        ManualLint::ManualOptExpect => if_expr_pattern_matches_tail_var(expr, arenas),
        ManualLint::ManualUnwrapOrDefault => if_expr_pattern_matches_tail_var(expr, arenas),
        ManualLint::ManualUnwrapOr => if_expr_pattern_matches_tail_var(expr, arenas),
        _ => false,
    }
}

fn check_syntax_res_if(
    expr: &ExprIf,
    db: &dyn SemanticGroup,
    arenas: &Arenas,
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
        ManualLint::ManualIsOk => is_expected_variant(&tail_expr_id, arenas, db, TRUE),
        ManualLint::ManualIsErr => is_expected_variant(&tail_expr_id, arenas, db, FALSE),
        ManualLint::ManualOk => {
            if_expr_condition_and_block_match_enum_pattern(expr, db, arenas, SOME)
        }
        ManualLint::ManualResExpect => if_expr_pattern_matches_tail_var(expr, arenas),
        ManualLint::ManualUnwrapOr => if_expr_pattern_matches_tail_var(expr, arenas),
        ManualLint::ManualUnwrapOrDefault => if_expr_pattern_matches_tail_var(expr, arenas),
        _ => false,
    }
}

fn check_syntax_err_if(
    expr: &ExprIf,
    db: &dyn SemanticGroup,
    arenas: &Arenas,
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

fn check_syntax_opt_else(
    expr: &ExprIf,
    db: &dyn SemanticGroup,
    arenas: &Arenas,
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
        ManualLint::ManualOkOr => is_expected_variant(&tail_expr_id, arenas, db, ERR),
        ManualLint::ManualIsSome => is_expected_variant(&tail_expr_id, arenas, db, FALSE),
        ManualLint::ManualIsNone => is_expected_variant(&tail_expr_id, arenas, db, TRUE),
        ManualLint::ManualOptExpect => is_expected_function(tail_expr, db, PANIC_WITH_FELT252),
        ManualLint::ManualUnwrapOrDefault => check_is_default(db, tail_expr, arenas),
        ManualLint::ManualUnwrapOr => {
            !check_is_default(db, tail_expr, arenas)
                && !func_call_or_block_returns_never(tail_expr, db, arenas)
        }
        _ => false,
    }
}

fn check_syntax_res_else(
    expr: &ExprIf,
    db: &dyn SemanticGroup,
    arenas: &Arenas,
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
        ManualLint::ManualIsOk => is_expected_variant(&tail_expr_id, arenas, db, FALSE),
        ManualLint::ManualIsErr => is_expected_variant(&tail_expr_id, arenas, db, TRUE),
        ManualLint::ManualOk => is_expected_variant(&tail_expr_id, arenas, db, NONE),
        ManualLint::ManualResExpect => {
            is_expected_function(&arenas.exprs[tail_expr_id], db, PANIC_WITH_FELT252)
        }
        ManualLint::ManualUnwrapOr => {
            !check_is_default(db, tail_expr, arenas)
                && !func_call_or_block_returns_never(tail_expr, db, arenas)
        }
        ManualLint::ManualUnwrapOrDefault => check_is_default(db, tail_expr, arenas),
        _ => false,
    }
}

fn check_syntax_err_else(
    expr: &ExprIf,
    db: &dyn SemanticGroup,
    arenas: &Arenas,
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
        ManualLint::ManualErr => is_expected_variant(&tail_expr_id, arenas, db, NONE),
        ManualLint::ManualExpectErr => {
            is_expected_function(&arenas.exprs[tail_expr_id], db, PANIC_WITH_FELT252)
        }
        _ => false,
    }
}
