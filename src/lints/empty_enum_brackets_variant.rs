use cairo_lang_defs::{ids::ModuleItemId, plugin::PluginDiagnostic};
use cairo_lang_diagnostics::Severity;
use cairo_lang_semantic::items::enm::EnumSemantic;
use cairo_lang_syntax::node::{
    SyntaxNode, TypedStablePtr, TypedSyntaxNode,
    ast::{self, OptionTypeClause},
    db::SyntaxGroup,
};

use crate::{
    LinterGroup,
    context::{CairoLintKind, Lint},
    fixer::InternalFix,
};

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

    fn has_fixer(&self) -> bool {
        true
    }

    fn fix<'db>(
        &self,
        db: &'db dyn LinterGroup,
        node: SyntaxNode<'db>,
    ) -> Option<InternalFix<'db>> {
        fix_empty_enum_brackets_variant(db, node)
    }

    fn fix_message(&self) -> Option<&'static str> {
        Some("Remove unit type definition from enum variant")
    }
}

#[tracing::instrument(skip_all, level = "trace")]
pub fn check_empty_enum_brackets_variant<'db>(
    db: &'db dyn LinterGroup,
    item: &ModuleItemId<'db>,
    diagnostics: &mut Vec<PluginDiagnostic<'db>>,
) {
    let ModuleItemId::Enum(enum_id) = item else {
        return;
    };

    let Ok(variants) = db.enum_variants(*enum_id) else {
        return;
    };

    for variant in variants.values() {
        let Ok(semantic_variant) = db.variant_semantic(*enum_id, *variant) else {
            return;
        };

        // Check if the variant is of unit type `()`
        if semantic_variant.ty.is_unit(db) {
            let ast_variant = variant.stable_ptr(db).lookup(db);

            // Determine if the variant includes a type clause, or if the type clause is empty
            if let OptionTypeClause::TypeClause(_) = ast_variant.type_clause(db) {
                diagnostics.push(PluginDiagnostic {
                    stable_ptr: variant.stable_ptr(db).untyped(),
                    message: EmptyEnumBracketsVariant.diagnostic_message().to_string(),
                    severity: Severity::Warning,
                    inner_span: None,
                });
            }
        }
    }
}

#[tracing::instrument(skip_all, level = "trace")]
fn fix_empty_enum_brackets_variant<'db>(
    db: &'db dyn SyntaxGroup,
    node: SyntaxNode<'db>,
) -> Option<InternalFix<'db>> {
    let ast_variant = ast::Variant::from_syntax_node(db, node);

    // Extract a clean type definition, to remove
    let type_clause = ast_variant
        .type_clause(db)
        .as_syntax_node()
        .get_text_without_trivia(db);

    let variant_text = node.get_text(db);
    let fixed = variant_text.replace(type_clause, "");

    Some(InternalFix {
        node,
        suggestion: fixed,
        description: EmptyEnumBracketsVariant.fix_message().unwrap().to_string(),
        import_addition_paths: None,
    })
}
