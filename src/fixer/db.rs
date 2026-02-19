use cairo_lang_defs::db::{defs_group_input, init_external_files};
use cairo_lang_filesystem::db::files_group_input;
use cairo_lang_semantic::db::semantic_group_input;

use salsa::{Database, Setter};

#[salsa::db]
#[derive(Clone)]
pub struct FixerDatabase {
    storage: salsa::Storage<Self>,
}

impl salsa::Database for FixerDatabase {}

impl FixerDatabase {
    pub fn new_from(db: &dyn Database) -> Self {
        let mut new_db = Self::new();
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
            .set_file_overrides(&mut new_db)
            .to(files_group_input(db).file_overrides(db).clone());
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

        new_db
    }

    fn new() -> Self {
        Self {
            storage: Default::default(),
        }
    }
}
