use cairo_lang_defs::{ids::ModuleItemId, plugin::PluginDiagnostic};
use cairo_lang_semantic::db::SemanticGroup;
use cairo_lang_syntax::node::{db::SyntaxGroup, SyntaxNode};
use once_cell::sync::Lazy;
use std::sync::Arc;

use crate::{fixes, lints};

/// Type describing a linter's rule fixing function.
type FixingFunction =
    Arc<dyn Fn(&dyn SyntaxGroup, SyntaxNode) -> Option<(SyntaxNode, String)> + Send + Sync>;

/// Type describing a linter group's rule cheking function.
type CheckingFunction =
    Arc<dyn Fn(&dyn SemanticGroup, &ModuleItemId, &mut Vec<PluginDiagnostic>) + Send + Sync>;

/// Enum representing the kind of a linter. Some lint rules might have the same kind.
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
    EqualityOperation,
    Performance,
}

/// A group of lint rules. We want to group lint rules beucase
/// some lint rules can share an allowed name for compiler or the cheking function.
pub struct LintRuleGroup {
    /// Collection of `LintRule`s that are directly connected to this group's checking function.
    rules: Vec<LintRule>,
    /// A Function which will be fired during linter plugin analysis.
    /// This one should emit certain diagnostics in order to later identify (and maybe fix) the linting problem.
    check_function: CheckingFunction,
    /// A name that is going to be registered by the compiler as an allowed lint to be ignored.
    /// Some multiple lint rules might have the same allowed name. This way all of the will be ignored with only one allow attribute.
    allowed_name: &'static str,
}

/// Core struct describing a single lint rule with its properties.
pub struct LintRule {
    /// A predefined message that is going to appear in the compiler's diagnostic output. It should be the same as the one in the lint check function.
    diagnostic_message: &'static str,
    /// The kind of the lint rule. Some lint rules might have the same kind.
    kind: CairoLintKind,
    /// The fixing function for the lint rule. It is optional as not all lint rules have a fix.
    fix_function: Option<FixingFunction>,
    // pub check_function: CheckingFunction,
}

/// A global Linter context. It contains all the lint rules.
pub struct LintContext {
    lint_rules: Vec<LintRuleGroup>,
}

