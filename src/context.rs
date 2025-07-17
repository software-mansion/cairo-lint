use crate::corelib::CorelibContext;
use crate::fixes::InternalFix;
use crate::lints::bitwise_for_parity_check::BitwiseForParity;
use crate::lints::bitwise_for_parity_check::check_bitwise_for_parity;
use crate::lints::bool_comparison::BoolComparison;
use crate::lints::bool_comparison::check_bool_comparison;
use crate::lints::breaks::BreakUnit;
use crate::lints::breaks::check_break;
use crate::lints::clone_on_copy::{CloneOnCopy, check_clone_on_copy};
use crate::lints::double_comparison::ContradictoryComparison;
use crate::lints::double_comparison::ImpossibleComparison;
use crate::lints::double_comparison::RedundantComparison;
use crate::lints::double_comparison::SimplifiableComparison;
use crate::lints::double_comparison::check_double_comparison;
use crate::lints::double_parens::DoubleParens;
use crate::lints::double_parens::check_double_parens;
use crate::lints::duplicate_underscore_args::DuplicateUnderscoreArgs;
use crate::lints::duplicate_underscore_args::check_duplicate_underscore_args;
use crate::lints::empty_enum_brackets_variant::EmptyEnumBracketsVariant;
use crate::lints::empty_enum_brackets_variant::check_empty_enum_brackets_variant;
use crate::lints::enum_variant_names::EnumVariantNames;
use crate::lints::enum_variant_names::check_enum_variant_names;
use crate::lints::eq_op::BitwiseEqualityOperation;
use crate::lints::eq_op::DifferenceEqualityOperation;
use crate::lints::eq_op::DivisionEqualityOperation;
use crate::lints::eq_op::EqualComparisonOperation;
use crate::lints::eq_op::LogicalEqualityOperation;
use crate::lints::eq_op::NotEqualComparisonOperation;
use crate::lints::eq_op::check_eq_op;
use crate::lints::erasing_op::ErasingOperation;
use crate::lints::erasing_op::check_erasing_operation;
use crate::lints::ifs::collapsible_if::CollapsibleIf;
use crate::lints::ifs::collapsible_if::check_collapsible_if;
use crate::lints::ifs::collapsible_if_else::CollapsibleIfElse;
use crate::lints::ifs::collapsible_if_else::check_collapsible_if_else;
use crate::lints::ifs::equatable_if_let::EquatableIfLet;
use crate::lints::ifs::equatable_if_let::check_equatable_if_let;
use crate::lints::ifs::ifs_same_cond::DuplicateIfCondition;
use crate::lints::ifs::ifs_same_cond::check_duplicate_if_condition;
use crate::lints::int_op_one::IntegerGreaterEqualMinusOne;
use crate::lints::int_op_one::IntegerGreaterEqualPlusOne;
use crate::lints::int_op_one::IntegerLessEqualMinusOne;
use crate::lints::int_op_one::IntegerLessEqualPlusOne;
use crate::lints::int_op_one::check_int_op_one;
use crate::lints::loops::loop_for_while::LoopForWhile;
use crate::lints::loops::loop_for_while::check_loop_for_while;
use crate::lints::loops::loop_match_pop_front::LoopMatchPopFront;
use crate::lints::loops::loop_match_pop_front::check_loop_match_pop_front;
use crate::lints::manual::manual_assert::ManualAssert;
use crate::lints::manual::manual_assert::check_manual_assert;
use crate::lints::manual::manual_err::ManualErr;
use crate::lints::manual::manual_err::check_manual_err;
use crate::lints::manual::manual_expect::ManualExpect;
use crate::lints::manual::manual_expect::check_manual_expect;
use crate::lints::manual::manual_expect_err::ManualExpectErr;
use crate::lints::manual::manual_expect_err::check_manual_expect_err;
use crate::lints::manual::manual_is::ManualIsErr;
use crate::lints::manual::manual_is::ManualIsNone;
use crate::lints::manual::manual_is::ManualIsOk;
use crate::lints::manual::manual_is::ManualIsSome;
use crate::lints::manual::manual_is::check_manual_is;
use crate::lints::manual::manual_is_empty::{ManualIsEmpty, check_manual_is_empty};
use crate::lints::manual::manual_ok::ManualOk;
use crate::lints::manual::manual_ok::check_manual_ok;
use crate::lints::manual::manual_ok_or::ManualOkOr;
use crate::lints::manual::manual_ok_or::check_manual_ok_or;
use crate::lints::manual::manual_unwrap_or::ManualUnwrapOr;
use crate::lints::manual::manual_unwrap_or::check_manual_unwrap_or;
use crate::lints::manual::manual_unwrap_or_default::ManualUnwrapOrDefault;
use crate::lints::manual::manual_unwrap_or_default::check_manual_unwrap_or_default;
use crate::lints::panic::PanicInCode;
use crate::lints::panic::check_panic_usage;
use crate::lints::performance::inefficient_while_comp::InefficientWhileComparison;
use crate::lints::performance::inefficient_while_comp::check_inefficient_while_comp;
use crate::lints::redundant_brackets_in_enum_call::RedundantBracketsInEnumCall;
use crate::lints::redundant_brackets_in_enum_call::check_redundant_brackets_in_enum_call;
use crate::lints::redundant_op::RedundantOperation;
use crate::lints::redundant_op::check_redundant_operation;
use crate::lints::single_match::DestructMatch;
use crate::lints::single_match::EqualityMatch;
use crate::lints::single_match::check_single_matches;
use crate::lints::unit_return_type::UnitReturnType;
use crate::lints::unit_return_type::check_unit_return_type;
use crate::lints::unwrap_syscall::UnwrapSyscall;
use crate::lints::unwrap_syscall::check_unwrap_syscall;
use cairo_lang_defs::{ids::ModuleItemId, plugin::PluginDiagnostic};
use cairo_lang_semantic::db::SemanticGroup;
use cairo_lang_syntax::node::SyntaxNode;
use itertools::Itertools;
use std::collections::HashMap;
use std::sync::LazyLock;

