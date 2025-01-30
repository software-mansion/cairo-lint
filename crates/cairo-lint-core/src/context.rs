use crate::lints;
use crate::lints::bitwise_for_parity_check::check_bitwise_for_parity;
use crate::lints::bitwise_for_parity_check::BitwiseForParity;
use crate::lints::bool_comparison::check_bool_comparison;
use crate::lints::bool_comparison::BoolComparison;
use crate::lints::breaks::check_break;
use crate::lints::breaks::BreakUnit;
use crate::lints::double_comparison::check_double_comparison;
use crate::lints::double_comparison::ContradictoryComparison;
use crate::lints::double_comparison::ImpossibleComparison;
use crate::lints::double_comparison::RedundantComparison;
use crate::lints::double_comparison::SimplifiableComparison;
use crate::lints::double_parens::check_double_parens;
use crate::lints::double_parens::DoubleParens;
use crate::lints::duplicate_underscore_args::check_duplicate_underscore_args;
use crate::lints::duplicate_underscore_args::DuplicateUnderscoreArgs;
use crate::lints::eq_op::check_eq_op;
use crate::lints::eq_op::BitwiseEqualityOperation;
use crate::lints::eq_op::DifferenceEqualityOperation;
use crate::lints::eq_op::DivisionEqualityOperation;
use crate::lints::eq_op::EqualComparisonOperation;
use crate::lints::eq_op::LogicalEqualityOperation;
use crate::lints::eq_op::NotEqualComparisonOperation;
use crate::lints::erasing_op::check_erasing_operation;
use crate::lints::erasing_op::ErasingOperation;
use crate::lints::ifs::collapsible_if::check_collapsible_if;
use crate::lints::ifs::collapsible_if::CollapsibleIf;
use crate::lints::ifs::collapsible_if_else::check_collapsible_if_else;
use crate::lints::ifs::collapsible_if_else::CollapsibleIfElse;
use crate::lints::ifs::equatable_if_let::check_equatable_if_let;
use crate::lints::ifs::equatable_if_let::EquatableIfLet;
use crate::lints::ifs::ifs_same_cond::check_duplicate_if_condition;
use crate::lints::ifs::ifs_same_cond::DuplicateIfCondition;
use crate::lints::int_op_one::check_int_op_one;
use crate::lints::int_op_one::IntegerGreaterEqualMinusOne;
use crate::lints::int_op_one::IntegerGreaterEqualPlusOne;
use crate::lints::int_op_one::IntegerLessEqualMinusOne;
use crate::lints::int_op_one::IntegerLessEqualPlusOne;
use crate::lints::loops::loop_for_while::check_loop_for_while;
use crate::lints::loops::loop_for_while::LoopForWhile;
use crate::lints::loops::loop_match_pop_front::check_loop_match_pop_front;
use crate::lints::loops::loop_match_pop_front::LoopMatchPopFront;
use crate::lints::manual::manual_err::check_manual_err;
use crate::lints::manual::manual_err::ManualErr;
use crate::lints::manual::manual_expect::check_manual_expect;
use crate::lints::manual::manual_expect::ManualExpect;
use crate::lints::manual::manual_expect_err::check_manual_expect_err;
use crate::lints::manual::manual_expect_err::ManualExpectErr;
use crate::lints::manual::manual_is::ManualIsErr;
use crate::lints::manual::manual_is::ManualIsNone;
use crate::lints::manual::manual_is::ManualIsOk;
use crate::lints::manual::manual_is::ManualIsSome;
use crate::lints::manual::manual_ok::check_manual_ok;
use crate::lints::manual::manual_ok::ManualOk;
use crate::lints::manual::manual_ok_or::check_manual_ok_or;
use crate::lints::manual::manual_ok_or::ManualOkOr;
use crate::lints::manual::manual_unwrap_or_default::check_manual_unwrap_or_default;
use crate::lints::manual::manual_unwrap_or_default::ManualUnwrapOrDefault;
use crate::lints::panic::check_panic_usage;
use crate::lints::panic::PanicInCode;
use crate::lints::performance::check_inefficient_while_comp;
use crate::lints::performance::InefficientWhileComparison;
use crate::lints::single_match::check_single_matches;
use crate::lints::single_match::DestructMatchLint;
use crate::lints::single_match::EqualityMatch;
use cairo_lang_defs::{ids::ModuleItemId, plugin::PluginDiagnostic};
use cairo_lang_semantic::db::SemanticGroup;
use cairo_lang_syntax::node::{db::SyntaxGroup, SyntaxNode};
use itertools::Itertools;
use std::collections::HashMap;
use std::sync::Arc;
use std::sync::LazyLock;

