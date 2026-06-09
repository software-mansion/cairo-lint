use anyhow::{Result, anyhow};
use cairo_lang_compiler::{
    db::validate_corelib,
    project::{ProjectConfig, update_crate_roots_from_project_config},
};
use cairo_lang_defs::{
    db::{defs_group_input, init_defs_group, init_external_files},
    ids::{InlineMacroExprPluginLongId, MacroPluginLongId},
};
use cairo_lang_filesystem::flag::FlagsGroup;
use cairo_lang_filesystem::{
    cfg::CfgSet,
    db::{FilesGroup, init_dev_corelib, init_files_group},
    detect::detect_corelib,
    flag::Flag,
    ids::FlagLongId,
};
use cairo_lang_lowering::{db::init_lowering_group, optimizations::config::Optimizations};
use cairo_lang_semantic::{
    db::{init_semantic_group, semantic_group_input},
    ids::AnalyzerPluginLongId,
    inline_macros::get_default_plugin_suite,
    plugin::PluginSuite,
};

use crate::plugin::cairo_lint_allow_plugin_suite;
use salsa::Setter;

#[salsa::db]
#[derive(Clone)]
pub struct LinterAnalysisDatabase {
    storage: salsa::Storage<Self>,
}

impl LinterAnalysisDatabase {
    pub fn builder() -> LinterAnalysisDatabaseBuilder {
        LinterAnalysisDatabaseBuilder::new()
    }

    fn new(mut default_plugin_suite: PluginSuite) -> Self {
        let mut res = Self {
            storage: Default::default(),
        };
        init_files_group(&mut res);
        init_defs_group(&mut res);
        init_semantic_group(&mut res);
        init_external_files(&mut res);
        init_lowering_group(
            &mut res,
            Optimizations::enabled_with_minimal_movable_functions(),
            None,
        );

        default_plugin_suite.add(cairo_lint_allow_plugin_suite());

        defs_group_input(&res)
            .set_default_macro_plugins(&mut res)
            .to(Some(
                default_plugin_suite
                    .plugins
                    .into_iter()
                    .map(MacroPluginLongId)
                    .collect(),
            ));
        defs_group_input(&res)
            .set_default_inline_macro_plugins(&mut res)
            .to(Some(
                default_plugin_suite
                    .inline_macro_plugins
                    .into_iter()
                    .map(|(name, value)| (name, InlineMacroExprPluginLongId(value)))
                    .collect(),
            ));
        semantic_group_input(&res)
            .set_default_analyzer_plugins(&mut res)
            .to(Some(
                default_plugin_suite
                    .analyzer_plugins
                    .into_iter()
                    .map(AnalyzerPluginLongId)
                    .collect(),
            ));
        res
    }
}

impl salsa::Database for LinterAnalysisDatabase {}

#[derive(Clone, Debug)]
pub struct LinterAnalysisDatabaseBuilder {
    default_plugin_suite: PluginSuite,
    detect_corelib: bool,
    auto_withdraw_gas: bool,
    panic_backtrace: bool,
    unsafe_panic: bool,
    project_config: Option<Box<ProjectConfig>>,
    cfg_set: Option<CfgSet>,
}

impl LinterAnalysisDatabaseBuilder {
    fn new() -> Self {
        Self {
            default_plugin_suite: get_default_plugin_suite(),
            detect_corelib: false,
            auto_withdraw_gas: true,
            panic_backtrace: false,
            unsafe_panic: false,
            project_config: None,
            cfg_set: None,
        }
    }

    pub fn with_default_plugin_suite(&mut self, suite: PluginSuite) -> &mut Self {
        self.default_plugin_suite.add(suite);
        self
    }

    pub fn clear_plugins(&mut self) -> &mut Self {
        self.default_plugin_suite = get_default_plugin_suite();
        self
    }

    pub fn detect_corelib(&mut self) -> &mut Self {
        self.detect_corelib = true;
        self
    }

    pub fn with_project_config(&mut self, config: ProjectConfig) -> &mut Self {
        self.project_config = Some(Box::new(config));
        self
    }

    pub fn with_cfg(&mut self, cfg_set: impl Into<CfgSet>) -> &mut Self {
        self.cfg_set = Some(cfg_set.into());
        self
    }

    pub fn skip_auto_withdraw_gas(&mut self) -> &mut Self {
        self.auto_withdraw_gas = false;
        self
    }

    pub fn with_panic_backtrace(&mut self) -> &mut Self {
        self.panic_backtrace = true;
        self
    }

    pub fn with_unsafe_panic(&mut self) -> &mut Self {
        self.unsafe_panic = true;
        self
    }

    pub fn build(&mut self) -> Result<LinterAnalysisDatabase> {
        // NOTE: Order of operations matters here!
        // Errors if something is not OK are very subtle, mostly this results in missing
        // identifier diagnostics, or panics regarding lack of corelib items.

        let mut db = LinterAnalysisDatabase::new(self.default_plugin_suite.clone());

        if let Some(cfg_set) = &self.cfg_set {
            db.use_cfg(cfg_set);
        }

        if self.detect_corelib {
            let path =
                detect_corelib().ok_or_else(|| anyhow!("Failed to find development corelib."))?;
            init_dev_corelib(&mut db, path)
        }

        db.set_flag(
            FlagLongId("add_withdraw_gas".to_string()),
            Some(Flag::AddWithdrawGas(self.auto_withdraw_gas)),
        );

        db.set_flag(
            FlagLongId("panic_backtrace".to_string()),
            Some(Flag::PanicBacktrace(self.panic_backtrace)),
        );

        db.set_flag(
            FlagLongId("unsafe_panic".to_string()),
            Some(Flag::UnsafePanic(self.unsafe_panic)),
        );

        if let Some(config) = &self.project_config {
            update_crate_roots_from_project_config(&mut db, config.as_ref());
        }
        validate_corelib(&db)?;

        Ok(db)
    }
}