/// Type describing a linter group's rule checking function.
type CheckingFunction = fn(&dyn SemanticGroup, &CorelibContext, &ModuleItemId, &mut Vec<PluginDiagnostic>);

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
    ManualIsEmpty,
    ManualExpect,
    ManualAssert,
    DuplicateIfCondition,
    ManualExpectErr,
    IntGePlusOne,
    IntGeMinOne,
    IntLePlusOne,
    IntLeMinOne,
    ImpossibleComparison,
    EqualityOperation,
    Performance,
    RedundantOperation,
    EnumVariantNames,
    CloneOnCopy,
    EnumEmptyVariantBrackets,
    ManualUnwrapOr,
    UnitReturnType,
    UnwrapSyscall,
}

pub trait Lint: Sync + Send {
    /// A name that is going to be registered by the compiler as an allowed lint to be ignored.
    /// Some multiple lint rules might have the same allowed name. This way all of the will be ignored with only one allow attribute.
    fn allowed_name(&self) -> &'static str;
    /// A predefined message that is going to appear in the compiler's diagnostic output. It should be the same as the one in the lint check function.
    fn diagnostic_message(&self) -> &'static str;
    /// The kind of the lint rule. Some lint rules might have the same kind.
    fn kind(&self) -> CairoLintKind;

    /// Checks if the lint rule is enabled.
    /// By default all of the rules are enabled.
    fn is_enabled(&self) -> bool {
        true
    }

    /// Checks if the instance has a fixer.
    /// By default it return false.
    fn has_fixer(&self) -> bool {
        false
    }

    /// Generates full path to the lint rule. It helps map the Lint struct name to the actual lint rule.
    fn type_name(&self) -> &'static str {
        std::any::type_name::<Self>()
    }

    /// Attempts to generate a fix for this Lint's semantic diagnostic.
    /// # Arguments
    ///
    /// * `db` - A reference to the `dyn SemanticGroup`
    /// * `diag` - A reference to the SemanticDiagnostic to be fixed
    ///
    /// # Returns
    /// An `Option<Vec<InternalFix>>`. Returns `None` if no fix
    /// is available for the given diagnostic.
    ///
    /// By default there is no fixing procedure for a Lint.
    #[expect(unused_variables)]
    fn fix(&self, db: &dyn SemanticGroup, node: SyntaxNode) -> Option<InternalFix> {
        unreachable!("fix() has been called for a lint which has_fixer() returned false")
    }

    /// A short message describing the fix that will be applied.
    fn fix_message(&self) -> Option<&'static str> {
        unreachable!(
            "A fix message has been requested for a lint which has_fixer() returned false for."
        )
    }
}

