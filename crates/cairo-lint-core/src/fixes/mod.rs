use cairo_lang_compiler::db::RootDatabase;
use cairo_lang_defs::plugin::PluginDiagnostic;
use cairo_lang_filesystem::span::TextSpan;
use cairo_lang_semantic::{diagnostic::SemanticDiagnosticKind, SemanticDiagnostic};
use cairo_lang_syntax::node::SyntaxNode;
use cairo_lang_utils::Upcast;
pub use import_fixes::{apply_import_fixes, collect_unused_imports, ImportFix};
use log::debug;

use crate::LINT_CONTEXT;

pub mod break_unit;
pub mod comparisons;
pub mod desctruct_match;
pub mod double_comparison;
pub mod double_parens;
mod helper;
pub mod ifs;
mod import_fixes;
pub mod loops;
pub mod manual;

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
    let fix_function =
        LINT_CONTEXT.get_fixing_function_for_diagnostic_message(&plugin_diag.message);
    if let Some(func) = fix_function {
        func(db, plugin_diag.stable_ptr.lookup(db.upcast()))
    } else {
        None
    }
}
