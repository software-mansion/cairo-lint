use cairo_lang_filesystem::db::{
    CrateConfigurationInput, CrateSettings, set_crate_config_for_input,
};
use cairo_lang_filesystem::{
    db::{Edition, ExperimentalFeaturesConfig},
    ids::FileKind,
};
use std::collections::BTreeMap;

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
            repr_ptrs: true,
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

    let crate_config = CrateConfigurationInput {
        root: DirectoryInput::Virtual {
            files: BTreeMap::from([("lib.cairo".to_string(), file)]),
            dirs: Default::default(),
        },
        settings,
        cache_file: None,
    };

    set_crate_config_for_input(db, cr.clone(), Some(crate_config));

    cr
}
