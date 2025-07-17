use std::{collections::HashMap, f64::consts::E};

use cairo_lang_defs::ids::{
    FreeFunctionId, ImplDefId, ModuleId, ModuleItemId, TopLevelLanguageElementId,
};
use cairo_lang_filesystem::ids::CrateId;
use cairo_lang_semantic::db::SemanticGroup;

pub const BOOL_PARTIAL_EQ_PATH: &str = "core::BoolPartialEq";
pub const PANIC_PATH: &str = "core::panics::panic";
pub const PANIC_WITH_BYTE_ARRAY_PATH: &str = "core::panics::panic_with_byte_array";

static CORELIB_ITEM_PATHS: [&'static str; 3] =
    [BOOL_PARTIAL_EQ_PATH, PANIC_PATH, PANIC_WITH_BYTE_ARRAY_PATH];

pub struct CorelibContext {
    corelib_items: HashMap<String, Option<ModuleItemId>>,
}

impl CorelibContext {
    pub fn new(db: &dyn SemanticGroup) -> Self {
        let core_crate_id = CrateId::core(db);
        // eprintln!("Core crate ID: {:?}", core_crate_id);
        let modules = db.crate_modules(core_crate_id);
        Self {
            corelib_items: CORELIB_ITEM_PATHS
                .iter()
                .map(|path| {
                    for module in modules.iter() {
                        let item_id = find_item_with_path(db, *module, path);
                        if item_id.is_some() {
                            return (path.to_string(), item_id);
                        }
                    }

                    (path.to_string(), None)
                })
                .collect(),
        }
    }

    pub fn get_bool_partial_eq_trait_id(&self) -> ImplDefId {
        let item = self
            .corelib_items
            .get(BOOL_PARTIAL_EQ_PATH)
            .unwrap()
            .unwrap();
        match item {
            ModuleItemId::Impl(id) => id,
            _ => unreachable!("Expected BoolPartialEq to be an ImplDefId"),
        }
    }

    pub fn get_panic_function_id(&self) -> FreeFunctionId {
        let item = self.corelib_items.get(PANIC_PATH).unwrap().unwrap();
        match item {
            ModuleItemId::FreeFunction(id) => id,
            _ => unreachable!("Expected panic to be a FreeFunction"),
        }
    }

    pub fn get_panic_with_byte_array_function_id(&self) -> FreeFunctionId {
        let item = self
            .corelib_items
            .get(PANIC_WITH_BYTE_ARRAY_PATH)
            .unwrap()
            .unwrap();
        match item {
            ModuleItemId::FreeFunction(id) => id,
            _ => unreachable!("Expected panic_with_byte_array to be a FreeFunction"),
        }
    }
}

fn find_item_with_path(
    db: &dyn SemanticGroup,
    module_id: ModuleId,
    path: &str,
) -> Option<ModuleItemId> {
    let items = db.module_items(module_id).ok()?;
    for item in items.iter() {
        // eprintln!("Found item with path: {}", item.full_path(db));
        if item.full_path(db) == path {
            return Some(*item);
        }
        if let ModuleItemId::Submodule(submodule_id) = item {
            let submodule_item = find_item_with_path(db, ModuleId::Submodule(*submodule_id), path);
            if submodule_item.is_some() {
                return submodule_item;
            }
        }
    }
    None
}
