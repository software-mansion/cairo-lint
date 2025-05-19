//! # Import Fixes for Cairo Lint
//!
//! This module provides functionality to detect and fix unused imports in Cairo code.
//! The process involves three main steps:
//!
//! 1. Collecting unused imports: Analyze semantic diagnostics to identify unused imports.
//! 2. Creating import fixes: Generate `ImportFix` structures for each unused import.
//! 3. Applying fixes: Remove or modify the imports based on the collected fixes.
//!
//! The module handles both single imports and multi-imports, ensuring that only unused
//! items are removed while preserving the structure of the import statements.

use std::collections::HashMap;
use std::sync::Arc;

use cairo_lang_defs::ids::UseId;
use cairo_lang_defs::plugin::PluginDiagnostic;
use cairo_lang_diagnostics::DiagnosticEntry;
use cairo_lang_filesystem::db::{FilesGroup, FilesGroupEx};
use cairo_lang_filesystem::ids::{FileId, FileLongId};
use cairo_lang_filesystem::span::{TextOffset, TextSpan, TextWidth};
use cairo_lang_semantic::diagnostic::SemanticDiagnosticKind;
use cairo_lang_semantic::SemanticDiagnostic;
use cairo_lang_syntax::node::kind::SyntaxKind;
use cairo_lang_syntax::node::{SyntaxNode, TypedStablePtr, TypedSyntaxNode};
use cairo_lang_utils::LookupIntern;
use log::debug;
use lsp_types::Url;
use salsa::InternKey;

use crate::context::get_fix_for_diagnostic_message;
use crate::db::FixerDatabase;
use cairo_lang_defs::db::DefsGroup;
use cairo_lang_semantic::db::SemanticGroup;
use cairo_lang_syntax::node::db::SyntaxGroup;

/// Represents a fix for a diagnostic, containing the span of code to be replaced
/// and the suggested replacement.
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct Fix {
    pub span: TextSpan,
    pub suggestion: String,
}

pub fn get_fixes_without_resolving_overlapping(
    db: &(dyn SemanticGroup + 'static),
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
            let location = diag.location(db);
            fixes
                .entry(location.file_id)
                .or_insert_with(Vec::new)
                .push(Fix {
                    span: fix_node.span(db),
                    suggestion: fix,
                });
        }
    }
    fixes
}