/// A group of lint rules.
///
/// We want to group lint rules because some lint rules can share an allowed name for compiler or the checking function.
pub struct LintRuleGroup {
    /// Collection of `LintRule`s that are directly connected to this group's checking function.
    lints: Vec<Box<dyn Lint>>,
    /// A Function which will be fired during linter plugin analysis.
    /// This one should emit certain diagnostics in order to later identify (and maybe fix) the linting problem.
    check_function: CheckingFunction,
}

/// A global Linter context. It contains all the lint rules.
struct LintContext {
    lint_groups: Vec<LintRuleGroup>,
    diagnostic_to_lint_kind_map: HashMap<&'static str, CairoLintKind>,
}

impl LintContext {
    /// All of the predefined rules are stored here. If a new rule is added it should be added here as well.
    fn get_all_lints() -> Vec<LintRuleGroup> {
        vec![
            LintRuleGroup {
                lints: vec![Box::new(DestructMatch), Box::new(EqualityMatch)],
                check_function: check_single_matches,
            },
            LintRuleGroup {
                lints: vec![Box::new(DoubleParens)],
                check_function: check_double_parens,
            },
            LintRuleGroup {
                lints: vec![
                    Box::new(ImpossibleComparison),
                    Box::new(SimplifiableComparison),
                    Box::new(RedundantComparison),
                    Box::new(ContradictoryComparison),
                ],
                check_function: check_double_comparison,
            },
            LintRuleGroup {
                lints: vec![Box::new(EquatableIfLet)],
                check_function: check_equatable_if_let,
            },
            LintRuleGroup {
                lints: vec![Box::new(BreakUnit)],
                check_function: check_break,
            },
            LintRuleGroup {
                lints: vec![Box::new(BoolComparison)],
                check_function: check_bool_comparison,
            },
            LintRuleGroup {
                lints: vec![Box::new(CollapsibleIfElse)],
                check_function: check_collapsible_if_else,
            },
            LintRuleGroup {
                lints: vec![Box::new(CollapsibleIf)],
                check_function: check_collapsible_if,
            },
            LintRuleGroup {
                lints: vec![Box::new(DuplicateUnderscoreArgs)],
                check_function: check_duplicate_underscore_args,
            },
            LintRuleGroup {
                lints: vec![Box::new(LoopMatchPopFront)],
                check_function: check_loop_match_pop_front,
            },
            LintRuleGroup {
                lints: vec![Box::new(ManualUnwrapOrDefault)],
                check_function: check_manual_unwrap_or_default,
            },
            LintRuleGroup {
                lints: vec![Box::new(BitwiseForParity)],
                check_function: check_bitwise_for_parity,
            },
            LintRuleGroup {
                lints: vec![Box::new(LoopForWhile)],
                check_function: check_loop_for_while,
            },
            LintRuleGroup {
                lints: vec![Box::new(PanicInCode)],
                check_function: check_panic_usage,
            },
            LintRuleGroup {
                lints: vec![Box::new(ErasingOperation)],
                check_function: check_erasing_operation,
            },
            LintRuleGroup {
                lints: vec![Box::new(ManualOkOr)],
                check_function: check_manual_ok_or,
            },
            LintRuleGroup {
                lints: vec![Box::new(ManualIsEmpty)],
                check_function: check_manual_is_empty,
            },
            LintRuleGroup {
                lints: vec![Box::new(ManualOk)],
                check_function: check_manual_ok,
            },
            LintRuleGroup {
                lints: vec![Box::new(ManualErr)],
                check_function: check_manual_err,
            },
            LintRuleGroup {
                lints: vec![
                    Box::new(ManualIsSome),
                    Box::new(ManualIsNone),
                    Box::new(ManualIsOk),
                    Box::new(ManualIsErr),
                ],
                check_function: check_manual_is,
            },
            LintRuleGroup {
                lints: vec![Box::new(ManualExpect)],
                check_function: check_manual_expect,
            },
            LintRuleGroup {
                lints: vec![Box::new(DuplicateIfCondition)],
                check_function: check_duplicate_if_condition,
            },
            LintRuleGroup {
                lints: vec![Box::new(ManualExpectErr)],
                check_function: check_manual_expect_err,
            },
            // LintRuleGroup {
            //     lints: vec![
            //         Box::new(IntegerGreaterEqualPlusOne),
            //         Box::new(IntegerGreaterEqualMinusOne),
            //         Box::new(IntegerLessEqualPlusOne),
            //         Box::new(IntegerLessEqualMinusOne),
            //     ],
            //     check_function: check_int_op_one,
            // },
            LintRuleGroup {
                lints: vec![
                    Box::new(DivisionEqualityOperation),
                    Box::new(EqualComparisonOperation),
                    Box::new(NotEqualComparisonOperation),
                    Box::new(DifferenceEqualityOperation),
                    Box::new(BitwiseEqualityOperation),
                    Box::new(LogicalEqualityOperation),
                ],
                check_function: check_eq_op,
            },
            LintRuleGroup {
                lints: vec![Box::new(InefficientWhileComparison)],
                check_function: check_inefficient_while_comp,
            },
            LintRuleGroup {
                lints: vec![Box::new(RedundantOperation)],
                check_function: check_redundant_operation,
            },
            LintRuleGroup {
                lints: vec![Box::new(EnumVariantNames)],
                check_function: check_enum_variant_names,
            },
            // LintRuleGroup {
            //     lints: vec![Box::new(CloneOnCopy)],
            //     check_function: check_clone_on_copy,
            // },
            LintRuleGroup {
                lints: vec![Box::new(EmptyEnumBracketsVariant)],
                check_function: check_empty_enum_brackets_variant,
            },
            LintRuleGroup {
                lints: vec![Box::new(ManualAssert)],
                check_function: check_manual_assert,
            },
            LintRuleGroup {
                lints: vec![Box::new(RedundantBracketsInEnumCall)],
                check_function: check_redundant_brackets_in_enum_call,
            },
            LintRuleGroup {
                lints: vec![Box::new(ManualUnwrapOr)],
                check_function: check_manual_unwrap_or,
            },
            LintRuleGroup {
                lints: vec![Box::new(UnitReturnType)],
                check_function: check_unit_return_type,
            },
            // LintRuleGroup {
            //     lints: vec![Box::new(UnwrapSyscall)],
            //     check_function: check_unwrap_syscall,
            // },
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
    db: &dyn SemanticGroup,
    node: SyntaxNode,
    message: &str,
) -> Option<InternalFix> {
    LINT_CONTEXT
        .lint_groups
        .iter()
        .flat_map(|rule_group| &rule_group.lints)
        .find(|rule| rule.diagnostic_message() == message && rule.has_fixer())
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
        .unique_by(|rule| rule.check_function)
        .map(|rule_group| &rule_group.check_function)
}

/// Get lint name based on the diagnostic message.
pub fn get_name_for_diagnostic_message(message: &str) -> Option<&'static str> {
    LINT_CONTEXT
        .lint_groups
        .iter()
        .flat_map(|group| group.lints.iter())
        .find(|rule| rule.diagnostic_message() == message)
        .map(|rule| rule.allowed_name())
}

/// Checks if the lint related to the diagnostic message is enabled by default.
pub fn is_lint_enabled_by_default(message: &str) -> Option<bool> {
    LINT_CONTEXT
        .lint_groups
        .iter()
        .flat_map(|group| group.lints.iter())
        .find(|rule| rule.diagnostic_message() == message)
        .map(|rule| rule.is_enabled())
}

#[allow(clippy::borrowed_box)]
/// Finds the lint by it's struct's name.
/// By struct name we mean the last part of the path of the lint rule.
/// For example, for `crate::lints::bool_comparison::BoolComparison` the struct name is `BoolComparison`.
pub fn find_lint_by_struct_name(name: &str) -> Option<&Box<dyn Lint>> {
    LINT_CONTEXT
        .lint_groups
        .iter()
        .flat_map(|group| group.lints.iter())
        .find(|rule| rule.type_name().split("::").last().unwrap() == name)
}

/// Get lint name based on the fix message.
/// Only checks lints that have a fixer.
pub fn get_name_for_fix_message(message: &str) -> Option<&'static str> {
    LINT_CONTEXT
        .lint_groups
        .iter()
        .flat_map(|group| group.lints.iter())
        .find(|rule| {
            rule.has_fixer()
                && rule.fix_message().is_some()
                && rule.fix_message().unwrap() == message
        })
        .map(|rule| rule.allowed_name())
}

/// Returns `fix_message` for all lints that support fixes.
pub fn get_all_fix_messages() -> Vec<Option<&'static str>> {
    LINT_CONTEXT
        .lint_groups
        .iter()
        .flat_map(|rule_group| {
            rule_group
                .lints
                .iter()
                .filter(|rule| rule.has_fixer())
                .map(|rule| rule.fix_message())
        })
        .collect()
}
