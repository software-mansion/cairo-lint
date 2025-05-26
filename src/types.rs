use cairo_lang_defs::ids::{GenericTypeId, ImportableId};
use cairo_lang_semantic::{
    db::SemanticGroup,
    items::{
        constant::ConstValueId,
        imp::{ImplId, ImplLongId},
    },
    GenericArgumentId, TypeId, TypeLongId,
};
use cairo_lang_utils::ordered_hash_map::OrderedHashMap;
use cairo_lang_utils::LookupIntern;
use itertools::Itertools;

enum InferredValue {
    Type(TypeId),
    Constant(ConstValueId),
    Impl(ImplId),
}

impl InferredValue {
    pub fn format(
        &self,
        db: &dyn SemanticGroup,
        importables: &OrderedHashMap<ImportableId, String>,
    ) -> String {
        match *self {
            InferredValue::Type(ty) => format_type(db, ty, importables),
            InferredValue::Constant(const_id) => const_id.format(db),
            InferredValue::Impl(impl_id) => format_impl(db, impl_id),
        }
    }

    pub fn try_from_generic_arg_id(generic: GenericArgumentId) -> Option<Self> {
        match generic {
            GenericArgumentId::Type(ty) => Some(InferredValue::Type(ty)),
            GenericArgumentId::Constant(const_id) => Some(InferredValue::Constant(const_id)),
            GenericArgumentId::Impl(impl_id) => Some(InferredValue::Impl(impl_id)),
            GenericArgumentId::NegImpl => None,
        }
    }
}

pub fn format_type(
    db: &dyn SemanticGroup,
    ty: TypeId,
    importables: &OrderedHashMap<ImportableId, String>,
) -> String {
    match ty.lookup_intern(db) {
        TypeLongId::Concrete(concrete_type) => {
            let importable = match concrete_type.generic_type(db) {
                GenericTypeId::Enum(enum_id) => ImportableId::Enum(enum_id),
                GenericTypeId::Struct(struct_id) => ImportableId::Struct(struct_id),
                GenericTypeId::Extern(extern_id) => ImportableId::ExternType(extern_id),
            };
            let path = importables
                .get(&importable)
                .cloned()
                .unwrap_or_else(|| concrete_type.generic_type(db).format(db));
            let generics = concrete_type.generic_args(db);

            if generics.is_empty() {
                path
            } else {
                let generics_list = generics
                    .into_iter()
                    .map(|generic| {
                        InferredValue::try_from_generic_arg_id(generic)
                            .map(|value| value.format(db, importables))
                            .unwrap_or_else(|| generic.format(db))
                    })
                    .join(", ");

                format!("{path}<{generics_list}>",)
            }
        }
        TypeLongId::Tuple(types) => {
            format!(
                "({})",
                types
                    .into_iter()
                    .map(|ty| format_type(db, ty, importables))
                    .join(", ")
            )
        }
        TypeLongId::Snapshot(ty) => {
            format!("@{}", format_type(db, ty, importables))
        }
        TypeLongId::FixedSizeArray { type_id, size } => {
            format!(
                "[{}; {}]",
                format_type(db, type_id, importables),
                size.format(db)
            )
        }
        TypeLongId::Closure(closure) => {
            format!(
                "fn ({}) -> {}",
                closure
                    .param_tys
                    .into_iter()
                    .map(|ty| format_type(db, ty, importables))
                    .join(", "),
                format_type(db, closure.ret_ty, importables)
            )
        }
        _ => ty.format(db),
    }
}

fn format_impl(db: &dyn SemanticGroup, impl_id: ImplId) -> String {
    // Translate unresolved impl to `<missing>` instead of printing its salsa ID.
    if matches!(impl_id.lookup_intern(db), ImplLongId::ImplVar(_)) {
        "<missing>".to_string()
    } else {
        impl_id.format(db)
    }
}
