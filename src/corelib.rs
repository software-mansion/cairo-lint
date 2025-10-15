use cairo_lang_defs::ids::EnumId;
use cairo_lang_defs::{
    db::DefsGroup,
    ids::{
        ExternFunctionId, FreeFunctionId, ImplDefId, ImplItemId, LookupItemId, ModuleId,
        ModuleItemId, SubmoduleId, TopLevelLanguageElementId, TraitFunctionId, TraitItemId,
    },
};
use cairo_lang_filesystem::ids::CrateId;
use cairo_lang_semantic::items::imp::ImplSemantic;
use cairo_lang_semantic::items::trt::TraitSemantic;
use cairo_lang_utils::ordered_hash_map::OrderedHashMap;
use salsa::{Database, Update};

pub const BOOL_PARTIAL_EQ_PATH: &str = "core::BoolPartialEq";
pub const PANIC_PATH: &str = "core::panics::panic";
pub const PANIC_WITH_BYTE_ARRAY_PATH: &str = "core::panics::panic_with_byte_array";
pub const T_COPY_CLONE_PATH: &str = "core::clone::TCopyClone";
pub const PARTIAL_ORD_LE_PATH: &str = "core::traits::PartialOrd::le";
pub const PARTIAL_ORD_GE_PATH: &str = "core::traits::PartialOrd::ge";
pub const ADD_TRAIT_FUNCTION_PATH: &str = "core::traits::Add::add";
pub const SUB_TRAIT_FUNCTION_PATH: &str = "core::traits::Sub::sub";
pub const INTEGER_MODULE_PATH: &str = "core::integer";
pub const INTO_TRAIT_FUNCTION_PATH: &str = "core::traits::Into::into";
pub const TRY_INTO_TRAIT_FUNCTION_PATH: &str = "core::traits::TryInto::try_into";
pub const OPTION_TYPE_PATH: &str = "core::option::Option";

static CORELIB_ITEM_PATHS: [&str; 12] = [
    BOOL_PARTIAL_EQ_PATH,
    PANIC_PATH,
    PANIC_WITH_BYTE_ARRAY_PATH,
    T_COPY_CLONE_PATH,
    PARTIAL_ORD_LE_PATH,
    PARTIAL_ORD_GE_PATH,
    ADD_TRAIT_FUNCTION_PATH,
    SUB_TRAIT_FUNCTION_PATH,
    INTEGER_MODULE_PATH,
    OPTION_TYPE_PATH,
    INTO_TRAIT_FUNCTION_PATH,
    TRY_INTO_TRAIT_FUNCTION_PATH,
];

#[derive(PartialEq, Eq, Hash, Debug, Clone, Update)]
pub struct CorelibContext<'db> {
    corelib_items: OrderedHashMap<String, Option<LookupItemId<'db>>>,
}

impl<'db> CorelibContext<'db> {
    pub(crate) fn new(db: &'db dyn Database) -> Self {
        let core_crate_id = CrateId::core(db);
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

    // TODO (https://github.com/software-mansion/cairo-lint/issues/398): Write a macro for these getters to avoid boilerplate.
    pub fn get_bool_partial_eq_impl_id(&self) -> ImplDefId<'db> {
        let item = self
            .corelib_items
            .get(BOOL_PARTIAL_EQ_PATH)
            .expect("Expected BoolPartialEq to be present in corelib items")
            .expect("Expected BoolPartialEq to be defined in the corelib");
        match item {
            LookupItemId::ModuleItem(ModuleItemId::Impl(id)) => id,
            _ => unreachable!("Expected BoolPartialEq to be an ImplDefId"),
        }
    }

