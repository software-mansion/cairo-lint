use std::{collections::BTreeMap, sync::Arc};

use cairo_lang_defs::{
    db::{DefsDatabase, DefsGroup, try_ext_as_virtual_impl},
    ids::{InlineMacroExprPluginId, MacroPluginId},
};
use cairo_lang_filesystem::{
    db::{CrateConfiguration, ExternalFiles, FilesDatabase, FilesGroup},
    flag::Flag,
    ids::{CrateId, Directory, FileId, FlagId, VirtualFile},
};
use cairo_lang_parser::db::{ParserDatabase, ParserGroup};
use cairo_lang_semantic::{
    db::{SemanticDatabase, SemanticGroup},
    ids::AnalyzerPluginId,
};
use cairo_lang_syntax::node::db::{SyntaxDatabase, SyntaxGroup};
use cairo_lang_utils::Upcast;
use cairo_lang_utils::{Intern, ordered_hash_map::OrderedHashMap};
use cairo_lang_utils::{LookupIntern, smol_str::SmolStr};

use crate::{LinterDatabase, LinterGroup};

#[salsa::database(
    LinterDatabase,
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

        // SemanticGroup salsa inputs.
        new_db.migrate_default_analyzer_plugins(db);
        new_db.migrate_analyzer_plugin_overrides(db);

        // DefsGroup salsa inputs.
        new_db.migrate_default_macro_plugins(db);
        new_db.migrate_macro_plugin_overrides(db);
        new_db.migrate_default_inline_macro_plugins(db);
        new_db.migrate_inline_macro_plugin_overrides(db);

        // FilesGroup salsa inputs.
        new_db.migrate_crate_configs(db);
        new_db.migrate_file_overrides(db);
        new_db.migrate_flags(db);
        new_db.set_cfg_set(db.cfg_set());
        new_db
    }

    fn new() -> Self {
        Self {
            storage: Default::default(),
        }
    }

    fn migrate_default_analyzer_plugins(&mut self, old_db: &(dyn SemanticGroup + 'static)) {
        let new_ids = self.intern_analyzer_plugin_ids(old_db, &old_db.default_analyzer_plugins());

        self.set_default_analyzer_plugins(Arc::from(new_ids));
    }

    fn migrate_analyzer_plugin_overrides(&mut self, old_db: &(dyn SemanticGroup + 'static)) {
        let mut new_analyzer_plugin_overrides: OrderedHashMap<CrateId, Arc<[AnalyzerPluginId]>> =
            OrderedHashMap::default();
        old_db
            .analyzer_plugin_overrides()
            .iter()
            .for_each(|(crate_id, analyzer_plugin_ids)| {
                let new_ids = self.intern_analyzer_plugin_ids(old_db, analyzer_plugin_ids);
                let new_crate_id = crate_id.lookup_intern(old_db).intern(self);
                new_analyzer_plugin_overrides
                    .insert(new_crate_id, Arc::<[AnalyzerPluginId]>::from(new_ids));
            });
        self.set_analyzer_plugin_overrides(Arc::from(new_analyzer_plugin_overrides));
    }

    fn migrate_default_macro_plugins(&mut self, old_db: &(dyn SemanticGroup + 'static)) {
        let new_ids = self.intern_macro_plugin_ids(old_db, &old_db.default_macro_plugins());

        self.set_default_macro_plugins(Arc::from(new_ids));
    }

    fn migrate_macro_plugin_overrides(&mut self, old_db: &(dyn SemanticGroup + 'static)) {
        let mut new_macro_plugin_overrides: OrderedHashMap<CrateId, Arc<[MacroPluginId]>> =
            OrderedHashMap::default();
        old_db
            .macro_plugin_overrides()
            .iter()
            .for_each(|(crate_id, macro_plugin_ids)| {
                let new_ids = self.intern_macro_plugin_ids(old_db, macro_plugin_ids);
                let new_crate_id = crate_id.lookup_intern(old_db).intern(self);
                new_macro_plugin_overrides
                    .insert(new_crate_id, Arc::<[MacroPluginId]>::from(new_ids));
            });
        self.set_macro_plugin_overrides(Arc::from(new_macro_plugin_overrides));
    }

    fn migrate_default_inline_macro_plugins(&mut self, old_db: &(dyn SemanticGroup + 'static)) {
        let new_default_inline_macro_plugins =
            self.intern_inline_macro_plugin_ids(old_db, &old_db.default_inline_macro_plugins());

        self.set_default_inline_macro_plugins(Arc::from(new_default_inline_macro_plugins));
    }

    fn migrate_inline_macro_plugin_overrides(&mut self, old_db: &(dyn SemanticGroup + 'static)) {
        let mut new_inline_macro_plugin_overrides: OrderedHashMap<
            CrateId,
            Arc<OrderedHashMap<String, InlineMacroExprPluginId>>,
        > = OrderedHashMap::default();
        old_db.inline_macro_plugin_overrides().iter().for_each(
            |(crate_id, inline_macro_plugins)| {
                let new_inline_macro_plugin_ids =
                    self.intern_inline_macro_plugin_ids(old_db, inline_macro_plugins);
                let new_crate_id = crate_id.lookup_intern(old_db).intern(self);
                new_inline_macro_plugin_overrides
                    .insert(new_crate_id, Arc::from(new_inline_macro_plugin_ids));
            },
        );
        self.set_inline_macro_plugin_overrides(Arc::from(new_inline_macro_plugin_overrides));
    }

    fn migrate_crate_configs(&mut self, old_db: &(dyn SemanticGroup + 'static)) {
        let mut new_crate_configs: OrderedHashMap<CrateId, CrateConfiguration> =
            OrderedHashMap::default();
        old_db
            .crate_configs()
            .iter()
            .for_each(|(crate_id, crate_config)| {
                let new_crate_id = crate_id.lookup_intern(old_db).intern(self);
                let mut new_crate_config = crate_config.clone();

                if let Some(blob_id) = crate_config.cache_file {
                    new_crate_config.cache_file = Some(blob_id.lookup_intern(old_db).intern(self));
                }

                if matches!(crate_config.root, Directory::Virtual { .. }) {
                    new_crate_config.root =
                        self.get_migrated_root_directory(old_db, &mut new_crate_config.root)
                }

                new_crate_configs.insert(new_crate_id, crate_config.clone());
            });
        self.set_crate_configs(Arc::from(new_crate_configs));
    }

    fn migrate_file_overrides(&mut self, old_db: &(dyn SemanticGroup + 'static)) {
        let mut new_file_overrides: OrderedHashMap<FileId, Arc<str>> = OrderedHashMap::default();
        old_db
            .file_overrides()
            .iter()
            .for_each(|(file_id, content)| {
                let new_file_id = file_id.lookup_intern(old_db).intern(self);
                new_file_overrides.insert(new_file_id, content.clone());
            });
        self.set_file_overrides(Arc::from(new_file_overrides));
    }

    fn migrate_flags(&mut self, old_db: &(dyn SemanticGroup + 'static)) {
        let mut new_flags: OrderedHashMap<FlagId, Arc<Flag>> = OrderedHashMap::default();
        old_db.flags().iter().for_each(|(flag_id, flag)| {
            let new_flag_id = flag_id.lookup_intern(old_db).intern(self);
            new_flags.insert(new_flag_id, flag.clone());
        });
        self.set_flags(Arc::from(new_flags));
    }

    fn get_migrated_root_directory(
        &mut self,
        old_db: &(dyn SemanticGroup + 'static),
        dir: &mut Directory,
    ) -> Directory {
        match dir {
            Directory::Real(_) => dir.clone(),
            Directory::Virtual { files, dirs } => {
                let mut new_files: BTreeMap<SmolStr, FileId> = BTreeMap::default();
                let mut new_dirs: BTreeMap<SmolStr, Box<Directory>> = BTreeMap::default();
                for (name, file_id) in files.iter() {
                    let new_file_id = file_id.lookup_intern(old_db).intern(self);
                    new_files.insert(name.clone(), new_file_id);
                }
                for (name, subdir) in dirs.iter() {
                    let mut subdir_clone = (**subdir).clone();
                    let new_subdir = self.get_migrated_root_directory(old_db, &mut subdir_clone);
                    new_dirs.insert(name.clone(), Box::new(new_subdir));
                }
                Directory::Virtual {
                    files: new_files,
                    dirs: new_dirs,
                }
            }
        }
    }

    fn intern_analyzer_plugin_ids(
        &mut self,
        old_db: &(dyn SemanticGroup + 'static),
        analyzer_plugins_ids: &Arc<[AnalyzerPluginId]>,
    ) -> Vec<AnalyzerPluginId> {
        analyzer_plugins_ids
            .iter()
            .map(|plugin_id| plugin_id.lookup_intern(old_db).intern(self))
            .collect()
    }

    fn intern_macro_plugin_ids(
        &mut self,
        old_db: &(dyn DefsGroup + 'static),
        macro_plugins_ids: &Arc<[MacroPluginId]>,
    ) -> Vec<MacroPluginId> {
        macro_plugins_ids
            .iter()
            .map(|plugin_id| plugin_id.lookup_intern(old_db).intern(self))
            .collect()
    }

    fn intern_inline_macro_plugin_ids(
        &mut self,
        old_db: &(dyn DefsGroup + 'static),
        inline_macro_plugins_ids: &Arc<OrderedHashMap<String, InlineMacroExprPluginId>>,
    ) -> OrderedHashMap<String, InlineMacroExprPluginId> {
        inline_macro_plugins_ids
            .iter()
            .map(|(key, plugin_id)| {
                let new_plugin_id = plugin_id.lookup_intern(old_db).intern(self);
                (key.clone(), new_plugin_id)
            })
            .collect()
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

impl Upcast<dyn LinterGroup> for FixerDatabase {
    fn upcast(&self) -> &(dyn LinterGroup + 'static) {
        self
    }
}