/// Attempts to fix a semantic diagnostic.
///
/// This function is the entry point for fixing semantic diagnostics. It examines the
/// diagnostic kind and delegates to specific fix functions based on the diagnostic type.
///
/// # Arguments
///
/// * `db` - A reference to the `dyn SemanticGroup`
/// * `diag` - A reference to the SemanticDiagnostic to be fixed
///
/// # Returns
///
/// An `Option<(SyntaxNode, String)>` where the `SyntaxNode` represents the node to be
/// replaced, and the `String` is the suggested replacement. Returns `None` if no fix
/// is available for the given diagnostic.
pub fn fix_semantic_diagnostic(
    db: &dyn SemanticGroup,
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
/// * `db` - A reference to the `dyn SemanticGroup`
/// * `diag` - A reference to the SemanticDiagnostic
/// * `plugin_diag` - A reference to the PluginDiagnostic
///
/// # Returns
///
/// An `Option<(SyntaxNode, String)>` containing the node to be replaced and the
/// suggested replacement.
fn fix_plugin_diagnostic(
    db: &dyn SemanticGroup,
    plugin_diag: &PluginDiagnostic,
) -> Option<(SyntaxNode, String)> {
    let node = plugin_diag.stable_ptr.lookup(db);
    get_fix_for_diagnostic_message(db, node, &plugin_diag.message)
}

/// Represents a fix for unused imports in a specific syntax node.
#[derive(Debug, Clone)]
pub struct ImportFix {
    /// The node that contains the imports to be fixed.
    pub node: SyntaxNode,
    /// The items to remove from the imports.
    pub items_to_remove: Vec<String>,
}

impl ImportFix {
    /// Creates a new `ImportFix` for the given syntax node.
    pub fn new(node: SyntaxNode) -> Self {
        ImportFix {
            node,
            items_to_remove: vec![],
        }
    }
}

/// Collects unused imports from semantic diagnostics.
///
/// # Arguments
///
/// * `db` - The root database containing the project information.
/// * `diags` - A vector of semantic diagnostics.
///
/// # Returns
///
/// A HashMap where keys are FileIds and values are HashMaps of SyntaxNodes to ImportFixes.
pub fn collect_unused_imports(
    db: &(dyn SemanticGroup + 'static),
    diags: &Vec<SemanticDiagnostic>,
) -> HashMap<FileId, HashMap<SyntaxNode, ImportFix>> {
    let mut file_fixes = HashMap::new();

    for diag in diags {
        if let SemanticDiagnosticKind::UnusedImport(id) = &diag.kind {
            let file_id = diag.location(db).file_id;

            let local_fixes = file_fixes.entry(file_id).or_insert_with(HashMap::new);
            process_unused_import(db, id, local_fixes);
        }
    }

    file_fixes
}

/// Processes an unused import and updates the fixes HashMap.
///
/// # Arguments
///
/// * `db` - The root database containing the project information.
/// * `id` - The UseId of the unused import.
/// * `fixes` - A mutable reference to the HashMap of fixes.
fn process_unused_import(
    db: &dyn DefsGroup,
    id: &UseId,
    fixes: &mut HashMap<SyntaxNode, ImportFix>,
) {
    let unused_node = id.stable_ptr(db).lookup(db).as_syntax_node();
    let mut current_node = unused_node;

    while let Some(parent) = current_node.parent(db) {
        match parent.kind(db) {
            SyntaxKind::UsePathMulti => {
                fixes
                    .entry(parent)
                    .or_insert_with(|| ImportFix::new(parent))
                    .items_to_remove
                    .push(unused_node.get_text_without_trivia(db));
                break;
            }
            SyntaxKind::ItemUse => {
                fixes.insert(parent, ImportFix::new(parent));
                break;
            }
            _ => current_node = parent,
        }
    }
}

/// Applies the collected import fixes to generate a list of Fix objects.
///
/// # Arguments
///
/// * `db` - The root database containing the project information.
/// * `fixes` - A HashMap of SyntaxNodes to ImportFixes.
///
/// # Returns
///
/// A vector of Fix objects representing the applied fixes.
pub fn apply_import_fixes(
    db: &dyn SyntaxGroup,
    fixes: &HashMap<SyntaxNode, ImportFix>,
) -> Vec<Fix> {
    fixes
        .iter()
        .flat_map(|(_, import_fix)| {
            let span = import_fix.node.span(db);

            if import_fix.items_to_remove.is_empty() {
                // Single import case: remove entire import
                vec![Fix {
                    span,
                    suggestion: String::new(),
                }]
            } else {
                // Multi-import case
                handle_multi_import(db, &import_fix.node, &import_fix.items_to_remove)
            }
        })
        .collect()
}

/// Handles multi-import cases, deciding whether to remove the entire import or specific items.
///
/// # Arguments
///
/// * `db` - The root database containing the project information.
/// * `node` - The syntax node of the import.
/// * `items_to_remove` - A slice of strings representing the items to be removed.
///
/// # Returns
///
/// A vector of Fix objects for the multi-import case.
fn handle_multi_import(
    db: &dyn SyntaxGroup,
    node: &SyntaxNode,
    items_to_remove: &[String],
) -> Vec<Fix> {
    if all_descendants_removed(db, node, items_to_remove) {
        remove_entire_import(db, node)
    } else {
        remove_specific_items(db, node, items_to_remove)
    }
}

/// Checks if all descendants of a node are to be removed.
///
/// # Arguments
///
/// * `db` - The root database containing the project information.
/// * `node` - The syntax node to check.
/// * `items_to_remove` - A slice of strings representing the items to be removed.
///
/// # Returns
///
/// A boolean indicating whether all descendants should be removed.
fn all_descendants_removed(
    db: &dyn SyntaxGroup,
    node: &SyntaxNode,
    items_to_remove: &[String],
) -> bool {
    node.descendants(db)
        .filter(|child| child.kind(db) == SyntaxKind::UsePathLeaf)
        .all(|child| items_to_remove.contains(&child.get_text_without_trivia(db)))
}

/// Removes an entire import statement.
///
/// We traverse the parents until we either find a UsePathList on the path - then, we can remove the
/// current node from that list - or we find an ItemUse, in which case we remove the entire import
/// line.
///
/// # Arguments
/// * `db` - The root database containing the project information.
/// * `node` - The syntax node of the import to remove.
///
/// # Returns
///
/// A vector of Fix objects for removing the entire import.
fn remove_entire_import(db: &dyn SyntaxGroup, node: &SyntaxNode) -> Vec<Fix> {
    let mut current_node = *node;
    while let Some(parent) = current_node.parent(db) {
        // Go up until we find a UsePathList on the path - then, we can remove the current node from that
        // list.
        if parent.kind(db) == SyntaxKind::UsePathList {
            // To remove the current node from the UsePathList, we need to:
            // 1. Get the text of the current node, which becomes "to remove"
            // 2. Rewrite the UsePathList with the current node text removed.
            let items_to_remove = vec![current_node.get_text_without_trivia(db)];
            if let Some(grandparent) = parent.parent(db) {
                return handle_multi_import(db, &grandparent, &items_to_remove);
            }
        }
        if parent.kind(db) == SyntaxKind::ItemUse {
            current_node = parent;
            break;
        }
        current_node = parent;
    }
    vec![Fix {
        span: current_node.span(db),
        suggestion: String::new(),
    }]
}

/// Removes specific items from a multi-import statement.
///
/// # Arguments
///
/// * `db` - The root database containing the project information.
/// * `node` - The syntax node of the import.
/// * `items_to_remove` - A slice of strings representing the items to be removed.
///
/// # Returns
///
/// A vector of Fix objects for removing specific items from the import.
fn remove_specific_items(
    db: &dyn SyntaxGroup,
    node: &SyntaxNode,
    items_to_remove: &[String],
) -> Vec<Fix> {
    let use_path_list = find_use_path_list(db, node);
    let children = use_path_list.get_children(db);
    let children: Vec<SyntaxNode> = children
        .iter()
        .filter(|child| {
            let text = child.get_text(db).trim().replace('\n', "");
            !text.is_empty() && !text.eq(",")
        })
        .cloned()
        .collect();
    let mut items: Vec<_> = children
        .iter()
        .map(|child| child.get_text(db).trim().to_string())
        .collect();
    items.retain(|item| !items_to_remove.contains(&item.to_string()));

    let text = if items.len() == 1 {
        items[0].to_string()
    } else {
        format!("{{{}}}", items.join(", "))
    };

    vec![Fix {
        span: node.span(db),
        suggestion: text,
    }]
}

/// Finds the UsePathList node within a given syntax node.
///
/// # Arguments
///
/// * `db` - The root database containing the project information.
/// * `node` - The syntax node to search within.
///
/// # Returns
///
/// The UsePathList syntax node, or the original node if not found.
fn find_use_path_list(db: &dyn SyntaxGroup, node: &SyntaxNode) -> SyntaxNode {
    node.descendants(db)
        .find(|descendant| descendant.kind(db) == SyntaxKind::UsePathList)
        .unwrap_or(*node)
}

pub fn merge_overlapping_fixes(
    db: &mut FixerDatabase,
    file_id: FileId,
    fixes: Vec<Fix>,
) -> Vec<Fix> {
    let mut current_fixes: Vec<Fix> = fixes.clone();
    let mut were_overlapped = false;
    let file_content = db.file_content(file_id).unwrap();

    while let Some(overlapping_fix) = get_first_overlapping_fix(&current_fixes) {
        were_overlapped = true;
        apply_fix_for_file(db, file_id, overlapping_fix);

        let diags: Vec<SemanticDiagnostic> = db
            .file_modules(file_id)
            .unwrap()
            .iter()
            .filter_map(|module_id| db.module_semantic_diagnostics(*module_id).ok())
            .flat_map(|diag| diag.get_all())
            .filter(|diag| diag.stable_location.diagnostic_location(db).file_id == file_id)
            .collect();

        current_fixes = get_fixes_without_resolving_overlapping(db, diags)
            .values()
            .flat_map(|v| v.clone())
            .collect();
    }

    if were_overlapped {
        for fix in current_fixes.iter() {
            apply_fix_for_file(db, file_id, fix.clone());
        }
        let file_content_after = db.file_content(file_id).unwrap();
        // Currently we are just replacing the entire file content with the new fixed one.
        // This is not ideal, but as for now we don't need to worry about it.
        current_fixes = vec![Fix {
            span: TextSpan {
                start: TextOffset::START,
                end: TextWidth::from_str(&file_content).as_offset(),
            },
            suggestion: file_content_after.to_string(),
        }];
    }
    current_fixes
}

fn get_first_overlapping_fix(fixes: &[Fix]) -> Option<Fix> {
    for current_fix in fixes.iter() {
        if fixes
            .iter()
            .any(|fix| spans_intersects(fix.span, current_fix.span) && fix != current_fix)
        {
            return Some(current_fix.clone());
        }
    }
    None
}

fn apply_fix_for_file(db: &mut FixerDatabase, file_id: FileId, fix: Fix) {
    let mut content = db.file_content(file_id).unwrap().to_string();
    content.replace_range(fix.span.to_str_range(), &fix.suggestion);
    db.override_file_content(file_id, Some(Arc::from(content)));
}

fn spans_intersects(span_a: TextSpan, span_b: TextSpan) -> bool {
    span_a.start <= span_b.end && span_b.start <= span_a.end
}

/// Get the [`FileId`] for a [`Url`].
pub fn file_for_url(db: &(dyn SemanticGroup + 'static), uri: &Url) -> Option<FileId> {
    match uri.scheme() {
        "file" => uri
            .to_file_path()
            .inspect_err(|()| panic!("invalid file url: {uri}"))
            .ok()
            .map(|path| FileId::new(db.upcast(), path)),
        "vfs" => uri
            .host_str()
            .or_else(|| {
                panic!("invalid vfs url, missing host string: {uri:?}");
            })?
            .parse::<usize>()
            .inspect_err(|e| {
                panic!("invalid vfs url, host string is not a valid integer, {e}: {uri:?}")
            })
            .ok()
            .map(Into::into)
            .map(FileId::from_intern_id),
        _ => {
            panic!("invalid url, scheme is not supported by this language server: {uri:?}");
        }
    }
}

/// Get the canonical [`Url`] for a [`FileId`].
pub fn url_for_file(db: &(dyn SemanticGroup + 'static), file_id: FileId) -> Option<Url> {
    let vf = match file_id.lookup_intern(db) {
        FileLongId::OnDisk(path) => return Some(Url::from_file_path(path).unwrap()),
        FileLongId::Virtual(vf) => vf,
        FileLongId::External(id) => db.try_ext_as_virtual(id)?,
    };
    let mut url = Url::parse("vfs://").unwrap();
    url.set_host(Some(&file_id.as_intern_id().to_string()))
        .unwrap();
    url.path_segments_mut()
        .unwrap()
        .push(&format!("{}.cairo", vf.name));
    Some(url)
}
