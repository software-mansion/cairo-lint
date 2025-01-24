use cairo_lang_defs::ids::{LanguageElementId, ModuleId};
use cairo_lang_defs::plugin::PluginDiagnostic;
use cairo_lang_filesystem::ids::FileLongId;
use cairo_lang_semantic::db::SemanticGroup;
use cairo_lang_semantic::plugin::{AnalyzerPlugin, PluginSuite};
use cairo_lang_utils::LookupIntern;

use crate::LINT_CONTEXT;

pub fn cairo_lint_plugin_suite() -> PluginSuite {
    let mut suite = PluginSuite::default();
    suite.add_analyzer_plugin::<CairoLint>();
    suite
}

#[derive(Debug, Default)]
pub struct CairoLint {
    include_compiler_generated_files: bool,
}

impl CairoLint {
    pub fn new(include_compiler_generated_files: bool) -> Self {
        Self {
            include_compiler_generated_files,
        }
    }
}

impl AnalyzerPlugin for CairoLint {
    fn declared_allows(&self) -> Vec<String> {
        LINT_CONTEXT
            .get_unique_allowed_names()
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

            let checking_functions = LINT_CONTEXT.get_all_checking_functions();

            for checking_function in checking_functions {
                checking_function(db, item, &mut diags);
            }
        }
        diags
    }
}
