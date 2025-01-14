use break_unit::fix_break_unit;
use cairo_lang_compiler::db::RootDatabase;
use cairo_lang_defs::plugin::PluginDiagnostic;
use cairo_lang_filesystem::span::TextSpan;
use cairo_lang_semantic::{diagnostic::SemanticDiagnosticKind, SemanticDiagnostic};
use cairo_lang_syntax::node::{
    ast::{ExprBinary, ExprIf},
    SyntaxNode, TypedSyntaxNode,
};
use cairo_lang_utils::Upcast;
use comparisons::{
    bool_comparison::fix_bool_comparison, int_ge_min_one::fix_int_ge_min_one,
    int_ge_plus_one::fix_int_ge_plus_one, int_le_min_one::fix_int_le_min_one,
    int_le_plus_one::fix_int_le_plus_one,
};
use desctruct_match::fix_destruct_match;
use double_comparison::fix_double_comparison;
use double_parens::fix_double_parens;
use ifs::{
    collapsible_if::fix_collapsible_if, collapsible_if_else::fix_collapsible_if_else,
    equatable_if_let::fix_equatable_if_let,
};
pub use import_fixes::{apply_import_fixes, collect_unused_imports, ImportFix};
use log::debug;
use loops::{loop_break::fix_loop_break, loop_match_pop_front::fix_loop_match_pop_front};
use manual::manual_ok::fix_manual_ok;
use manual::manual_ok_or::fix_manual_ok_or;
use manual::manual_unwrap_or_default::fix_manual_unwrap_or_default;
use manual::{
    manual_err::fix_manual_err, manual_expect::fix_manual_expect,
    manual_expect_err::fix_manual_expect_err, manual_is_err::fix_manual_is_err,
    manual_is_none::fix_manual_is_none, manual_is_ok::fix_manual_is_ok,
    manual_is_some::fix_manual_is_some,
};

use crate::{diagnostic_kind_from_message, CairoLintKind};

mod break_unit;
mod comparisons;
mod desctruct_match;
mod double_comparison;
mod double_parens;
mod helper;
mod ifs;
mod import_fixes;
mod loops;
mod manual;

/// Represents a fix for a diagnostic, containing the span of code to be replaced
/// and the suggested replacement.
#[derive(Debug, Clone)]
pub struct Fix {
    pub span: TextSpan,
    pub suggestion: String,
}

/// Attempts to fix a semantic diagnostic.
///
/// This function is the entry point for fixing semantic diagnostics. It examines the
/// diagnostic kind and delegates to specific fix functions based on the diagnostic type.
///
/// # Arguments
///
/// * `db` - A reference to the RootDatabase
/// * `diag` - A reference to the SemanticDiagnostic to be fixed
///
/// # Returns
///
/// An `Option<(SyntaxNode, String)>` where the `SyntaxNode` represents the node to be
/// replaced, and the `String` is the suggested replacement. Returns `None` if no fix
/// is available for the given diagnostic.
pub fn fix_semantic_diagnostic(
    db: &RootDatabase,
    diag: &SemanticDiagnostic,
) -> Option<(SyntaxNode, String)> {
    match diag.kind {
        SemanticDiagnosticKind::PluginDiagnostic(ref plugin_diag) => {
            fix_plugin_diagnostic(db, plugin_diag)
        }
        SemanticDiagnosticKind::UnusedImport(_) => {
            debug!("Unused imports should be handled in preemptively");
            None
        }
        _ => {
            debug!("No fix available for diagnostic: {:?}", diag.kind);
            None
        }
    }
}