impl LintContext {
    /// All of the predefined rules are stored here. If a new rule is added it should be added here as well.
    fn get_all_rules() -> Vec<LintRuleGroup> {
        vec![
            LintRuleGroup {
                rules: vec![
                    LintRule {
                        diagnostic_message: lints::single_match::DESTRUCT_MATCH,
                        kind: CairoLintKind::DestructMatch,
                        fix_function: Some(Arc::new(fixes::desctruct_match::fix_destruct_match)),
                    },
                    LintRule {
                        diagnostic_message: lints::single_match::MATCH_FOR_EQUALITY,
                        kind: CairoLintKind::MatchForEquality,
                        fix_function: None,
                    },
                ],
                allowed_name: lints::single_match::LINT_NAME,
                check_function: Arc::new(lints::single_match::check_single_matches),
            },
            LintRuleGroup {
                rules: vec![LintRule {
                    diagnostic_message: lints::double_parens::DOUBLE_PARENS,
                    kind: CairoLintKind::DoubleParens,
                    fix_function: Some(Arc::new(fixes::double_parens::fix_double_parens)),
                }],
                allowed_name: lints::double_parens::LINT_NAME,
                check_function: Arc::new(lints::double_parens::check_double_parens),
            },
            LintRuleGroup {
                rules: vec![
                    LintRule {
                        diagnostic_message: lints::double_comparison::REDUNDANT_COMPARISON,
                        kind: CairoLintKind::DoubleComparison,
                        fix_function: Some(Arc::new(
                            fixes::double_comparison::fix_double_comparison,
                        )),
                    },
                    LintRule {
                        diagnostic_message: lints::double_comparison::SIMPLIFIABLE_COMPARISON,
                        kind: CairoLintKind::DoubleComparison,
                        fix_function: Some(Arc::new(
                            fixes::double_comparison::fix_double_comparison,
                        )),
                    },
                    LintRule {
                        diagnostic_message: lints::double_comparison::CONTRADICTORY_COMPARISON,
                        kind: CairoLintKind::DoubleComparison,
                        fix_function: Some(Arc::new(
                            fixes::double_comparison::fix_double_comparison,
                        )),
                    },
                    LintRule {
                        diagnostic_message: lints::double_comparison::IMPOSSIBLE_COMPARISON,
                        kind: CairoLintKind::ImposibleComparison,
                        fix_function: None,
                    },
                ],
                allowed_name: lints::double_comparison::ALLOWED_NAME,
                check_function: Arc::new(lints::double_comparison::check_double_comparison),
            },
            LintRuleGroup {
                rules: vec![LintRule {
                    diagnostic_message: lints::ifs::equatable_if_let::EQUATABLE_IF_LET,
                    kind: CairoLintKind::EquatableIfLet,
                    fix_function: Some(Arc::new(
                        fixes::ifs::equatable_if_let::fix_equatable_if_let,
                    )),
                }],
                allowed_name: lints::ifs::equatable_if_let::LINT_NAME,
                check_function: Arc::new(lints::ifs::equatable_if_let::check_equatable_if_let),
            },
            LintRuleGroup {
                rules: vec![LintRule {
                    diagnostic_message: lints::breaks::BREAK_UNIT,
                    kind: CairoLintKind::BreakUnit,
                    fix_function: Some(Arc::new(fixes::break_unit::fix_break_unit)),
                }],
                allowed_name: lints::breaks::LINT_NAME,
                check_function: Arc::new(lints::breaks::check_break),
            },
            LintRuleGroup {
                rules: vec![LintRule {
                    diagnostic_message: lints::bool_comparison::BOOL_COMPARISON,
                    kind: CairoLintKind::BoolComparison,
                    fix_function: Some(Arc::new(
                        fixes::comparisons::bool_comparison::fix_bool_comparison,
                    )),
                }],
                allowed_name: lints::bool_comparison::LINT_NAME,
                check_function: Arc::new(lints::bool_comparison::check_bool_comparison),
            },
            LintRuleGroup {
                rules: vec![LintRule {
                    diagnostic_message: lints::ifs::collapsible_if_else::COLLAPSIBLE_IF_ELSE,
                    kind: CairoLintKind::CollapsibleIfElse,
                    fix_function: Some(Arc::new(
                        fixes::ifs::collapsible_if_else::fix_collapsible_if_else,
                    )),
                }],
                allowed_name: lints::ifs::collapsible_if_else::LINT_NAME,
                check_function: Arc::new(
                    lints::ifs::collapsible_if_else::check_collapsible_if_else,
                ),
            },
            LintRuleGroup {
                rules: vec![LintRule {
                    diagnostic_message: lints::ifs::collapsible_if::COLLAPSIBLE_IF,
                    kind: CairoLintKind::CollapsibleIf,
                    fix_function: Some(Arc::new(fixes::ifs::collapsible_if::fix_collapsible_if)),
                }],
                allowed_name: lints::ifs::collapsible_if::LINT_NAME,
                check_function: Arc::new(lints::ifs::collapsible_if::check_collapsible_if),
            },
            LintRuleGroup {
                rules: vec![LintRule {
                    diagnostic_message: lints::duplicate_underscore_args::DUPLICATE_UNDERSCORE_ARGS,
                    kind: CairoLintKind::DuplicateUnderscoreArgs,
                    fix_function: None,
                }],
                allowed_name: lints::duplicate_underscore_args::LINT_NAME,
                check_function: Arc::new(
                    lints::duplicate_underscore_args::check_duplicate_underscore_args,
                ),
            },
            LintRuleGroup {
                rules: vec![LintRule {
                    diagnostic_message: lints::loops::loop_match_pop_front::LOOP_MATCH_POP_FRONT,
                    kind: CairoLintKind::LoopMatchPopFront,
                    fix_function: Some(Arc::new(
                        fixes::loops::loop_match_pop_front::fix_loop_match_pop_front,
                    )),
                }],
                allowed_name: lints::loops::loop_match_pop_front::LINT_NAME,
                check_function: Arc::new(
                    lints::loops::loop_match_pop_front::check_loop_match_pop_front,
                ),
            },
            LintRuleGroup {
                rules: vec![LintRule {
                    diagnostic_message:
                        lints::manual::manual_unwrap_or_default::MANUAL_UNWRAP_OR_DEFAULT,
                    kind: CairoLintKind::ManualUnwrapOrDefault,
                    fix_function: Some(Arc::new(
                        fixes::manual::manual_unwrap_or_default::fix_manual_unwrap_or_default,
                    )),
                }],
                allowed_name: lints::manual::manual_unwrap_or_default::LINT_NAME,
                check_function: Arc::new(
                    lints::manual::manual_unwrap_or_default::check_manual_unwrap_or_default,
                ),
            },
            LintRuleGroup {
                rules: vec![LintRule {
                    diagnostic_message: lints::bitwise_for_parity_check::BITWISE_FOR_PARITY,
                    kind: CairoLintKind::BitwiseForParityCheck,
                    fix_function: None,
                }],
                allowed_name: lints::bitwise_for_parity_check::LINT_NAME,
                check_function: Arc::new(lints::bitwise_for_parity_check::check_bitwise_for_parity),
            },
            LintRuleGroup {
                rules: vec![LintRule {
                    diagnostic_message: lints::loops::loop_for_while::LOOP_FOR_WHILE,
                    kind: CairoLintKind::LoopForWhile,
                    fix_function: Some(Arc::new(fixes::loops::loop_break::fix_loop_break)),
                }],
                allowed_name: lints::loops::loop_for_while::LINT_NAME,
                check_function: Arc::new(lints::loops::loop_for_while::check_loop_for_while),
            },
            LintRuleGroup {
                rules: vec![LintRule {
                    diagnostic_message: lints::panic::PANIC_IN_CODE,
                    kind: CairoLintKind::Panic,
                    fix_function: None,
                }],
                allowed_name: lints::panic::LINT_NAME,
                check_function: Arc::new(lints::panic::check_panic_usage),
            },
            LintRuleGroup {
                rules: vec![LintRule {
                    diagnostic_message: lints::erasing_op::ERASING_OPERATION,
                    kind: CairoLintKind::ErasingOperation,
                    fix_function: None,
                }],
                allowed_name: lints::erasing_op::LINT_NAME,
                check_function: Arc::new(lints::erasing_op::check_erasing_operation),
            },
            LintRuleGroup {
                rules: vec![LintRule {
                    diagnostic_message: lints::manual::manual_ok_or::MANUAL_OK_OR,
                    kind: CairoLintKind::ManualOkOr,
                    fix_function: Some(Arc::new(fixes::manual::manual_ok_or::fix_manual_ok_or)),
                }],
                allowed_name: lints::manual::manual_ok_or::LINT_NAME,
                check_function: Arc::new(lints::manual::manual_ok_or::check_manual_ok_or),
            },
            LintRuleGroup {
                rules: vec![LintRule {
                    diagnostic_message: lints::manual::manual_ok::MANUAL_OK,
                    kind: CairoLintKind::ManualOk,
                    fix_function: Some(Arc::new(fixes::manual::manual_ok::fix_manual_ok)),
                }],
                allowed_name: lints::manual::manual_ok::LINT_NAME,
                check_function: Arc::new(lints::manual::manual_ok::check_manual_ok),
            },
            LintRuleGroup {
                rules: vec![LintRule {
                    diagnostic_message: lints::manual::manual_err::MANUAL_ERR,
                    kind: CairoLintKind::ManualErr,
                    fix_function: Some(Arc::new(fixes::manual::manual_err::fix_manual_err)),
                }],
                allowed_name: lints::manual::manual_err::LINT_NAME,
                check_function: Arc::new(lints::manual::manual_err::check_manual_err),
            },
            LintRuleGroup {
                rules: vec![
                    LintRule {
                        diagnostic_message: lints::manual::manual_is::MANUAL_IS_SOME,
                        kind: CairoLintKind::ManualIsSome,
                        fix_function: Some(Arc::new(
                            fixes::manual::manual_is_some::fix_manual_is_some,
                        )),
                    },
                    LintRule {
                        diagnostic_message: lints::manual::manual_is::MANUAL_IS_NONE,
                        kind: CairoLintKind::ManualIsNone,
                        fix_function: Some(Arc::new(
                            fixes::manual::manual_is_none::fix_manual_is_none,
                        )),
                    },
                    LintRule {
                        diagnostic_message: lints::manual::manual_is::MANUAL_IS_OK,
                        kind: CairoLintKind::ManualIsOk,
                        fix_function: Some(Arc::new(fixes::manual::manual_is_ok::fix_manual_is_ok)),
                    },
                    LintRule {
                        diagnostic_message: lints::manual::manual_is::MANUAL_IS_ERR,
                        kind: CairoLintKind::ManualIsErr,
                        fix_function: Some(Arc::new(
                            fixes::manual::manual_is_err::fix_manual_is_err,
                        )),
                    },
                ],
                allowed_name: lints::manual::manual_is::ALLOWED_NAME,
                check_function: Arc::new(lints::manual::manual_is::check_manual_is),
            },
            LintRuleGroup {
                rules: vec![LintRule {
                    diagnostic_message: lints::manual::manual_expect::MANUAL_EXPECT,
                    kind: CairoLintKind::ManualExpect,
                    fix_function: Some(Arc::new(fixes::manual::manual_expect::fix_manual_expect)),
                }],
                allowed_name: lints::manual::manual_expect::LINT_NAME,
                check_function: Arc::new(lints::manual::manual_expect::check_manual_expect),
            },
            LintRuleGroup {
                rules: vec![LintRule {
                    diagnostic_message: lints::ifs::ifs_same_cond::DUPLICATE_IF_CONDITION,
                    kind: CairoLintKind::DuplicateIfCondition,
                    fix_function: None,
                }],
                allowed_name: lints::ifs::ifs_same_cond::LINT_NAME,
                check_function: Arc::new(lints::ifs::ifs_same_cond::check_duplicate_if_condition),
            },
            LintRuleGroup {
                rules: vec![LintRule {
                    diagnostic_message: lints::manual::manual_expect_err::MANUAL_EXPECT_ERR,
                    kind: CairoLintKind::ManualExpectErr,
                    fix_function: Some(Arc::new(
                        fixes::manual::manual_expect_err::fix_manual_expect_err,
                    )),
                }],
                allowed_name: lints::manual::manual_expect_err::LINT_NAME,
                check_function: Arc::new(lints::manual::manual_expect_err::check_manual_expect_err),
            },
            LintRuleGroup {
                rules: vec![
                    LintRule {
                        diagnostic_message: lints::int_op_one::INT_GE_PLUS_ONE,
                        kind: CairoLintKind::IntGePlusOne,
                        fix_function: Some(Arc::new(
                            fixes::comparisons::int_ge_plus_one::fix_int_ge_plus_one,
                        )),
                    },
                    LintRule {
                        diagnostic_message: lints::int_op_one::INT_GE_MIN_ONE,
                        kind: CairoLintKind::IntGeMinOne,
                        fix_function: Some(Arc::new(
                            fixes::comparisons::int_ge_min_one::fix_int_ge_min_one,
                        )),
                    },
                    LintRule {
                        diagnostic_message: lints::int_op_one::INT_LE_PLUS_ONE,
                        kind: CairoLintKind::IntLePlusOne,
                        fix_function: Some(Arc::new(
                            fixes::comparisons::int_le_plus_one::fix_int_le_plus_one,
                        )),
                    },
                    LintRule {
                        diagnostic_message: lints::int_op_one::INT_LE_MIN_ONE,
                        kind: CairoLintKind::IntLeMinOne,
                        fix_function: Some(Arc::new(
                            fixes::comparisons::int_le_min_one::fix_int_le_min_one,
                        )),
                    },
                ],
                allowed_name: lints::int_op_one::LINT_NAME,
                check_function: Arc::new(lints::int_op_one::check_int_op_one),
            },
            LintRuleGroup {
                rules: vec![
                    LintRule {
                        diagnostic_message: lints::eq_op::DIV_EQ_OP,
                        kind: CairoLintKind::EqualityOperation,
                        fix_function: None,
                    },
                    LintRule {
                        diagnostic_message: lints::eq_op::EQ_COMP_OP,
                        kind: CairoLintKind::EqualityOperation,
                        fix_function: None,
                    },
                    LintRule {
                        diagnostic_message: lints::eq_op::NEQ_COMP_OP,
                        kind: CairoLintKind::EqualityOperation,
                        fix_function: None,
                    },
                    LintRule {
                        diagnostic_message: lints::eq_op::EQ_DIFF_OP,
                        kind: CairoLintKind::EqualityOperation,
                        fix_function: None,
                    },
                    LintRule {
                        diagnostic_message: lints::eq_op::EQ_BITWISE_OP,
                        kind: CairoLintKind::EqualityOperation,
                        fix_function: None,
                    },
                    LintRule {
                        diagnostic_message: lints::eq_op::EQ_LOGICAL_OP,
                        kind: CairoLintKind::EqualityOperation,
                        fix_function: None,
                    },
                ],
                allowed_name: lints::eq_op::LINT_NAME,
                check_function: Arc::new(lints::eq_op::check_eq_op),
            },
            LintRuleGroup {
                rules: vec![LintRule {
                    diagnostic_message: lints::performance::INEFFICIENT_WHILE_COMP_MESSAGE,
                    kind: CairoLintKind::Performance,
                    fix_function: None,
                }],
                allowed_name: lints::performance::LINT_NAME,
                check_function: Arc::new(lints::performance::check_inefficient_while_comp),
            },
        ]
    }

