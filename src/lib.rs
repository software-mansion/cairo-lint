use cairo_lang_defs::plugin::PluginDiagnostic;
use cairo_lang_formatter::FormatterConfig;
use cairo_lang_utils::ordered_hash_map::OrderedHashMap;
use fixer::{
    DiagnosticFixSuggestion, FixerDatabase, get_fixes_without_resolving_overlapping,
    merge_overlapping_fixes,
};

use cairo_lang_syntax::node::db::SyntaxGroup;
use helper::format_fixed_file;
use itertools::Itertools;

use std::{cmp::Reverse, collections::HashMap};

use anyhow::{Result, anyhow};
use cairo_lang_filesystem::ids::FileId;
use cairo_lang_semantic::{SemanticDiagnostic, db::SemanticGroup};

pub static CAIRO_LINT_TOOL_NAME: &str = "cairo-lint";

/// Describes tool metadata for the Cairo lint.
/// IMPORTANT: This one is a public type, so watch out when modifying it,
/// as it might break the backwards compatibility.
pub type CairoLintToolMetadata = OrderedHashMap<String, bool>;

pub mod context;

mod corelib;
pub mod diagnostics;
mod fixer;
mod helper;
mod lang;
pub mod lints;
mod mappings;
pub mod plugin;
mod queries;

pub use corelib::CorelibContext;
pub use lang::{
    LinterAnalysisDatabase, LinterAnalysisDatabaseBuilder, LinterDiagnosticParams, LinterGroup,
};

use context::{CairoLintKind, get_lint_type_from_diagnostic_message};

pub trait CairoLintGroup: SemanticGroup + SyntaxGroup {}

/// Gets the fixes for a set of a compiler diagnostics (that uses Cairo lint analyzer plugin).
/// # Arguments
///
/// * `db` - The reference to the database.
/// * `diagnostics` - The list of all compiler diagnostics including those coming from the cairo-lint plugin.
///
/// # Returns
///
/// A HashMap where:
/// * keys are FileIds (that points to a file that the fixes might be applied to).
/// * values are vectors of proposed Fixes.
#[tracing::instrument(skip_all, level = "trace")]
pub fn get_fixes<'db>(
    db: &'db (dyn LinterGroup + 'static),
    linter_params: &LinterDiagnosticParams,
    diagnostics: Vec<SemanticDiagnostic<'db>>,
) -> HashMap<FileId<'db>, Vec<DiagnosticFixSuggestion>> {
    // We need to create a new database to avoid modifying the original one.
    // This one is used to resolve the overlapping fixes.
    let mut new_db = FixerDatabase::new_from(db);
    let fixes = get_fixes_without_resolving_overlapping(db, diagnostics);
    fixes
        .into_iter()
        .map(|(file_id, fixes)| {
            let new_fixes = merge_overlapping_fixes(
                &mut new_db,
                linter_params,
                file_id.long(db).into_file_input(db),
                fixes,
            );
            (file_id, new_fixes)
        })
        .collect()
}

/// Gets all possible fixes for a set of compiler diagnostics (that uses Cairo lint analyzer plugin)
/// without resolving overlapping fixes. This is needed when you want to see all potential fixes,
/// even if they might conflict with each other.
///
/// # Arguments
///
/// * `db` - The reference to the database.
/// * `diagnostics` - The list of all compiler diagnostics including those coming from the cairo-lint plugin.
///
/// # Returns
///
/// A HashMap where:
/// * keys are FileIds (that points to a file that the fixes might be applied to).
/// * values are vectors of proposed Fixes.
#[tracing::instrument(skip_all, level = "trace")]
pub fn get_separated_fixes<'db>(
    db: &'db (dyn LinterGroup + 'static),
    diagnostics: Vec<SemanticDiagnostic<'db>>,
) -> HashMap<FileId<'db>, Vec<DiagnosticFixSuggestion>> {
    get_fixes_without_resolving_overlapping(db, diagnostics)
}

/// Applies the fixes to the file.
///
/// # Arguments
///
/// * `file_id` - The FileId of the file that the fixes should be applied to.
/// * `fixes` - The list of fixes that should be applied to the file.
/// * `db` - The reference to the database that contains the file content.
#[tracing::instrument(skip_all, level = "trace")]
pub fn apply_file_fixes<'db>(
    file_id: FileId<'db>,
    fixes: Vec<DiagnosticFixSuggestion>,
    db: &'db dyn SyntaxGroup,
    formatter_config: FormatterConfig,
) -> Result<()> {
    // Those suggestions MUST be sorted in reverse, so changes at the end of the file,
    // doesn't affect the spans of the previous file suggestions.
    let suggestions = fixes
        .iter()
        .flat_map(|fix| fix.suggestions.iter())
        .sorted_by_key(|suggestion| Reverse(suggestion.span.start))
        .collect::<Vec<_>>();

    // Get all the files that need to be fixed
    let mut files: HashMap<FileId, String> = HashMap::default();
    files.insert(
        file_id,
        db.file_content(file_id)
            .ok_or(anyhow!("{} not found", file_id.file_name(db)))?
            .long(db)
            .to_string(),
    );

    // Can't fail we just set the file value.
    files.entry(file_id).and_modify(|file| {
        for suggestion in suggestions {
            file.replace_range(suggestion.span.to_str_range(), &suggestion.code)
        }
    });

    // Dump them in place.
    std::fs::write(
        file_id.full_path(db),
        format_fixed_file(db, formatter_config, files.get(&file_id).unwrap().clone()),
    )?;

    Ok(())
}

/// Checks if the diagnostic is a panic diagnostic.
pub fn is_panic_diagnostic(diag: &PluginDiagnostic) -> bool {
    get_lint_type_from_diagnostic_message(&diag.message) == CairoLintKind::Panic
}
