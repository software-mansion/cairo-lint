use cairo_lang_filesystem::db::{CrateConfigurationInput, CrateSettings, FilesGroup};
use cairo_lang_filesystem::{
    db::{Edition, ExperimentalFeaturesConfig},
    ids::FileKind,
};
use cairo_lang_utils::ordered_hash_map::OrderedHashMap;
use std::{collections::BTreeMap, sync::Arc};

use crate::CRATE_CONFIG;
use cairo_lang_filesystem::ids::{CrateInput, DirectoryInput, FileInput, VirtualFileInput};
use cairo_lint::LinterAnalysisDatabase;

pub fn setup_test_crate_ex(db: &mut LinterAnalysisDatabase, content: &str) -> CrateInput {
    let settings = CrateSettings {
        name: None,
        edition: Edition::latest(),
        version: None,
        dependencies: Default::default(),
        experimental_features: ExperimentalFeaturesConfig {
            negative_impls: true,
            associated_item_constraints: true,
            coupons: true,
            user_defined_inline_macros: true,
        },
        cfg_set: Default::default(),
    };
    let file = FileInput::Virtual(VirtualFileInput {
        parent: None,
        name: "lib.cairo".into(),
        content: content.into(),
        code_mappings: [].into(),
        kind: FileKind::Module,
        original_item_removed: false,
    });

    let cr = CrateInput::Virtual {
        name: "test".into(),
        file_long_id: file.clone(),
        settings: CRATE_CONFIG.to_string(),
        cache_file: None,
    };

    db.set_crate_configs_input(Arc::new(OrderedHashMap::from([(
        cr.clone(),
        CrateConfigurationInput {
            root: DirectoryInput::Virtual {
                files: BTreeMap::from([("lib.cairo".to_string(), file)]),
                dirs: Default::default(),
            },
            settings,
            cache_file: None,
        },
    )])));

    cr
}
