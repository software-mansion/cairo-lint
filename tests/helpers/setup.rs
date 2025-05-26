use cairo_lang_filesystem::db::CrateSettings;
use cairo_lang_filesystem::{
    db::{CrateConfiguration, Edition, ExperimentalFeaturesConfig},
    ids::{CrateId, CrateLongId, Directory, FileKind, FileLongId, VirtualFile},
};
use cairo_lang_semantic::db::SemanticGroup;
use cairo_lang_utils::ordered_hash_map::OrderedHashMap;
use cairo_lang_utils::Intern;
use std::{collections::BTreeMap, sync::Arc};

use crate::CRATE_CONFIG;

pub fn setup_test_crate_ex(db: &mut dyn SemanticGroup, content: &str) -> CrateId {
    let file_id = FileLongId::Virtual(VirtualFile {
        parent: None,
        name: "lib.cairo".into(),
        content: content.into(),
        code_mappings: [].into(),
        kind: FileKind::Module,
        original_item_removed: false,
    })
    .intern(db);

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
    let crate_config = CrateConfiguration {
        root: Directory::Virtual {
            files: BTreeMap::from([("lib.cairo".into(), file_id)]),
            dirs: Default::default(),
        },
        settings,
        cache_file: None,
    };

    let crate_id = CrateLongId::Virtual {
        name: "test".into(),
        file_id,
        settings: CRATE_CONFIG.to_string(),
        cache_file: None,
    }
    .intern(db);

    db.set_crate_configs(Arc::new(OrderedHashMap::from([(crate_id, crate_config)])));

    crate_id
}
