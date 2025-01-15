use std::sync::Arc;

use cairo_lang_syntax::node::{db::SyntaxGroup, SyntaxNode};

pub mod diagnostics;
pub mod fixes;
pub mod lints;
pub mod plugin;

type FixingFunction = Arc<dyn Fn(&dyn SyntaxGroup, SyntaxNode) -> Option<(SyntaxNode, String)>>;

#[derive(Debug, PartialEq, Clone, Copy)]
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

pub struct LintRule {
    pub allowed_name: &'static str,
    pub diagnostic_message: &'static str,
    pub kind: CairoLintKind,
    pub fix_function: Option<FixingFunction>,
}

pub struct LintContext {
    lint_rules: Vec<LintRule>,
}

impl LintContext {
    fn get_all_rules() -> Vec<LintRule> {
        vec![
            LintRule {
                allowed_name: lints::single_match::LINT_NAME,
                diagnostic_message: lints::single_match::DESTRUCT_MATCH,
                kind: CairoLintKind::DestructMatch,
                fix_function: Some(Arc::new(fixes::desctruct_match::fix_destruct_match)),
            },
            LintRule {
                allowed_name: lints::single_match::LINT_NAME,
                diagnostic_message: lints::single_match::MATCH_FOR_EQUALITY,
                kind: CairoLintKind::MatchForEquality,
                fix_function: None,
            },
            LintRule {
                allowed_name: lints::double_parens::LINT_NAME,
                diagnostic_message: lints::double_parens::DOUBLE_PARENS,
                kind: CairoLintKind::DoubleParens,
                fix_function: Some(Arc::new(fixes::double_parens::fix_double_parens)),
            },
            LintRule {
                allowed_name: lints::ifs::equatable_if_let::LINT_NAME,
                diagnostic_message: lints::ifs::equatable_if_let::EQUATABLE_IF_LET,
                kind: CairoLintKind::EquatableIfLet,
                fix_function: Some(Arc::new(fixes::ifs::equatable_if_let::fix_equatable_if_let)),
            },
            LintRule {
                allowed_name: lints::breaks::LINT_NAME,
                diagnostic_message: lints::breaks::BREAK_UNIT,
                kind: CairoLintKind::BreakUnit,
                fix_function: Some(Arc::new(fixes::break_unit::fix_break_unit)),
            },
            LintRule {
                allowed_name: lints::bool_comparison::LINT_NAME,
                diagnostic_message: lints::bool_comparison::BOOL_COMPARISON,
                kind: CairoLintKind::BoolComparison,
                fix_function: Some(Arc::new(
                    fixes::comparisons::bool_comparison::fix_bool_comparison,
                )),
            },
            LintRule {
                allowed_name: lints::ifs::collapsible_if_else::LINT_NAME,
                diagnostic_message: lints::ifs::collapsible_if_else::COLLAPSIBLE_IF_ELSE,
                kind: CairoLintKind::CollapsibleIfElse,
                fix_function: Some(Arc::new(
                    fixes::ifs::collapsible_if_else::fix_collapsible_if_else,
                )),
            },
            LintRule {
                allowed_name: lints::ifs::collapsible_if::LINT_NAME,
                diagnostic_message: lints::ifs::collapsible_if::COLLAPSIBLE_IF,
                kind: CairoLintKind::CollapsibleIf,
                fix_function: Some(Arc::new(fixes::ifs::collapsible_if::fix_collapsible_if)),
            },
            LintRule {
                allowed_name: lints::duplicate_underscore_args::LINT_NAME,
                diagnostic_message: lints::duplicate_underscore_args::DUPLICATE_UNDERSCORE_ARGS,
                kind: CairoLintKind::DuplicateUnderscoreArgs,
                fix_function: None,
            },
            LintRule {
                allowed_name: lints::loops::loop_match_pop_front::LINT_NAME,
                diagnostic_message: lints::loops::loop_match_pop_front::LOOP_MATCH_POP_FRONT,
                kind: CairoLintKind::LoopMatchPopFront,
                fix_function: Some(Arc::new(
                    fixes::loops::loop_match_pop_front::fix_loop_match_pop_front,
                )),
            },
            LintRule {
                allowed_name: lints::manual::manual_unwrap_or_default::LINT_NAME,
                diagnostic_message:
                    lints::manual::manual_unwrap_or_default::MANUAL_UNWRAP_OR_DEFAULT,
                kind: CairoLintKind::ManualUnwrapOrDefault,
                fix_function: Some(Arc::new(
                    fixes::manual::manual_unwrap_or_default::fix_manual_unwrap_or_default,
                )),
            },
            LintRule {
                allowed_name: lints::bitwise_for_parity_check::LINT_NAME,
                diagnostic_message: lints::bitwise_for_parity_check::BITWISE_FOR_PARITY,
                kind: CairoLintKind::BitwiseForParityCheck,
                fix_function: None,
            },
            LintRule {
                allowed_name: lints::loops::loop_for_while::LINT_NAME,
                diagnostic_message: lints::loops::loop_for_while::LOOP_FOR_WHILE,
                kind: CairoLintKind::LoopForWhile,
                fix_function: Some(Arc::new(fixes::loops::loop_break::fix_loop_break)),
            },
            LintRule {
                allowed_name: lints::panic::LINT_NAME,
                diagnostic_message: lints::panic::PANIC_IN_CODE,
                kind: CairoLintKind::Panic,
                fix_function: None,
            },
            LintRule {
                allowed_name: lints::erasing_op::LINT_NAME,
                diagnostic_message: lints::erasing_op::ERASING_OPERATION,
                kind: CairoLintKind::ErasingOperation,
                fix_function: None,
            },
            LintRule {
                allowed_name: lints::manual::manual_ok_or::LINT_NAME,
                diagnostic_message: lints::manual::manual_ok_or::MANUAL_OK_OR,
                kind: CairoLintKind::ManualOkOr,
                fix_function: Some(Arc::new(fixes::manual::manual_ok_or::fix_manual_ok_or)),
            },
            LintRule {
                allowed_name: lints::manual::manual_ok::LINT_NAME,
                diagnostic_message: lints::manual::manual_ok::MANUAL_OK,
                kind: CairoLintKind::ManualOk,
                fix_function: Some(Arc::new(fixes::manual::manual_ok::fix_manual_ok)),
            },
            LintRule {
                allowed_name: lints::manual::manual_err::LINT_NAME,
                diagnostic_message: lints::manual::manual_err::MANUAL_ERR,
                kind: CairoLintKind::ManualErr,
                fix_function: Some(Arc::new(fixes::manual::manual_err::fix_manual_err)),
            },
            LintRule {
                allowed_name: lints::manual::manual_is::some::LINT_NAME,
                diagnostic_message: lints::manual::manual_is::MANUAL_IS_SOME,
                kind: CairoLintKind::ManualIsSome,
                fix_function: Some(Arc::new(fixes::manual::manual_is_some::fix_manual_is_some)),
            },
            LintRule {
                allowed_name: lints::manual::manual_is::none::LINT_NAME,
                diagnostic_message: lints::manual::manual_is::MANUAL_IS_NONE,
                kind: CairoLintKind::ManualIsNone,
                fix_function: Some(Arc::new(fixes::manual::manual_is_none::fix_manual_is_none)),
            },
            LintRule {
                allowed_name: lints::manual::manual_is::ok::LINT_NAME,
                diagnostic_message: lints::manual::manual_is::MANUAL_IS_OK,
                kind: CairoLintKind::ManualIsOk,
                fix_function: Some(Arc::new(fixes::manual::manual_is_ok::fix_manual_is_ok)),
            },
            LintRule {
                allowed_name: lints::manual::manual_is::err::LINT_NAME,
                diagnostic_message: lints::manual::manual_is::MANUAL_IS_ERR,
                kind: CairoLintKind::ManualIsErr,
                fix_function: Some(Arc::new(fixes::manual::manual_is_err::fix_manual_is_err)),
            },
            LintRule {
                allowed_name: lints::manual::manual_expect::LINT_NAME,
                diagnostic_message: lints::manual::manual_expect::MANUAL_EXPECT,
                kind: CairoLintKind::ManualExpect,
                fix_function: Some(Arc::new(fixes::manual::manual_expect::fix_manual_expect)),
            },
            LintRule {
                allowed_name: lints::ifs::ifs_same_cond::LINT_NAME,
                diagnostic_message: lints::ifs::ifs_same_cond::DUPLICATE_IF_CONDITION,
                kind: CairoLintKind::DuplicateIfCondition,
                fix_function: None,
            },
            LintRule {
                allowed_name: lints::manual::manual_expect_err::LINT_NAME,
                diagnostic_message: lints::manual::manual_expect_err::MANUAL_EXPECT_ERR,
                kind: CairoLintKind::ManualExpectErr,
                fix_function: Some(Arc::new(
                    fixes::manual::manual_expect_err::fix_manual_expect_err,
                )),
            },
            LintRule {
                allowed_name: lints::int_op_one::LINT_NAME,
                diagnostic_message: lints::int_op_one::INT_GE_PLUS_ONE,
                kind: CairoLintKind::IntGePlusOne,
                fix_function: Some(Arc::new(
                    fixes::comparisons::int_ge_plus_one::fix_int_ge_plus_one,
                )),
            },
            LintRule {
                allowed_name: lints::int_op_one::LINT_NAME,
                diagnostic_message: lints::int_op_one::INT_GE_MIN_ONE,
                kind: CairoLintKind::IntGeMinOne,
                fix_function: Some(Arc::new(
                    fixes::comparisons::int_ge_min_one::fix_int_ge_min_one,
                )),
            },
            LintRule {
                allowed_name: lints::int_op_one::LINT_NAME,
                diagnostic_message: lints::int_op_one::INT_LE_PLUS_ONE,
                kind: CairoLintKind::IntLePlusOne,
                fix_function: Some(Arc::new(
                    fixes::comparisons::int_le_plus_one::fix_int_le_plus_one,
                )),
            },
            LintRule {
                allowed_name: lints::int_op_one::LINT_NAME,
                diagnostic_message: lints::int_op_one::INT_LE_MIN_ONE,
                kind: CairoLintKind::IntLeMinOne,
                fix_function: Some(Arc::new(
                    fixes::comparisons::int_le_min_one::fix_int_le_min_one,
                )),
            },
        ]
    }

    pub fn new() -> Self {
        Self {
            lint_rules: Self::get_all_rules(),
        }
    }

    pub fn lint_type_from_diagnostic_message(&self, message: &str) -> CairoLintKind {
        for rule in &self.lint_rules {
            if rule.diagnostic_message == message {
                return rule.kind;
            }
        }
        CairoLintKind::Unknown
    }

    pub fn get_fixing_function_for_diagnostic_message(
        &self,
        message: &str,
    ) -> Option<FixingFunction> {
        self.lint_rules
            .iter()
            .find(|rule| rule.diagnostic_message == message)
            .and_then(|rule| rule.fix_function.clone())
    }
}
