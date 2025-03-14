use crate::context::{CairoLintKind, Lint};
use cairo_lang_defs::ids::{LanguageElementId, ModuleItemId};
use cairo_lang_defs::plugin::PluginDiagnostic;
use cairo_lang_diagnostics::Severity;
use cairo_lang_semantic::db::SemanticGroup;
use cairo_lang_syntax::node::db::SyntaxGroup;
use cairo_lang_syntax::node::{
    ast::ItemEnum as AstEnumItem, SyntaxNode, Terminal, TypedSyntaxNode,
};

pub struct EnumVariantNames;

/// ## What it does
///
/// Detects enumeration variants that are prefixed or suffixed by the same characters.
///
/// ## Example
///
/// ```cairo
/// enum Cake {
///     BlackForestCake,
///     HummingbirdCake,
///     BattenbergCake,
/// }
/// ```
///
/// Can be simplified to:
///
/// ```cairo
/// enum Cake {
///     BlackForest,
///     Hummingbird,
///     Battenberg,
/// }
/// ```
impl Lint for EnumVariantNames {
    fn allowed_name(&self) -> &'static str {
        "enum_variant_names"
    }

    fn diagnostic_message(&self) -> &'static str {
        "All enum variants are prefixed or suffixed by the same characters."
    }

    fn kind(&self) -> CairoLintKind {
        CairoLintKind::EnumVariantNames
    }

    fn has_fixer(&self) -> bool {
        true
    }

    fn fix(&self, db: &dyn SyntaxGroup, node: SyntaxNode) -> Option<(SyntaxNode, String)> {
        fix_enum_variant_names(db, node)
    }
}

pub fn check_enum_variant_names(
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
    let variant_names: Vec<String> = variants.iter().map(|v| v.0.to_string()).collect();

    let (prefix, suffix) = get_prefix_and_suffix(&variant_names);

    if !prefix.is_empty() || !suffix.is_empty() {
        diagnostics.push(PluginDiagnostic {
            stable_ptr: enum_id.untyped_stable_ptr(db.upcast()),
            message: EnumVariantNames.diagnostic_message().to_string(),
            severity: Severity::Warning,
        });
    }
}

fn fix_enum_variant_names(db: &dyn SyntaxGroup, node: SyntaxNode) -> Option<(SyntaxNode, String)> {
    let enum_item = AstEnumItem::from_syntax_node(db, node.clone());

    let source = enum_item.as_syntax_node().get_text(db);
    let variants = enum_item.variants(db).elements(db);

    let variant_names: Vec<String> = variants
        .iter()
        .map(|v| v.name(db).text(db).to_string())
        .collect();

    let (prefixes, suffixes) = get_prefix_and_suffix(&variant_names);

    let mut fixed_enum = source.clone();

    for variant in &variant_names {
        let mut fixed_name = variant.clone();

        for prefix in &prefixes {
            if let Some(stripped) = fixed_name.strip_prefix(prefix) {
                fixed_name = stripped.to_string();
            }
        }

        for suffix in &suffixes {
            if let Some(stripped) = fixed_name.strip_suffix(suffix) {
                fixed_name = stripped.to_string();
            }
        }

        fixed_enum = fixed_enum.replace(variant, &fixed_name);
    }

    Some((node, fixed_enum))
}

fn get_prefix_and_suffix(variant_names: &[String]) -> (Vec<String>, Vec<String>) {
    let Some(first) = variant_names.first() else {
        return (vec![], vec![]);
    };

    if variant_names.len() == 1 {
        return (vec![], vec![]);
    }

    let mut prefix = word_split(first);
    let mut suffix = prefix.clone();
    suffix.reverse();

    for variant_name in variant_names.iter().skip(1) {
        let variant_split = word_split(variant_name);

        if variant_split.len() == 1 {
            return (vec![], vec![]);
        }

        prefix = prefix
            .iter()
            .zip(&variant_split)
            .take_while(|(a, b)| a == b)
            .map(|(a, _)| a.clone())
            .collect();

        suffix = suffix
            .iter()
            .zip(variant_split.iter().rev())
            .take_while(|(a, b)| a == b)
            .map(|(a, _)| a.clone())
            .collect();
    }

    (prefix, suffix)
}

fn word_split(name: &str) -> Vec<String> {
    let mut parts = Vec::new();
    let mut start = 0;

    let chars: Vec<char> = name.chars().collect();

    for i in 1..chars.len() {
        let prev = chars[i - 1];
        let curr = chars[i];

        if curr.is_uppercase() && prev.is_lowercase() {
            parts.push(name[start..i].to_string());
            start = i;
        } else if curr == '_' {
            parts.push(name[start..i].to_string());
            start = i + 1;
        }
    }

    if start < name.len() {
        parts.push(name[start..].to_string());
    }

    parts
}