    pub fn get_panic_function_id(&self) -> ExternFunctionId<'db> {
        let item = self
            .corelib_items
            .get(PANIC_PATH)
            .expect("Expected panic to be present in corelib items")
            .expect("Expected panic to be defined in the corelib");
        match item {
            LookupItemId::ModuleItem(ModuleItemId::ExternFunction(id)) => id,
            _ => unreachable!("Expected panic to be a ExternFunction"),
        }
    }

    pub fn get_panic_with_byte_array_function_id(&self) -> FreeFunctionId<'db> {
        let item = self
            .corelib_items
            .get(PANIC_WITH_BYTE_ARRAY_PATH)
            .expect("Expected panic_with_byte_array to be present in corelib items")
            .expect("Expected panic_with_byte_array to be defined in the corelib");
        match item {
            LookupItemId::ModuleItem(ModuleItemId::FreeFunction(id)) => id,
            _ => unreachable!("Expected panic_with_byte_array to be a FreeFunction"),
        }
    }

    pub fn get_t_copy_clone_impl_id(&self) -> ImplDefId<'db> {
        let item = self
            .corelib_items
            .get(T_COPY_CLONE_PATH)
            .expect("Expected TCopyClone to be present in corelib items")
            .expect("Expected TCopyClone to be defined in the corelib");
        match item {
            LookupItemId::ModuleItem(ModuleItemId::Impl(id)) => id,
            _ => unreachable!("Expected TCopyClone to be an ImplDefId"),
        }
    }

    pub fn get_partial_ord_le_trait_function_id(&self) -> TraitFunctionId<'db> {
        let item = self
            .corelib_items
            .get(PARTIAL_ORD_LE_PATH)
            .expect("Expected PartialOrd::le to be present in corelib items")
            .expect("Expected PartialOrd::le to be defined in the corelib");
        match item {
            LookupItemId::TraitItem(TraitItemId::Function(id)) => id,
            _ => unreachable!("Expected PartialOrd::le to be a TraitFunctionId"),
        }
    }

    pub fn get_partial_ord_ge_trait_function_id(&self) -> TraitFunctionId<'db> {
        let item = self
            .corelib_items
            .get(PARTIAL_ORD_GE_PATH)
            .expect("Expected PartialOrd::ge to be present in corelib items")
            .expect("Expected PartialOrd::ge to be defined in the corelib");
        match item {
            LookupItemId::TraitItem(TraitItemId::Function(id)) => id,
            _ => unreachable!("Expected PartialOrd::ge to be a TraitFunctionId"),
        }
    }

    pub fn get_add_trait_function_id(&self) -> TraitFunctionId<'db> {
        let item = self
            .corelib_items
            .get(ADD_TRAIT_FUNCTION_PATH)
            .expect("Expected Add::add to be present in corelib items")
            .expect("Expected Add::add to be defined in the corelib");
        match item {
            LookupItemId::TraitItem(TraitItemId::Function(id)) => id,
            _ => unreachable!("Expected Add::add to be a TraitFunctionId"),
        }
    }

    pub fn get_sub_trait_function_id(&self) -> TraitFunctionId<'db> {
        let item = self
            .corelib_items
            .get(SUB_TRAIT_FUNCTION_PATH)
            .expect("Expected Sub::sub to be present in corelib items")
            .expect("Expected Sub::sub to be defined in the corelib");
        match item {
            LookupItemId::TraitItem(TraitItemId::Function(id)) => id,
            _ => unreachable!("Expected Sub::sub to be a TraitFunctionId"),
        }
    }

    pub fn get_integer_module_id(&self) -> SubmoduleId<'db> {
        let item = self
            .corelib_items
            .get(INTEGER_MODULE_PATH)
            .expect("Expected integer module to be present in corelib items")
            .expect("Expected integer module to be defined in the corelib");
        match item {
            LookupItemId::ModuleItem(ModuleItemId::Submodule(id)) => id,
            _ => unreachable!("Expected integer module to be a Submodule"),
        }
    }

    pub fn get_into_trait_function_id(&self) -> TraitFunctionId<'db> {
        let item = self
            .corelib_items
            .get(INTO_TRAIT_FUNCTION_PATH)
            .expect("Expected Into::into to be present in corelib items")
            .expect("Expected Into::into to be defined in the corelib");
        match item {
            LookupItemId::TraitItem(TraitItemId::Function(id)) => id,
            _ => unreachable!("Expected Into::into to be a TraitFunctionId"),
        }
    }

    pub fn get_try_into_trait_function_id(&self) -> TraitFunctionId<'db> {
        let item = self
            .corelib_items
            .get(TRY_INTO_TRAIT_FUNCTION_PATH)
            .expect("Expected TryInto::try_into to be present in corelib items")
            .expect("Expected TryInto::try_into to be defined in the corelib");
        match item {
            LookupItemId::TraitItem(TraitItemId::Function(id)) => id,
            _ => unreachable!("Expected TryInto::try_into to be a TraitFunctionId"),
        }
    }
    pub fn get_option_enum_id(&self) -> EnumId<'db> {
        let item = self
            .corelib_items
            .get(OPTION_TYPE_PATH)
            .expect("Expected Option to be present in corelib items")
            .expect("Expected Option to be defined in the corelib");
        match item {
            LookupItemId::ModuleItem(ModuleItemId::Enum(id)) => id,
            _ => unreachable!("Expected Option to be a EnumId"),
        }
    }
}

