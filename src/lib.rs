use cairo_lang_defs::plugin::PluginDiagnostic;
use fixes::{apply_import_fixes, collect_unused_imports, fix_semantic_diagnostic, Fix, ImportFix};

use cairo_lang_syntax::node::SyntaxNode;

use std::{cmp::Reverse, collections::HashMap};

use anyhow::{anyhow, Result};
use cairo_lang_compiler::db::RootDatabase;
use cairo_lang_diagnostics::DiagnosticEntry;
use cairo_lang_filesystem::db::FilesGroup;
use cairo_lang_filesystem::ids::FileId;
use cairo_lang_semantic::{diagnostic::SemanticDiagnosticKind, SemanticDiagnostic};
use cairo_lang_utils::Upcast;
use serde::{Deserialize, Serialize};

pub static CAIRO_LINT_TOOL_NAME: &str = "cairo-lint";

/// Describes tool metadata for the Cairo lint.
#[derive(Serialize, Deserialize, Clone, Default, Debug)]
pub struct CairoLintToolMetadata {
    pub nopanic: bool,
}

pub mod context;
pub mod diagnostics;
pub mod fixes;
mod helper;
pub mod lints;
pub mod plugin;
mod queries;

use context::{get_lint_type_from_diagnostic_message, CairoLintKind};

/// Gets the fixes for a set of a compiler diagnostics (that uses Cairo lint analyzer plugin).
/// # Arguments
///
/// * `db` - The reference to the RootDatabase that the diagnostics were based upon.
/// * `diagnostics` - The list of compiler diagnostics. Make sure that the diagnostics from the Cairo lint analyzer plugin are also included.
///
/// # Returns
///
/// A HashMap where:
/// * keys are FileIds (that points to a file that the fixes might be applied to)
/// * values are vectors of proposed Fixes.
pub fn get_fixes(
    db: &RootDatabase,
    diagnostics: Vec<SemanticDiagnostic>,
) -> HashMap<FileId, Vec<Fix>> {
    // Handling unused imports separately as we need to run pre-analysis on the diagnostics.
    // to handle complex cases.
    let unused_imports: HashMap<FileId, HashMap<SyntaxNode, ImportFix>> =
        collect_unused_imports(db, &diagnostics);
    let mut fixes = HashMap::new();
    unused_imports.keys().for_each(|file_id| {
        let file_fixes: Vec<Fix> = apply_import_fixes(db, unused_imports.get(file_id).unwrap());
        fixes.insert(*file_id, file_fixes);
    });

    let diags_without_imports = diagnostics
        .iter()
        .filter(|diag| !matches!(diag.kind, SemanticDiagnosticKind::UnusedImport(_)))
        .collect::<Vec<_>>();

    for diag in diags_without_imports {
        if let Some((fix_node, fix)) = fix_semantic_diagnostic(db, diag) {
            let location = diag.location(db.upcast());
            fixes
                .entry(location.file_id)
                .or_insert_with(Vec::new)
                .push(Fix {
                    span: fix_node.span(db.upcast()),
                    suggestion: fix,
                });
        }
    }
    fixes
}

/// Applies the fixes to the file.
///
/// # Arguments
///
/// * `file_id` - The FileId of the file that the fixes should be applied to.
/// * `fixes` - The list of fixes that should be applied to the file.
pub fn apply_file_fixes(file_id: FileId, fixes: Vec<Fix>, db: &RootDatabase) -> Result<()> {
    let mut fixes = fixes;
    fixes.sort_by_key(|fix| Reverse(fix.span.start));
    let mut fixable_diagnostics = Vec::with_capacity(fixes.len());
    if fixes.len() <= 1 {
        fixable_diagnostics = fixes;
    } else {
        // Check if we have nested diagnostics. If so it's a nightmare to fix hence just ignore it
        for i in 0..fixes.len() - 1 {
            let first = fixes[i].span;
            let second = fixes[i + 1].span;
            if first.start >= second.end {
                fixable_diagnostics.push(fixes[i].clone());
                if i == fixes.len() - 1 {
                    fixable_diagnostics.push(fixes[i + 1].clone());
                }
            }
        }
    }
    // Get all the files that need to be fixed
    let mut files: HashMap<FileId, String> = HashMap::default();
    files.insert(
        file_id,
        db.file_content(file_id)
            .ok_or(anyhow!("{} not found", file_id.file_name(db.upcast())))?
            .to_string(),
    );
    // Fix the files
    for fix in fixable_diagnostics {
        // Can't fail we just set the file value.
        files
            .entry(file_id)
            .and_modify(|file| file.replace_range(fix.span.to_str_range(), &fix.suggestion));
    }
    // Dump them in place
    std::fs::write(file_id.full_path(db.upcast()), files.get(&file_id).unwrap())?;
    Ok(())
}

/// Checks if the diagnostic is a panic diagnostic.
pub fn is_panic_diagnostic(diag: &PluginDiagnostic) -> bool {
    get_lint_type_from_diagnostic_message(&diag.message) == CairoLintKind::Panic
}
