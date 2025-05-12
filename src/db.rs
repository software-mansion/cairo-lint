use std::sync::Arc;

use cairo_lang_defs::db::{init_defs_group, try_ext_as_virtual_impl, DefsDatabase, DefsGroup};
use cairo_lang_filesystem::{
    db::{init_files_group, AsFilesGroupMut, ExternalFiles, FilesDatabase, FilesGroup},
    ids::VirtualFile,
};
use cairo_lang_parser::db::{ParserDatabase, ParserGroup};
use cairo_lang_semantic::db::{init_semantic_group, SemanticDatabase, SemanticGroup};
use cairo_lang_syntax::node::db::{SyntaxDatabase, SyntaxGroup};
use cairo_lang_utils::Upcast;

use crate::CairoLintGroup;

#[salsa::database(
    SemanticDatabase,
    DefsDatabase,
    SyntaxDatabase,
    FilesDatabase,
    ParserDatabase
)]
pub struct FixerDatabase {
    storage: salsa::Storage<Self>,
}

impl salsa::Database for FixerDatabase {}

impl FixerDatabase {
    pub fn new_from(db: &(dyn SemanticGroup + 'static)) -> Self {
        let mut new_db = Self::new();
        new_db.migrate_default_plugins(db);
        new_db
    }

    fn new() -> Self {
        let mut db = Self {
            storage: Default::default(),
        };

        init_files_group(&mut db);
        init_defs_group(&mut db);
        init_semantic_group(&mut db);

        db
    }

    fn migrate_default_plugins(&mut self, old_db: &(dyn SemanticGroup + 'static)) {
        self.set_default_macro_plugins(
            old_db
                .default_macro_plugins()
                .iter()
                .map(|&id| self.intern_macro_plugin(old_db.lookup_intern_macro_plugin(id)))
                .collect(),
        );

        self.set_default_analyzer_plugins(
            old_db
                .default_analyzer_plugins()
                .iter()
                .map(|&id| self.intern_analyzer_plugin(old_db.lookup_intern_analyzer_plugin(id)))
                .collect(),
        );

        self.set_default_inline_macro_plugins(Arc::new(
            old_db
                .default_inline_macro_plugins()
                .iter()
                .map(|(name, &id)| {
                    (
                        name.clone(),
                        self.intern_inline_macro_plugin(
                            old_db.lookup_intern_inline_macro_plugin(id),
                        ),
                    )
                })
                .collect(),
        ));
    }
}

impl ExternalFiles for FixerDatabase {
    fn try_ext_as_virtual(&self, external_id: salsa::InternId) -> Option<VirtualFile> {
        try_ext_as_virtual_impl(self.upcast(), external_id)
    }
}

impl salsa::ParallelDatabase for FixerDatabase {
    fn snapshot(&self) -> salsa::Snapshot<Self> {
        salsa::Snapshot::new(FixerDatabase {
            storage: self.storage.snapshot(),
        })
    }
}

impl AsFilesGroupMut for FixerDatabase {
    fn as_files_group_mut(&mut self) -> &mut (dyn FilesGroup + 'static) {
        self
    }
}

impl Upcast<dyn FilesGroup> for FixerDatabase {
    fn upcast(&self) -> &(dyn FilesGroup + 'static) {
        self
    }
}

impl Upcast<dyn SyntaxGroup> for FixerDatabase {
    fn upcast(&self) -> &(dyn SyntaxGroup + 'static) {
        self
    }
}

impl Upcast<dyn DefsGroup> for FixerDatabase {
    fn upcast(&self) -> &(dyn DefsGroup + 'static) {
        self
    }
}

impl Upcast<dyn SemanticGroup> for FixerDatabase {
    fn upcast(&self) -> &(dyn SemanticGroup + 'static) {
        self
    }
}

impl Upcast<dyn ParserGroup> for FixerDatabase {
    fn upcast(&self) -> &(dyn ParserGroup + 'static) {
        self
    }
}