    fn new() -> Self {
        Self {
            lint_rules: Self::get_all_rules(),
        }
    }

    /// Get the lint type based on the diagnostic message.
    /// If the diagnostic message doesn't match any of the rules, it returns `CairoLintKind::Unknown`.
    pub fn get_lint_type_from_diagnostic_message(&self, message: &str) -> CairoLintKind {
        self.lint_rules
            .iter()
            .flat_map(|rule_group| &rule_group.rules)
            .find(|rule| rule.diagnostic_message == message)
            .map_or(CairoLintKind::Unknown, |rule| rule.kind)
    }

    /// Get the fixing function based on the diagnostic message.
    /// For some of the rules there is no fixing function, so it returns `None`.
    pub fn get_fixing_function_for_diagnostic_message(
        &self,
        message: &str,
    ) -> Option<FixingFunction> {
        self.lint_rules
            .iter()
            .flat_map(|rule_group| &rule_group.rules)
            .find(|rule| rule.diagnostic_message == message)
            .and_then(|rule| rule.fix_function.clone())
    }

    /// Get all the unique allowed names for the lint rule groups.
    pub fn get_unique_allowed_names(&self) -> Vec<&'static str> {
        self.lint_rules
            .iter()
            .map(|rule_group| rule_group.allowed_name)
            .collect()
    }

    /// Get all the checking functions that exist for each `LintRuleGroup`.
    pub fn get_all_checking_functions(&self) -> impl Iterator<Item = &CheckingFunction> {
        self.lint_rules
            .iter()
            .map(|rule_group| &rule_group.check_function)
    }
}

/// A singleton instance of the `LintContext`. It should be the only instance of the `LintContext`.
pub static LINT_CONTEXT: Lazy<LintContext> = Lazy::new(LintContext::new);
