use lints::{
    bitwise_for_parity_check, bool_comparison, breaks, double_comparison, double_parens,
    duplicate_underscore_args, erasing_op,
    ifs::{collapsible_if, collapsible_if_else, equatable_if_let, ifs_same_cond},
    int_op_one,
    loops::{loop_for_while, loop_match_pop_front},
    manual::{
        manual_err, manual_expect, manual_expect_err, manual_is, manual_ok, manual_ok_or,
        manual_unwrap_or_default,
    },
    panic, single_match,
};

pub mod diagnostics;
pub mod fixes;
pub mod lints;
pub mod plugin;

#[derive(Debug, PartialEq)]
pub enum CairoLintKind {
    DestructMatch,
    MatchForEquality,
    DoubleComparison,
    DoubleParens,
    EquatableIfLet,
    BreakUnit,
    BoolComparison,
    CollapsibleIfElse,
    CollapsibleIf,
    DuplicateUnderscoreArgs,
    LoopMatchPopFront,
    ManualUnwrapOrDefault,
    BitwiseForParityCheck,
    LoopForWhile,
    Unknown,
    Panic,
    ErasingOperation,
    ManualOkOr,
    ManualOk,
    ManualErr,
    ManualIsSome,
    ManualIsNone,
    ManualIsOk,
    ManualIsErr,
    ManualExpect,
    DuplicateIfCondition,
    ManualExpectErr,
    IntGePlusOne,
    IntGeMinOne,
    IntLePlusOne,
    IntLeMinOne,
    ImposibleComparison,
}

pub fn diagnostic_kind_from_message(message: &str) -> CairoLintKind {
    match message {
        single_match::DESTRUCT_MATCH => CairoLintKind::DestructMatch,
        single_match::MATCH_FOR_EQUALITY => CairoLintKind::MatchForEquality,
        double_parens::DOUBLE_PARENS => CairoLintKind::DoubleParens,
        double_comparison::SIMPLIFIABLE_COMPARISON => CairoLintKind::DoubleComparison,
        double_comparison::REDUNDANT_COMPARISON => CairoLintKind::DoubleComparison,
        double_comparison::CONTRADICTORY_COMPARISON => CairoLintKind::DoubleComparison,
        breaks::BREAK_UNIT => CairoLintKind::BreakUnit,
        equatable_if_let::EQUATABLE_IF_LET => CairoLintKind::EquatableIfLet,
        bool_comparison::BOOL_COMPARISON => CairoLintKind::BoolComparison,
        collapsible_if_else::COLLAPSIBLE_IF_ELSE => CairoLintKind::CollapsibleIfElse,
        duplicate_underscore_args::DUPLICATE_UNDERSCORE_ARGS => {
            CairoLintKind::DuplicateUnderscoreArgs
        }
        collapsible_if::COLLAPSIBLE_IF => CairoLintKind::CollapsibleIf,
        loop_match_pop_front::LOOP_MATCH_POP_FRONT => CairoLintKind::LoopMatchPopFront,
        manual_unwrap_or_default::MANUAL_UNWRAP_OR_DEFAULT => CairoLintKind::ManualUnwrapOrDefault,
        panic::PANIC_IN_CODE => CairoLintKind::Panic,
        loop_for_while::LOOP_FOR_WHILE => CairoLintKind::LoopForWhile,
        erasing_op::ERASING_OPERATION => CairoLintKind::ErasingOperation,
        manual_ok_or::MANUAL_OK_OR => CairoLintKind::ManualOkOr,
        manual_ok::MANUAL_OK => CairoLintKind::ManualOk,
        manual_err::MANUAL_ERR => CairoLintKind::ManualErr,
        bitwise_for_parity_check::BITWISE_FOR_PARITY => CairoLintKind::BitwiseForParityCheck,
        manual_is::MANUAL_IS_SOME => CairoLintKind::ManualIsSome,
        manual_is::MANUAL_IS_NONE => CairoLintKind::ManualIsNone,
        manual_is::MANUAL_IS_OK => CairoLintKind::ManualIsOk,
        manual_is::MANUAL_IS_ERR => CairoLintKind::ManualIsErr,
        manual_expect::MANUAL_EXPECT => CairoLintKind::ManualExpect,
        ifs_same_cond::DUPLICATE_IF_CONDITION => CairoLintKind::DuplicateIfCondition,
        manual_expect_err::MANUAL_EXPECT_ERR => CairoLintKind::ManualExpectErr,
        int_op_one::INT_GE_PLUS_ONE => CairoLintKind::IntGePlusOne,
        int_op_one::INT_GE_MIN_ONE => CairoLintKind::IntGeMinOne,
        int_op_one::INT_LE_PLUS_ONE => CairoLintKind::IntLePlusOne,
        int_op_one::INT_LE_MIN_ONE => CairoLintKind::IntLeMinOne,
        _ => CairoLintKind::Unknown,
    }
}
