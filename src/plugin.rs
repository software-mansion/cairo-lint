use anyhow::{anyhow, Result};
use cairo_lang_defs::ids::{LanguageElementId, ModuleId};
use cairo_lang_defs::plugin::PluginDiagnostic;
use cairo_lang_filesystem::ids::{FileId, FileLongId};
use cairo_lang_semantic::db::SemanticGroup;
use cairo_lang_semantic::plugin::{AnalyzerPlugin, PluginSuite};
use cairo_lang_syntax::node::db::SyntaxGroup;
use cairo_lang_syntax::node::helpers::QueryAttrs;
use cairo_lang_syntax::node::SyntaxNode;
use cairo_lang_utils::LookupIntern;
use std::sync::Arc;

use crate::context::{
    get_all_checking_functions, get_name_for_diagnostic_message, get_unique_allowed_names,
    is_lint_enabled_by_default,
};
use crate::CairoLintToolMetadata;

pub fn cairo_lint_plugin_suite(tool_metadata: CairoLintToolMetadata) -> Result<PluginSuite> {
    let mut suite = PluginSuite::default();
    validate_cairo_lint_metadata(&tool_metadata)?;
    suite.add_analyzer_plugin_ex(Arc::new(CairoLint::new(false, tool_metadata)));
    Ok(suite)
}

pub fn cairo_lint_plugin_suite_without_metadata_validation(
    tool_metadata: CairoLintToolMetadata,
) -> PluginSuite {
    let mut suite = PluginSuite::default();
    suite.add_analyzer_plugin_ex(Arc::new(CairoLint::new(false, tool_metadata)));
    suite
}

pub fn cairo_lint_allow_plugin_suite() -> PluginSuite {
    let mut suite = PluginSuite::default();
    suite.add_analyzer_plugin::<CairoLintAllow>();
    suite
}

#[derive(Debug, Default)]
pub struct CairoLint {
    include_compiler_generated_files: bool,
    tool_metadata: CairoLintToolMetadata,
}

impl CairoLint {
    pub fn new(
        include_compiler_generated_files: bool,
        tool_metadata: CairoLintToolMetadata,
    ) -> Self {
        Self {
            include_compiler_generated_files,
            tool_metadata,
        }
    }

    pub fn include_compiler_generated_files(&self) -> bool {
        self.include_compiler_generated_files
    }

    pub fn tool_metadata(&self) -> &CairoLintToolMetadata {
        &self.tool_metadata
    }
}

impl AnalyzerPlugin for CairoLint {
    fn declared_allows(&self) -> Vec<String> {
        get_unique_allowed_names()
            .iter()
            .map(ToString::to_string)
            .collect()
    }

    fn diagnostics(&self, db: &dyn SemanticGroup, module_id: ModuleId) -> Vec<PluginDiagnostic> {
        let mut diags: Vec<(PluginDiagnostic, FileId)> = Vec::new();
        let Ok(items) = db.module_items(module_id) else {
            return Vec::default();
        };
        for item in &*items {
            let module_file = db.module_main_file(module_id).unwrap();
            let item_file = item.stable_location(db).file_id(db).lookup_intern(db);

            // Skip compiler generated files. By default it checks whether the item is inside the virtual or external file.
            if !self.include_compiler_generated_files
                && (matches!(item_file, FileLongId::Virtual(_) | FileLongId::External(_)))
            {
                continue;
            }

            let checking_functions = get_all_checking_functions();
            let mut item_diagnostics = Vec::new();

            for checking_function in checking_functions {
                checking_function(db, item, &mut item_diagnostics);
            }
            
            diags.extend(item_diagnostics.into_iter().map(|diag| (diag, module_file)));
        }

        diags
            .into_iter()
            .filter(|diag| {
                let diagnostic = &diag.0;
                let diagnostic_origin_module_file = &diag.1;
                let is_compiler_plugin_generated_file =
                    &diagnostic.stable_ptr.file_id(db) != diagnostic_origin_module_file;
                let node = diagnostic.stable_ptr.lookup(db.upcast());
                let allowed_name = get_name_for_diagnostic_message(&diagnostic.message).unwrap();
                let default_allowed = is_lint_enabled_by_default(&diagnostic.message).unwrap();
                let is_rule_allowed_globally = *self
                    .tool_metadata
                    .get(allowed_name)
                    .unwrap_or(&default_allowed);
                !node_has_ascendants_with_allow_name_attr(db.upcast(), node, allowed_name)
                    && is_rule_allowed_globally
                    && (self.include_compiler_generated_files || !is_compiler_plugin_generated_file)
            })
            .map(|diag| diag.0)
            .collect()
    }
}

/// Plugin with `declared_allows` matching these of [`CairoLint`] that does not emit diagnostics.
/// Add it when `CairoLint` is not present to avoid compiler warnings on unsupported
/// `allow` attribute arguments.
#[derive(Debug, Default)]
pub struct CairoLintAllow;

impl AnalyzerPlugin for CairoLintAllow {
    fn diagnostics(&self, _db: &dyn SemanticGroup, _module_id: ModuleId) -> Vec<PluginDiagnostic> {
        Vec::new()
    }

    fn declared_allows(&self) -> Vec<String> {
        get_unique_allowed_names()
            .iter()
            .map(ToString::to_string)
            .collect()
    }
}

fn node_has_ascendants_with_allow_name_attr(
    db: &dyn SyntaxGroup,
    node: SyntaxNode,
    allowed_name: &'static str,
) -> bool {
    for node in node.ancestors_with_self(db) {
        if node.has_attr_with_arg(db, "allow", allowed_name) {
            return true;
        }
    }
    false
}

fn validate_cairo_lint_metadata(tool_metadata: &CairoLintToolMetadata) -> Result<()> {
    for (name, _) in tool_metadata.iter() {
        if !get_unique_allowed_names().contains(&name.as_str()) {
            return Err(anyhow!(
                "The lint '{}' specified in `Scarb.toml` is not supported by the Cairo lint.",
                name
            ));
        }
    }
    Ok(())
}
