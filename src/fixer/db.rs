use cairo_lang_defs::db::{
    InlineMacroPluginOverrideStorage, InlineMacroPluginOverrideView, MacroPluginOverrideStorage,
    MacroPluginOverrideView, defs_group_input, init_external_files,
    register_inline_macro_plugin_override_view, register_macro_plugin_override_view,
    set_inline_macro_plugin_overrides_for_input, set_macro_plugin_overrides_for_input,
    snapshot_inline_macro_plugin_overrides, snapshot_macro_plugin_overrides,
};
use cairo_lang_filesystem::db::{
    CrateConfigStorage, CrateConfigView, FileContentStorage, FileContentView, files_group_input,
    new_crate_config_storage, new_file_content_storage, register_crate_config_view,
    register_files_group_view, set_crate_config_for_input, set_generated_file_content_for_input,
    set_on_disk_file_content_for_input, snapshot_crate_configs, snapshot_file_contents,
};
use cairo_lang_lowering::{db::init_lowering_group, optimizations::config::Optimizations};
use cairo_lang_semantic::db::{
    AnalyzerPluginOverrideStorage, AnalyzerPluginOverrideView,
    register_analyzer_plugin_override_view, semantic_group_input,
    set_analyzer_plugin_overrides_for_input, snapshot_analyzer_plugin_overrides,
};
use salsa::{Database, Setter};

#[salsa::db]
#[derive(Clone)]
pub struct FixerDatabase {
    storage: salsa::Storage<Self>,
    file_contents: FileContentStorage,
    crate_configs: CrateConfigStorage,
    macro_plugin_overrides: MacroPluginOverrideStorage,
    inline_macro_plugin_overrides: InlineMacroPluginOverrideStorage,
    analyzer_plugin_overrides: AnalyzerPluginOverrideStorage,
}

impl salsa::Database for FixerDatabase {}
impl FileContentView for FixerDatabase {
    fn file_content_storage(&self) -> Option<&FileContentStorage> {
        Some(&self.file_contents)
    }
}
impl CrateConfigView for FixerDatabase {
    fn crate_config_storage(&self) -> Option<&CrateConfigStorage> {
        Some(&self.crate_configs)
    }
}
impl MacroPluginOverrideView for FixerDatabase {
    fn macro_plugin_override_storage(&self) -> Option<&MacroPluginOverrideStorage> {
        Some(&self.macro_plugin_overrides)
    }
}
impl InlineMacroPluginOverrideView for FixerDatabase {
    fn inline_macro_plugin_override_storage(&self) -> Option<&InlineMacroPluginOverrideStorage> {
        Some(&self.inline_macro_plugin_overrides)
    }
}
impl AnalyzerPluginOverrideView for FixerDatabase {
    fn analyzer_plugin_override_storage(&self) -> Option<&AnalyzerPluginOverrideStorage> {
        Some(&self.analyzer_plugin_overrides)
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

        for (crate_input, config) in snapshot_crate_configs(db) {
            set_crate_config_for_input(&mut new_db, crate_input, Some(config));
        }

        for (crate_input, plugins) in snapshot_macro_plugin_overrides(db) {
            set_macro_plugin_overrides_for_input(&mut new_db, crate_input, Some(plugins));
        }
        for (crate_input, plugins) in snapshot_inline_macro_plugin_overrides(db) {
            set_inline_macro_plugin_overrides_for_input(&mut new_db, crate_input, Some(plugins));
        }
        for (crate_input, plugins) in snapshot_analyzer_plugin_overrides(db) {
            set_analyzer_plugin_overrides_for_input(&mut new_db, crate_input, Some(plugins));
        }

        for (file_input, (editor_content, generated_content)) in snapshot_file_contents(db) {
            if editor_content.is_some() {
                set_on_disk_file_content_for_input(&mut new_db, file_input.clone(), editor_content);
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
            file_contents: new_file_content_storage(),
            crate_configs: new_crate_config_storage(),
            macro_plugin_overrides: Default::default(),
            inline_macro_plugin_overrides: Default::default(),
            analyzer_plugin_overrides: Default::default(),
        };
        register_files_group_view(&db);
        register_crate_config_view(&db);
        register_macro_plugin_override_view(&db);
        register_inline_macro_plugin_override_view(&db);
        register_analyzer_plugin_override_view(&db);
        db
    }
}
