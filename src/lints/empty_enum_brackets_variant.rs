use cairo_lang_defs::{ids::ModuleItemId, plugin::PluginDiagnostic};
use cairo_lang_diagnostics::Severity;
use cairo_lang_semantic::db::SemanticGroup;
use cairo_lang_syntax::node::{ast::OptionTypeClause, TypedStablePtr};

use crate::context::{CairoLintKind, Lint};

pub struct EmptyEnumBracketsVariant;

/// ## What it does
///
/// Finds enum variants that are declared with empty brackets.
///
/// ## Example
///
/// ```cairo
///  enum MyEnum {
///     Data: u8,
///     Empty: ()       // redundant parentheses
///  }
/// ```
///
/// Can be simplified to:
///
/// ```cairo
///  enum MyEnum {
///     Data(u8),
///     Empty,
///  }
/// ```
impl Lint for EmptyEnumBracketsVariant {
    fn allowed_name(&self) -> &'static str {
        "empty_enum_brackets_variant"
    }

    fn diagnostic_message(&self) -> &'static str {
        "redundant parentheses in enum variant definition"
    }

    fn kind(&self) -> CairoLintKind {
        CairoLintKind::EnumEmptyVariantBrackets
    }
}

pub fn check_empty_enum_brackets_variant(
    db: &dyn SemanticGroup,
    item: &ModuleItemId,
    diagnostics: &mut Vec<PluginDiagnostic>,
) {
    let ModuleItemId::Enum(enum_id) = item else {
        return;
    };

    let Ok(variants) = db.enum_variants(*enum_id) else {
        return;
    };

    for variant in variants.values() {
        let semantic_variant = db.variant_semantic(*enum_id, *variant).unwrap();

        // Check if the variant is of unit type `()`
        if semantic_variant.ty.is_unit(db) {
            let ast_variant = variant.stable_ptr(db.upcast()).lookup(db.upcast());

            // Determine if the variant includes a type clause, or if the type clause is empty
            if let OptionTypeClause::TypeClause(_) = ast_variant.type_clause(db.upcast()) {
                diagnostics.push(PluginDiagnostic {
                    stable_ptr: variant.stable_ptr(db.upcast()).untyped(),
                    message: EmptyEnumBracketsVariant.diagnostic_message().to_string(),
                    severity: Severity::Warning,
                });
            }
        }
    }
}
