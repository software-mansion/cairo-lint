use cairo_lang_defs::db::{
    GranularInlineMacroPluginOverrideStorage, GranularInlineMacroPluginOverrideView,
    GranularMacroPluginOverrideStorage, GranularMacroPluginOverrideView, defs_group_input,
    init_external_files, new_granular_inline_macro_plugin_override_storage,
    new_granular_macro_plugin_override_storage, register_granular_inline_macro_plugin_override_view,
    register_granular_macro_plugin_override_view, snapshot_granular_inline_macro_plugin_overrides,
    snapshot_granular_macro_plugin_overrides, set_inline_macro_plugin_overrides_for_input,
    set_macro_plugin_overrides_for_input,
};
use cairo_lang_filesystem::db::{
    GranularCrateConfigStorage, GranularCrateConfigView, GranularFileContentStorage,
    GranularFileContentView, files_group_input,
    new_granular_crate_config_storage,
    new_granular_file_content_storage, register_files_group_view, set_editor_file_content_for_input,
    register_granular_crate_config_view, set_crate_config_for_input,
    set_generated_file_content_for_input, snapshot_granular_crate_configs,
    snapshot_granular_file_contents,
};
use cairo_lang_lowering::{db::init_lowering_group, optimizations::config::Optimizations};
use cairo_lang_semantic::db::{
    GranularAnalyzerPluginOverrideStorage, GranularAnalyzerPluginOverrideView,
    new_granular_analyzer_plugin_override_storage, register_granular_analyzer_plugin_override_view,
    semantic_group_input, set_analyzer_plugin_overrides_for_input,
    snapshot_granular_analyzer_plugin_overrides,
};
use salsa::{Database, Setter};

#[salsa::db]
#[derive(Clone)]
pub struct FixerDatabase {
    storage: salsa::Storage<Self>,
    granular_file_contents: GranularFileContentStorage,
    granular_crate_configs: GranularCrateConfigStorage,
    granular_macro_plugin_overrides: GranularMacroPluginOverrideStorage,
    granular_inline_macro_plugin_overrides: GranularInlineMacroPluginOverrideStorage,
    granular_analyzer_plugin_overrides: GranularAnalyzerPluginOverrideStorage,
}

impl salsa::Database for FixerDatabase {}
impl GranularFileContentView for FixerDatabase {
    fn granular_file_content_storage(&self) -> Option<&GranularFileContentStorage> {
        Some(&self.granular_file_contents)
    }
}
impl GranularCrateConfigView for FixerDatabase {
    fn granular_crate_config_storage(&self) -> Option<&GranularCrateConfigStorage> {
        Some(&self.granular_crate_configs)
    }
}
impl GranularMacroPluginOverrideView for FixerDatabase {
    fn granular_macro_plugin_override_storage(&self) -> Option<&GranularMacroPluginOverrideStorage> {
        Some(&self.granular_macro_plugin_overrides)
    }
}
impl GranularInlineMacroPluginOverrideView for FixerDatabase {
    fn granular_inline_macro_plugin_override_storage(
        &self,
    ) -> Option<&GranularInlineMacroPluginOverrideStorage> {
        Some(&self.granular_inline_macro_plugin_overrides)
    }
}
impl GranularAnalyzerPluginOverrideView for FixerDatabase {
    fn granular_analyzer_plugin_override_storage(
        &self,
    ) -> Option<&GranularAnalyzerPluginOverrideStorage> {
        Some(&self.granular_analyzer_plugin_overrides)
    }
}

impl FixerDatabase {
    pub fn new_from(db: &dyn Database) -> Self {
        let mut new_db = Self::new();

        init_lowering_group(
            &mut new_db,
            Optimizations::enabled_with_minimal_movable_functions(),
            None,
        );

        // SemanticGroup salsa inputs.
        semantic_group_input(&new_db)
            .set_default_analyzer_plugins(&mut new_db)
            .to(semantic_group_input(db)
                .default_analyzer_plugins(db)
                .clone());
        // DefsGroup salsa inputs.
        defs_group_input(&new_db)
            .set_default_macro_plugins(&mut new_db)
            .to(defs_group_input(db).default_macro_plugins(db).clone());
        defs_group_input(&new_db)
            .set_default_inline_macro_plugins(&mut new_db)
            .to(defs_group_input(db)
                .default_inline_macro_plugins(db)
                .clone());

        // FilesGroup salsa inputs.
        files_group_input(&new_db)
            .set_flags(&mut new_db)
            .to(files_group_input(db).flags(db).clone());
        files_group_input(&new_db)
            .set_cfg_set(&mut new_db)
            .to(files_group_input(db).cfg_set(db).clone());

        // Initiate it again instead of migrating because [`ExternalFiles.try_ext_as_virtual_obj`] is private.
        // We can do that since the only thing in this input is an `Arc` to a closure,
        // that is never supposed to be changed after the initialization.
        init_external_files(&mut new_db);

        for (crate_input, config) in snapshot_granular_crate_configs(db) {
            set_crate_config_for_input(&mut new_db, crate_input, Some(config));
        }

        for (crate_input, plugins) in snapshot_granular_macro_plugin_overrides(db) {
            set_macro_plugin_overrides_for_input(&mut new_db, crate_input, Some(plugins));
        }
        for (crate_input, plugins) in snapshot_granular_inline_macro_plugin_overrides(db) {
            set_inline_macro_plugin_overrides_for_input(&mut new_db, crate_input, Some(plugins));
        }
        for (crate_input, plugins) in snapshot_granular_analyzer_plugin_overrides(db) {
            set_analyzer_plugin_overrides_for_input(&mut new_db, crate_input, Some(plugins));
        }

        for (file_input, (editor_content, generated_content)) in snapshot_granular_file_contents(db)
        {
            if editor_content.is_some() {
                set_editor_file_content_for_input(
                    &mut new_db,
                    file_input.clone(),
                    editor_content,
                );
            }
            if generated_content.is_some() {
                set_generated_file_content_for_input(&mut new_db, file_input, generated_content);
            }
        }

        new_db
    }

    fn new() -> Self {
        let db = Self {
            storage: Default::default(),
            granular_file_contents: new_granular_file_content_storage(),
            granular_crate_configs: new_granular_crate_config_storage(),
            granular_macro_plugin_overrides: new_granular_macro_plugin_override_storage(),
            granular_inline_macro_plugin_overrides:
                new_granular_inline_macro_plugin_override_storage(),
            granular_analyzer_plugin_overrides: new_granular_analyzer_plugin_override_storage(),
        };
        register_files_group_view(&db);
        register_granular_crate_config_view(&db);
        register_granular_macro_plugin_override_view(&db);
        register_granular_inline_macro_plugin_override_view(&db);
        register_granular_analyzer_plugin_override_view(&db);
        db
    }
}
