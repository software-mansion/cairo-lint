use anyhow::{anyhow, Result};
use cairo_lang_defs::ids::{LanguageElementId, ModuleId};
use cairo_lang_defs::plugin::PluginDiagnostic;
use cairo_lang_filesystem::ids::FileLongId;
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
}

impl AnalyzerPlugin for CairoLint {
    fn declared_allows(&self) -> Vec<String> {
        get_unique_allowed_names()
            .iter()
            .map(ToString::to_string)
            .collect()
    }

    fn diagnostics(&self, db: &dyn SemanticGroup, module_id: ModuleId) -> Vec<PluginDiagnostic> {
        let mut diags = Vec::new();
        let Ok(items) = db.module_items(module_id) else {
            return diags;
        };
        for item in &*items {
            // Skip compiler generated files. By default it checks whether the item is inside the virtual or external file.
            if !self.include_compiler_generated_files
                && matches!(
                    item.stable_location(db.upcast())
                        .file_id(db.upcast())
                        .lookup_intern(db),
                    FileLongId::Virtual(_) | FileLongId::External(_)
                )
            {
                continue;
            }

            let checking_functions = get_all_checking_functions();

            for checking_function in checking_functions {
                checking_function(db, item, &mut diags);
            }
        }

        diags
            .into_iter()
            .filter(|diag| {
                let node = diag.stable_ptr.lookup(db.upcast());
                let allowed_name = get_name_for_diagnostic_message(&diag.message).unwrap();
                let default_allowed = is_lint_enabled_by_default(&diag.message).unwrap();
                !node_has_ascendants_with_allow_name_attr(db.upcast(), node, allowed_name)
                    && *self
                        .tool_metadata
                        .get(allowed_name)
                        .unwrap_or(&default_allowed)
            })
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
    for node in node.ancestors_with_self() {
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