/// Fixes a plugin diagnostic by delegating to the appropriate Fixer method.
///
/// # Arguments
///
/// * `db` - A reference to the RootDatabase
/// * `diag` - A reference to the SemanticDiagnostic
/// * `plugin_diag` - A reference to the PluginDiagnostic
///
/// # Returns
///
/// An `Option<(SyntaxNode, String)>` containing the node to be replaced and the
/// suggested replacement.
fn fix_plugin_diagnostic(
    db: &RootDatabase,
    plugin_diag: &PluginDiagnostic,
) -> Option<(SyntaxNode, String)> {
    match diagnostic_kind_from_message(&plugin_diag.message) {
        CairoLintKind::DoubleParens => {
            fix_double_parens(db.upcast(), plugin_diag.stable_ptr.lookup(db.upcast()))
        }
        CairoLintKind::DestructMatch => {
            fix_destruct_match(db, plugin_diag.stable_ptr.lookup(db.upcast()))
        }
        CairoLintKind::DoubleComparison => {
            fix_double_comparison(db.upcast(), plugin_diag.stable_ptr.lookup(db.upcast()))
        }
        CairoLintKind::EquatableIfLet => {
            fix_equatable_if_let(db, plugin_diag.stable_ptr.lookup(db.upcast()))
        }
        CairoLintKind::BreakUnit => fix_break_unit(db, plugin_diag.stable_ptr.lookup(db.upcast())),
        CairoLintKind::CollapsibleIf => {
            fix_collapsible_if(db, plugin_diag.stable_ptr.lookup(db.upcast()))
        }
        CairoLintKind::BoolComparison => fix_bool_comparison(
            db,
            ExprBinary::from_syntax_node(db.upcast(), plugin_diag.stable_ptr.lookup(db.upcast())),
        ),
        CairoLintKind::CollapsibleIfElse => fix_collapsible_if_else(
            db,
            &ExprIf::from_syntax_node(db.upcast(), plugin_diag.stable_ptr.lookup(db.upcast())),
        ),
        CairoLintKind::LoopMatchPopFront => {
            fix_loop_match_pop_front(db, plugin_diag.stable_ptr.lookup(db.upcast()))
        }
        CairoLintKind::ManualUnwrapOrDefault => {
            fix_manual_unwrap_or_default(db, plugin_diag.stable_ptr.lookup(db.upcast()))
        }
        CairoLintKind::LoopForWhile => {
            fix_loop_break(db.upcast(), plugin_diag.stable_ptr.lookup(db.upcast()))
        }
        CairoLintKind::ManualOkOr => {
            fix_manual_ok_or(db, plugin_diag.stable_ptr.lookup(db.upcast()))
        }
        CairoLintKind::ManualOk => fix_manual_ok(db, plugin_diag.stable_ptr.lookup(db.upcast())),
        CairoLintKind::ManualErr => fix_manual_err(db, plugin_diag.stable_ptr.lookup(db.upcast())),
        CairoLintKind::ManualIsSome => {
            fix_manual_is_some(db, plugin_diag.stable_ptr.lookup(db.upcast()))
        }
        CairoLintKind::ManualExpect => {
            fix_manual_expect(db, plugin_diag.stable_ptr.lookup(db.upcast()))
        }
        CairoLintKind::ManualExpectErr => {
            fix_manual_expect_err(db, plugin_diag.stable_ptr.lookup(db.upcast()))
        }
        CairoLintKind::ManualIsNone => {
            fix_manual_is_none(db, plugin_diag.stable_ptr.lookup(db.upcast()))
        }
        CairoLintKind::ManualIsOk => {
            fix_manual_is_ok(db, plugin_diag.stable_ptr.lookup(db.upcast()))
        }
        CairoLintKind::ManualIsErr => {
            fix_manual_is_err(db, plugin_diag.stable_ptr.lookup(db.upcast()))
        }
        CairoLintKind::IntGePlusOne => fix_int_ge_plus_one(
            db,
            ExprBinary::from_syntax_node(db.upcast(), plugin_diag.stable_ptr.lookup(db.upcast())),
        ),
        CairoLintKind::IntGeMinOne => fix_int_ge_min_one(
            db,
            ExprBinary::from_syntax_node(db.upcast(), plugin_diag.stable_ptr.lookup(db.upcast())),
        ),
        CairoLintKind::IntLePlusOne => fix_int_le_plus_one(
            db,
            ExprBinary::from_syntax_node(db.upcast(), plugin_diag.stable_ptr.lookup(db.upcast())),
        ),
        CairoLintKind::IntLeMinOne => fix_int_le_min_one(
            db,
            ExprBinary::from_syntax_node(db.upcast(), plugin_diag.stable_ptr.lookup(db.upcast())),
        ),
        _ => None,
    }
}
