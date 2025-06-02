use cairo_lang_defs::plugin::PluginDiagnostic;
use db::FixerDatabase;
use fixes::{
    file_for_url, get_fixes_without_resolving_overlapping, merge_overlapping_fixes, url_for_file,
    Fix,
};

use cairo_lang_syntax::node::db::SyntaxGroup;

use std::{cmp::Reverse, collections::HashMap};

use anyhow::{anyhow, Result};
use cairo_lang_filesystem::db::FilesGroup;
use cairo_lang_filesystem::ids::FileId;
use cairo_lang_semantic::{db::SemanticGroup, SemanticDiagnostic};

pub static CAIRO_LINT_TOOL_NAME: &str = "cairo-lint";

/// Describes tool metadata for the Cairo lint.
/// IMPORTANT: This one is a public type, so watch out when modifying it,
/// as it might break the backwards compatibility.
pub type CairoLintToolMetadata = HashMap<String, bool>;

pub mod context;
mod db;
pub mod diagnostics;
pub mod fixes;
mod helper;
pub mod lints;
pub mod plugin;
mod queries;
mod types;

use context::{get_lint_type_from_diagnostic_message, CairoLintKind};

pub trait CairoLintGroup: SemanticGroup + SyntaxGroup {}

/// Gets the fixes for a set of a compiler diagnostics (that uses Cairo lint analyzer plugin).
/// # Arguments
///
/// * `db` - The reference to the `dyn SemanticGroup` that the diagnostics were based upon.
/// * `diagnostics` - The list of compiler diagnostics. Make sure that the diagnostics from the Cairo lint analyzer plugin are also included.
///
/// # Returns
///
/// A HashMap where:
/// * keys are FileIds (that points to a file that the fixes might be applied to)
/// * values are vectors of proposed Fixes.
pub fn get_fixes(
    db: &(dyn SemanticGroup + 'static),
    diagnostics: Vec<SemanticDiagnostic>,
) -> HashMap<FileId, Vec<Fix>> {
    // We need to create a new database to avoid modifying the original one.
    // This one is used to resolve the overlapping fixes.
    let mut new_db = FixerDatabase::new_from(db);
    let fixes = get_fixes_without_resolving_overlapping(db, diagnostics);
    fixes
        .into_iter()
        .map(|(file_id, fixes)| {
            let file_url = url_for_file(db, file_id)
                .unwrap_or_else(|| panic!("FileId {:?} should have a URL", file_id));
            let new_db_file_id = file_for_url(&new_db, &file_url).unwrap_or_else(|| {
                panic!("FileUrl {:?} should have a corresponding FileId", file_url)
            });
            let new_fixes = merge_overlapping_fixes(&mut new_db, new_db_file_id, fixes);
            (file_id, new_fixes)
        })
        .collect()
}

/// Applies the fixes to the file.
///
/// # Arguments
///
/// * `file_id` - The FileId of the file that the fixes should be applied to.
/// * `fixes` - The list of fixes that should be applied to the file.
/// * `db` - The reference to the database that contains the file content.
pub fn apply_file_fixes(file_id: FileId, fixes: Vec<Fix>, db: &dyn FilesGroup) -> Result<()> {
    let mut fixes = fixes;
    // Those fixes MUST be sorted in reverse, so changes at the end of the file,
    // doesn't affect the spans of the previous file fixes.
    fixes.sort_by_key(|fix| Reverse(fix.span.start));
    // Get all the files that need to be fixed
    let mut files: HashMap<FileId, String> = HashMap::default();
    files.insert(
        file_id,
        db.file_content(file_id)
            .ok_or(anyhow!("{} not found", file_id.file_name(db)))?
            .to_string(),
    );
    // Fix the files
    for fix in fixes {
        // Can't fail we just set the file value.
        files
            .entry(file_id)
            .and_modify(|file| file.replace_range(fix.span.to_str_range(), &fix.suggestion));
    }
    // Dump them in place
    std::fs::write(file_id.full_path(db), files.get(&file_id).unwrap())?;
    Ok(())
}

/// Checks if the diagnostic is a panic diagnostic.
pub fn is_panic_diagnostic(diag: &PluginDiagnostic) -> bool {
    get_lint_type_from_diagnostic_message(&diag.message) == CairoLintKind::Panic
}