fn find_item_with_path<'db>(
    db: &'db dyn Database,
    module_id: ModuleId<'db>,
    path: &str,
) -> Option<LookupItemId<'db>> {
    let items = module_id.module_data(db).ok()?.items(db);
    for item in items.iter() {
        if item.full_path(db) == path {
            return Some(LookupItemId::ModuleItem(*item));
        }
        match item {
            ModuleItemId::Submodule(submodule_id) => {
                let submodule_item =
                    find_item_with_path(db, ModuleId::Submodule(*submodule_id), path);
                if submodule_item.is_some() {
                    return submodule_item;
                }
            }
            ModuleItemId::Impl(impl_id) => {
                if let Ok(functions) = db.impl_functions(*impl_id) {
                    for (_, impl_fn_id) in functions.iter() {
                        if impl_fn_id.full_path(db) == path {
                            return Some(LookupItemId::ImplItem(ImplItemId::Function(*impl_fn_id)));
                        }
                    }
                }

                if let Ok(types) = db.impl_types(*impl_id) {
                    for (impl_type_id, _) in types.iter() {
                        if impl_type_id.full_path(db) == path {
                            return Some(LookupItemId::ImplItem(ImplItemId::Type(*impl_type_id)));
                        }
                    }
                }

                if let Ok(consts) = db.impl_constants(*impl_id) {
                    for (impl_const_id, _) in consts.iter() {
                        if impl_const_id.full_path(db) == path {
                            return Some(LookupItemId::ImplItem(ImplItemId::Constant(
                                *impl_const_id,
                            )));
                        }
                    }
                }
            }
            ModuleItemId::Trait(trait_id) => {
                if let Ok(functions) = db.trait_functions(*trait_id) {
                    for (_, trait_fn_id) in functions.iter() {
                        if trait_fn_id.full_path(db) == path {
                            return Some(LookupItemId::TraitItem(TraitItemId::Function(
                                *trait_fn_id,
                            )));
                        }
                    }
                }

                if let Ok(types) = db.trait_types(*trait_id) {
                    for (_, trait_type_id) in types.iter() {
                        if trait_type_id.full_path(db) == path {
                            return Some(LookupItemId::TraitItem(TraitItemId::Type(
                                *trait_type_id,
                            )));
                        }
                    }
                }

                if let Ok(consts) = db.trait_constants(*trait_id) {
                    for (_, trait_const_id) in consts.iter() {
                        if trait_const_id.full_path(db) == path {
                            return Some(LookupItemId::TraitItem(TraitItemId::Constant(
                                *trait_const_id,
                            )));
                        }
                    }
                }
            }
            // The check for the item path happens before all the matches.
            _ => (),
        }
    }
    None
}
