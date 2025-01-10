use std::{cmp::Reverse, collections::HashMap};

use anyhow::{anyhow, Result};
use cairo_lang_compiler::db::RootDatabase;
use cairo_lang_diagnostics::DiagnosticEntry;
use cairo_lang_filesystem::db::FilesGroup;
use cairo_lang_filesystem::ids::FileId;
use cairo_lang_semantic::{diagnostic::SemanticDiagnosticKind, SemanticDiagnostic};
use cairo_lang_syntax::node::SyntaxNode;
use cairo_lang_utils::Upcast;
use fix::{apply_import_fixes, collect_unused_imports, fix_semantic_diagnostic, Fix, ImportFix};

pub mod diagnostics;
pub mod fix;
pub mod lints;
pub mod plugin;

pub fn apply_fixes<F>(
    db: &RootDatabase,
    diagnostics: Vec<SemanticDiagnostic>,
    printer: F,
) -> Result<()>
where
    F: Fn(&str),
{
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
    for (file_id, mut fixes) in fixes.into_iter() {
        printer(&file_id.file_name(db.upcast()));
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
        std::fs::write(file_id.full_path(db.upcast()), files.get(&file_id).unwrap())?
    }
    Ok(())
}
