use cairo_lang_defs::db::{defs_group_input, init_external_files};
use cairo_lang_filesystem::db::{
    GranularFileContentStorage, GranularFileContentView, files_group_input,
    new_granular_file_content_storage, register_files_group_view, set_editor_file_content_for_input,
    set_generated_file_content_for_input, snapshot_granular_file_contents,
};
use cairo_lang_lowering::{db::init_lowering_group, optimizations::config::Optimizations};
use cairo_lang_semantic::db::semantic_group_input;
use salsa::{Database, Setter};

#[salsa::db]
#[derive(Clone)]
pub struct FixerDatabase {
    storage: salsa::Storage<Self>,
    granular_file_contents: GranularFileContentStorage,
}

impl salsa::Database for FixerDatabase {}
impl GranularFileContentView for FixerDatabase {
    fn granular_file_content_storage(&self) -> Option<&GranularFileContentStorage> {
        Some(&self.granular_file_contents)
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
        semantic_group_input(&new_db)
            .set_analyzer_plugin_overrides(&mut new_db)
            .to(semantic_group_input(db)
                .analyzer_plugin_overrides(db)
                .clone());

        // DefsGroup salsa inputs.
        defs_group_input(&new_db)
            .set_default_macro_plugins(&mut new_db)
            .to(defs_group_input(db).default_macro_plugins(db).clone());
        defs_group_input(&new_db)
            .set_macro_plugin_overrides(&mut new_db)
            .to(defs_group_input(db).macro_plugin_overrides(db).clone());
        defs_group_input(&new_db)
            .set_default_inline_macro_plugins(&mut new_db)
            .to(defs_group_input(db)
                .default_inline_macro_plugins(db)
                .clone());
        defs_group_input(&new_db)
            .set_inline_macro_plugin_overrides(&mut new_db)
            .to(defs_group_input(db)
                .inline_macro_plugin_overrides(db)
                .clone());

        // FilesGroup salsa inputs.
        files_group_input(&new_db)
            .set_crate_configs(&mut new_db)
            .to(files_group_input(db).crate_configs(db).clone());
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
        };
        register_files_group_view(&db);
        db
    }
}
