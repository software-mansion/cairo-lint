use cairo_lang_defs::{ids::ModuleItemId, plugin::PluginDiagnostic};
use cairo_lang_diagnostics::Severity;
use cairo_lang_semantic::db::SemanticGroup;
use cairo_lang_syntax::node::{
    ast::{self, OptionTypeClause},
    TypedStablePtr, TypedSyntaxNode,
};

use crate::context::{CairoLintKind, Lint};

pub struct EmptyEnumBracketsVariant;

/// ## What it does
///
/// Finds enum variants without fields that are declared and used with empty brackets.
///
/// ## Example
///
/// ```cairo
///  enum MyEnum {
///     Data: u8,
///     Empty: ()       // redundant parentheses
///  }
///  
///  let a = MyEnum::Empty(()); // redundant parentheses
/// ```
///
/// Can be simplified to:
///
/// ```cairo
///  enum MyEnum {
///     Data(u8),
///     Empty,
///  }
///  
///  let a = MyEnum::Empty;
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

    fn is_enabled(&self) -> bool {
        true
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

    for variant in variants {
        let semantic = db.variant_semantic(*enum_id, variant.1).unwrap();

        if semantic.ty.is_unit(db) {
            let node = variant.1.stable_ptr(db.upcast()).0.lookup(db.upcast());

            let ast_variant = ast::Variant::from_syntax_node(db.upcast(), node);

            if let OptionTypeClause::TypeClause(_) = ast_variant.type_clause(db.upcast()) {
                diagnostics.push(PluginDiagnostic {
                    stable_ptr: variant.1.stable_ptr(db.upcast()).untyped(),
                    message: EmptyEnumBracketsVariant.diagnostic_message().to_string(),
                    severity: Severity::Warning,
                });
            }
        }
    }
}
