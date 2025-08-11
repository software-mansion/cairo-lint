use cairo_lang_defs::db::{DefsGroup, try_ext_as_virtual_impl};
use cairo_lang_filesystem::{
    db::{ExternalFiles, FilesGroup},
    ids::VirtualFile,
};
use cairo_lang_parser::db::ParserGroup;
use cairo_lang_semantic::db::{Elongate, SemanticGroup};
use cairo_lang_syntax::node::db::SyntaxGroup;
use cairo_lang_utils::Upcast;

use crate::LinterGroup;

#[salsa::db]
#[derive(Clone)]
pub struct FixerDatabase {
    storage: salsa::Storage<Self>,
}

impl salsa::Database for FixerDatabase {}

impl FixerDatabase {
    pub fn new_from(db: &(dyn SemanticGroup + 'static)) -> Self {
        let mut new_db = Self::new();

        // SemanticGroup salsa inputs.
        new_db.set_default_analyzer_plugins_input(db.default_analyzer_plugins_input());
        new_db.set_analyzer_plugin_overrides_input(db.analyzer_plugin_overrides_input());

        // DefsGroup salsa inputs.
        new_db.set_default_macro_plugins_input(db.default_macro_plugins_input());
        new_db.set_macro_plugin_overrides_input(db.macro_plugin_overrides_input());
        new_db.set_default_inline_macro_plugins_input(db.default_inline_macro_plugins_input());
        new_db.set_inline_macro_plugin_overrides_input(db.inline_macro_plugin_overrides_input());

        // FilesGroup salsa inputs.
        new_db.set_crate_configs_input(db.crate_configs_input());
        new_db.set_file_overrides_input(db.file_overrides_input());
        new_db.set_flags_input(db.flags_input());
        new_db.set_cfg_set(db.cfg_set());
        new_db
    }

    fn new() -> Self {
        Self {
            storage: Default::default(),
        }
    }
}

impl ExternalFiles for FixerDatabase {
    fn try_ext_as_virtual(&self, external_id: salsa::Id) -> Option<VirtualFile<'_>> {
        try_ext_as_virtual_impl(self, external_id)
    }
}

impl<'db> Upcast<'db, dyn FilesGroup> for FixerDatabase {
    fn upcast(&self) -> &(dyn FilesGroup + 'static) {
        self
    }
}

impl<'db> Upcast<'db, dyn SyntaxGroup> for FixerDatabase {
    fn upcast(&self) -> &(dyn SyntaxGroup + 'static) {
        self
    }
}

impl<'db> Upcast<'db, dyn DefsGroup> for FixerDatabase {
    fn upcast(&self) -> &(dyn DefsGroup + 'static) {
        self
    }
}

impl<'db> Upcast<'db, dyn SemanticGroup> for FixerDatabase {
    fn upcast(&self) -> &(dyn SemanticGroup + 'static) {
        self
    }
}

impl<'db> Upcast<'db, dyn ParserGroup> for FixerDatabase {
    fn upcast(&self) -> &(dyn ParserGroup + 'static) {
        self
    }
}

impl<'db> Upcast<'db, dyn LinterGroup> for FixerDatabase {
    fn upcast(&self) -> &(dyn LinterGroup + 'static) {
        self
    }
}

impl Elongate for FixerDatabase {
    fn elongate(&self) -> &(dyn SemanticGroup + 'static) {
        self
    }
}
