use std::{
    cell::UnsafeCell,
    ops::{Deref, DerefMut},
    sync::Arc,
};

use anyhow::{Result, anyhow};
use cairo_lang_compiler::{
    db::validate_corelib,
    project::{ProjectConfig, update_crate_roots_from_project_config},
};
use cairo_lang_defs::db::{DefsGroup, init_defs_group, try_ext_as_virtual_impl};
use cairo_lang_filesystem::{
    cfg::CfgSet,
    db::{ExternalFiles, FilesGroup, FilesGroupEx, init_dev_corelib, init_files_group},
    detect::detect_corelib,
    flag::Flag,
    ids::{FlagLongId, VirtualFile},
};
use cairo_lang_lowering::{
    db::{ExternalCodeSizeEstimator, LoweringGroup, init_lowering_group},
    utils::InliningStrategy,
};
use cairo_lang_parser::db::ParserGroup;
use cairo_lang_semantic::{
    db::{Elongate, PluginSuiteInput, SemanticGroup, init_semantic_group},
    inline_macros::get_default_plugin_suite,
    plugin::PluginSuite,
};
use cairo_lang_syntax::node::db::SyntaxGroup;
use cairo_lang_utils::{Upcast, smol_str::ToSmolStr};

use crate::{LinterGroup, plugin::cairo_lint_allow_plugin_suite};

#[salsa::db]
#[derive(Clone)]
pub struct LinterAnalysisDatabase {
    storage: salsa::Storage<Self>,
}

impl LinterAnalysisDatabase {
    pub fn builder() -> LinterAnalysisDatabaseBuilder {
        LinterAnalysisDatabaseBuilder::new()
    }

    fn new(default_plugin_suite: PluginSuite, inlining_strategy: InliningStrategy) -> Self {
        let mut default_plugin_suite = default_plugin_suite;
        let mut res = Self {
            storage: Default::default(),
        };
        init_files_group(&mut res);
        init_lowering_group(&mut res, inlining_strategy);
        init_defs_group(&mut res);
        init_semantic_group(&mut res);

        default_plugin_suite.add(cairo_lint_allow_plugin_suite());

        res.set_default_plugins_from_suite(default_plugin_suite);

        res
    }
}

impl salsa::Database for LinterAnalysisDatabase {}
impl ExternalFiles for LinterAnalysisDatabase {
    fn try_ext_as_virtual(&self, external_id: salsa::Id) -> Option<VirtualFile> {
        try_ext_as_virtual_impl(self, external_id)
    }
}

// We don't need this implementation at the moment but it's required by `LoweringGroup`.
impl ExternalCodeSizeEstimator for LinterAnalysisDatabase {
    fn estimate_size(
        &self,
        _function_id: cairo_lang_lowering::ids::ConcreteFunctionWithBodyId,
    ) -> cairo_lang_diagnostics::Maybe<isize> {
        cairo_lang_diagnostics::Maybe::Ok(0)
    }
}

impl<'db> Upcast<'db, dyn FilesGroup> for LinterAnalysisDatabase {
    fn upcast(&self) -> &(dyn FilesGroup + 'static) {
        self
    }
}

impl<'db> Upcast<'db, dyn SyntaxGroup> for LinterAnalysisDatabase {
    fn upcast(&self) -> &(dyn SyntaxGroup + 'static) {
        self
    }
}

impl<'db> Upcast<'db, dyn DefsGroup> for LinterAnalysisDatabase {
    fn upcast(&self) -> &(dyn DefsGroup + 'static) {
        self
    }
}

impl<'db> Upcast<'db, dyn SemanticGroup> for LinterAnalysisDatabase {
    fn upcast(&self) -> &(dyn SemanticGroup + 'static) {
        self
    }
}

impl<'db> Upcast<'db, dyn LoweringGroup> for LinterAnalysisDatabase {
    fn upcast(&self) -> &(dyn LoweringGroup + 'static) {
        self
    }
}

impl<'db> Upcast<'db, dyn ParserGroup> for LinterAnalysisDatabase {
    fn upcast(&self) -> &(dyn ParserGroup + 'static) {
        self
    }
}

impl<'db> Upcast<'db, dyn LinterGroup> for LinterAnalysisDatabase {
    fn upcast(&self) -> &(dyn LinterGroup + 'static) {
        self
    }
}

impl Elongate for LinterAnalysisDatabase {
    fn elongate(&self) -> &(dyn SemanticGroup + 'static) {
        self
    }
}

#[derive(Clone, Debug)]
pub struct LinterAnalysisDatabaseBuilder {
    default_plugin_suite: PluginSuite,
    detect_corelib: bool,
    auto_withdraw_gas: bool,
    panic_backtrace: bool,
    unsafe_panic: bool,
    project_config: Option<Box<ProjectConfig>>,
    cfg_set: Option<CfgSet>,
    inlining_strategy: InliningStrategy,
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
            inlining_strategy: InliningStrategy::Default,
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

    pub fn with_inlining_strategy(&mut self, inlining_strategy: InliningStrategy) -> &mut Self {
        self.inlining_strategy = inlining_strategy;
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

    pub fn build(&mut self) -> Result<DBWrapper> {
        // NOTE: Order of operations matters here!
        // Errors if something is not OK are very subtle, mostly this results in missing
        // identifier diagnostics, or panics regarding lack of corelib items.

        let mut db =
            LinterAnalysisDatabase::new(self.default_plugin_suite.clone(), self.inlining_strategy);

        if let Some(cfg_set) = &self.cfg_set {
            db.use_cfg(cfg_set);
        }

        if self.detect_corelib {
            let path =
                detect_corelib().ok_or_else(|| anyhow!("Failed to find development corelib."))?;
            init_dev_corelib(&mut db, path)
        }

        db.set_flag(
            FlagLongId("add_withdraw_gas".to_smolstr()),
            Some(Arc::new(Flag::AddWithdrawGas(self.auto_withdraw_gas))),
        );

        db.set_flag(
            FlagLongId("panic_backtrace".to_smolstr()),
            Some(Arc::new(Flag::PanicBacktrace(self.panic_backtrace))),
        );

        db.set_flag(
            FlagLongId("unsafe_panic".to_smolstr()),
            Some(Arc::new(Flag::UnsafePanic(self.unsafe_panic))),
        );

        if let Some(config) = &self.project_config {
            update_crate_roots_from_project_config(&mut db, config.as_ref());
        }
        validate_corelib(&db)?;

        Ok(DBWrapper::new(db))
    }
}

pub struct DBWrapper(UnsafeCell<LinterAnalysisDatabase>);

impl DBWrapper {
    fn new(db: LinterAnalysisDatabase) -> Self {
        Self(UnsafeCell::new(db))
    }

    #[allow(clippy::mut_from_ref)]
    pub fn get_mut(&self) -> &mut LinterAnalysisDatabase {
        //TODO, rework test macros so it will be unnecessary
        unsafe { &mut *self.0.get() }
    }
}

impl Deref for DBWrapper {
    type Target = LinterAnalysisDatabase;

    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0.get() }
    }
}

impl DerefMut for DBWrapper {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.get_mut()
    }
}