/// Type describing a linter group's rule checking function.
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
    ImpossibleComparison,
    EqualityOperation,
    Performance,
}

pub trait Lint: Sync {
    /// A name that is going to be registered by the compiler as an allowed lint to be ignored.
    /// Some multiple lint rules might have the same allowed name. This way all of the will be ignored with only one allow attribute.
    fn allowed_name(&self) -> &'static str;
    /// A predefined message that is going to appear in the compiler's diagnostic output. It should be the same as the one in the lint check function.
    fn diagnostic_message(&self) -> &'static str;
    /// The kind of the lint rule. Some lint rules might have the same kind.
    fn kind(&self) -> CairoLintKind;

    /// Checks if the instance has a fixer.
    /// By default it return false.
    fn has_fixer(&self) -> bool {
        false
    }

    /// Attempts to generate a fix for this Lint's semantic diagnostic.
    /// # Arguments
    ///
    /// * `db` - A reference to the RootDatabase
    /// * `diag` - A reference to the SemanticDiagnostic to be fixed
    ///
    /// # Returns
    /// An `Option<(SyntaxNode, String)>` where the `SyntaxNode` represents the node to be
    /// replaced, and the `String` is the suggested replacement. Returns `None` if no fix
    /// is available for the given diagnostic.
    ///
    /// By default there is no fixing procedure for a Lint.
    #[expect(unused_variables)]
    fn fix(&self, db: &dyn SyntaxGroup, node: SyntaxNode) -> Option<(SyntaxNode, String)> {
        None
    }
}

/// A group of lint rules.
///
/// We want to group lint rules because some lint rules can share an allowed name for compiler or the checking function.
pub struct LintRuleGroup<'a> {
    /// Collection of `LintRule`s that are directly connected to this group's checking function.
    lints: Vec<&'a (dyn Lint + Sync)>,
    /// A Function which will be fired during linter plugin analysis.
    /// This one should emit certain diagnostics in order to later identify (and maybe fix) the linting problem.
    check_function: CheckingFunction,
}

/// A global Linter context. It contains all the lint rules.
struct LintContext<'a> {
    lint_groups: Vec<LintRuleGroup<'a>>,
    diagnostic_to_lint_kind_map: HashMap<&'static str, CairoLintKind>,
}

impl<'a> LintContext<'a> {
    /// All of the predefined rules are stored here. If a new rule is added it should be added here as well.
    fn get_all_lints() -> Vec<LintRuleGroup<'a>> {
        vec![
            LintRuleGroup {
                lints: vec![&DestructMatchLint, &EqualityMatch],
                check_function: Arc::new(check_single_matches),
            },
            LintRuleGroup {
                lints: vec![&DoubleParens],
                check_function: Arc::new(check_double_parens),
            },
            LintRuleGroup {
                lints: vec![
                    &ImpossibleComparison,
                    &SimplifiableComparison,
                    &RedundantComparison,
                    &ContradictoryComparison,
                ],
                check_function: Arc::new(check_double_comparison),
            },
            LintRuleGroup {
                lints: vec![&EquatableIfLet],
                check_function: Arc::new(check_equatable_if_let),
            },
            LintRuleGroup {
                lints: vec![&BreakUnit],
                check_function: Arc::new(check_break),
            },
            LintRuleGroup {
                lints: vec![&BoolComparison],
                check_function: Arc::new(check_bool_comparison),
            },
            LintRuleGroup {
                lints: vec![&CollapsibleIfElse],
                check_function: Arc::new(check_collapsible_if_else),
            },
            LintRuleGroup {
                lints: vec![&CollapsibleIf],
                check_function: Arc::new(check_collapsible_if),
            },
            LintRuleGroup {
                lints: vec![&DuplicateUnderscoreArgs],
                check_function: Arc::new(check_duplicate_underscore_args),
            },
            LintRuleGroup {
                lints: vec![&LoopMatchPopFront],
                check_function: Arc::new(check_loop_match_pop_front),
            },
            LintRuleGroup {
                lints: vec![&ManualUnwrapOrDefault],
                check_function: Arc::new(check_manual_unwrap_or_default),
            },
            LintRuleGroup {
                lints: vec![&BitwiseForParity],
                check_function: Arc::new(check_bitwise_for_parity),
            },
            LintRuleGroup {
                lints: vec![&LoopForWhile],
                check_function: Arc::new(check_loop_for_while),
            },
            LintRuleGroup {
                lints: vec![&PanicInCode],
                check_function: Arc::new(check_panic_usage),
            },
            LintRuleGroup {
                lints: vec![&ErasingOperation],
                check_function: Arc::new(check_erasing_operation),
            },
            LintRuleGroup {
                lints: vec![&ManualOkOr],
                check_function: Arc::new(check_manual_ok_or),
            },
            LintRuleGroup {
                lints: vec![&ManualOk],
                check_function: Arc::new(check_manual_ok),
            },
            LintRuleGroup {
                lints: vec![&ManualErr],
                check_function: Arc::new(check_manual_err),
            },
            LintRuleGroup {
                lints: vec![&ManualIsSome, &ManualIsNone, &ManualIsOk, &ManualIsErr],
                check_function: Arc::new(lints::manual::manual_is::check_manual_is),
            },
            LintRuleGroup {
                lints: vec![&ManualExpect],
                check_function: Arc::new(check_manual_expect),
            },
            LintRuleGroup {
                lints: vec![&DuplicateIfCondition],
                check_function: Arc::new(check_duplicate_if_condition),
            },
            LintRuleGroup {
                lints: vec![&ManualExpectErr],
                check_function: Arc::new(check_manual_expect_err),
            },
            LintRuleGroup {
                lints: vec![
                    &IntegerGreaterEqualPlusOne,
                    &IntegerGreaterEqualMinusOne,
                    &IntegerLessEqualPlusOne,
                    &IntegerLessEqualMinusOne,
                ],
                check_function: Arc::new(check_int_op_one),
            },
            LintRuleGroup {
                lints: vec![
                    &DivisionEqualityOperation,
                    &EqualComparisonOperation,
                    &NotEqualComparisonOperation,
                    &DifferenceEqualityOperation,
                    &BitwiseEqualityOperation,
                    &LogicalEqualityOperation,
                ],
                check_function: Arc::new(check_eq_op),
            },
            LintRuleGroup {
                lints: vec![&InefficientWhileComparison],
                check_function: Arc::new(check_inefficient_while_comp),
            },
        ]
    }

    fn precompute_diagnostic_to_lint_kind_map(mut self) -> Self {
        let mut result: HashMap<&'static str, CairoLintKind> = HashMap::default();
        for rule_group in self.lint_groups.iter() {
            for rule in rule_group.lints.iter() {
                result.insert(rule.diagnostic_message(), rule.kind());
            }
        }
        self.diagnostic_to_lint_kind_map = result;
        self
    }

    fn new() -> Self {
        let new = Self {
            lint_groups: Self::get_all_lints(),
            diagnostic_to_lint_kind_map: Default::default(),
        };
        new.precompute_diagnostic_to_lint_kind_map()
    }

    fn get_lint_type_from_diagnostic_message(&self, message: &str) -> CairoLintKind {
        self.diagnostic_to_lint_kind_map
            .get(message)
            .copied()
            .unwrap_or(CairoLintKind::Unknown)
    }
}

/// A singleton instance of the `LintContext`. It should be the only instance of the `LintContext`.
static LINT_CONTEXT: LazyLock<LintContext> = LazyLock::new(LintContext::new);

/// Get the lint type based on the diagnostic message.
/// If the diagnostic message doesn't match any of the rules, it returns `CairoLintKind::Unknown`.
pub fn get_lint_type_from_diagnostic_message(message: &str) -> CairoLintKind {
    LINT_CONTEXT.get_lint_type_from_diagnostic_message(message)
}

/// Get the fixing function based on the diagnostic message.
/// For some of the rules there is no fixing function, so it returns `None`.
pub fn get_fix_for_diagnostic_message(
    db: &dyn SyntaxGroup,
    node: SyntaxNode,
    message: &str,
) -> Option<(SyntaxNode, String)> {
    LINT_CONTEXT
        .lint_groups
        .iter()
        .flat_map(|rule_group| &rule_group.lints)
        .find(|rule| rule.diagnostic_message() == message)
        .and_then(|rule| rule.fix(db, node))
}

/// Get all the unique allowed names for the lint rule groups.
pub fn get_unique_allowed_names() -> Vec<&'static str> {
    LINT_CONTEXT
        .lint_groups
        .iter()
        .flat_map(|rule_group| rule_group.lints.iter().map(|rule| rule.allowed_name()))
        .collect()
}

/// Get all the checking functions that exist for each `LintRuleGroup`.
pub fn get_all_checking_functions() -> impl Iterator<Item = &'static CheckingFunction> {
    LINT_CONTEXT
        .lint_groups
        .iter()
        .unique_by(|rule| Arc::as_ptr(&rule.check_function))
        .map(|rule_group| &rule_group.check_function)
}
