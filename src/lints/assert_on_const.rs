use cairo_lang_defs::{
    ids::{LookupItemId, ModuleItemId},
    plugin::PluginDiagnostic,
};
use cairo_lang_diagnostics::Severity;
use cairo_lang_semantic::{
    corelib::core_bool_enum, db::SemanticGroup, items::constant::ConstValue,
    resolve::ResolvedConcreteItem,
};
use cairo_lang_syntax::node::{
    Terminal, TypedSyntaxNode,
    ast::{PathSegment, TerminalIdentifier, TokenNode, TokenTree, WrappedTokenTree},
};
use if_chain::if_chain;

use crate::{
    context::{CairoLintKind, Lint},
    queries::get_all_inline_macro_calls,
};

pub struct AssertOnConst;

/// ## What it does
///
/// Checks for assertions on boolean literals and constants.
///
/// ## Example
///
/// ```cairo
/// fn main() {
///     const C: bool = true;
///     assert!(C);  // Always passes
///
///     assert!(true);  // Always passes
///     assert!(false);  // Never passes
/// }
/// ```
impl Lint for AssertOnConst {
    fn allowed_name(&self) -> &'static str {
        "assert_on_const"
    }

    fn diagnostic_message(&self) -> &'static str {
        "Unnecessary assert on a const value detected."
    }

    fn kind(&self) -> crate::context::CairoLintKind {
        CairoLintKind::RedundantOperation
    }
}

/// Checks for `assert!(true)` or `assert!(false)`.
#[tracing::instrument(skip_all, level = "trace")]
pub fn check_assert_on_const(
    db: &dyn SemanticGroup,
    item: &ModuleItemId,
    diagnostics: &mut Vec<PluginDiagnostic>,
) {
    let asserts = get_all_inline_macro_calls(db, item)
        .into_iter()
        .filter(|call| {
            let path_elements = call.path(db).segments(db).elements(db).collect::<Vec<_>>();
            match &path_elements[..] {
                [PathSegment::Simple(path_segment)] => path_segment.ident(db).text(db) == "assert",
                _ => false,
            }
        });

    for assert_call in asserts {
        let WrappedTokenTree::Parenthesized(subtree) = assert_call.arguments(db).subtree(db) else {
            continue;
        };

        // We only look for a single identifier or a boolean literal.
        let Some(TokenTree::Token(argument_token)) = subtree.tokens(db).elements(db).last() else {
            continue;
        };
        let argument_token = argument_token.leaf(db);
        let stable_ptr = argument_token.as_syntax_node().stable_ptr(db);

        match argument_token {
            TokenNode::TerminalTrue(_) | TokenNode::TerminalFalse(_) => {
                diagnostics.push(PluginDiagnostic {
                    stable_ptr,
                    message: AssertOnConst.diagnostic_message().to_string(),
                    severity: Severity::Warning,
                    inner_span: None,
                })
            }
            TokenNode::TerminalIdentifier(id) if is_boolean_const(db, &id, item) => diagnostics
                .push(PluginDiagnostic {
                    stable_ptr,
                    message: AssertOnConst.diagnostic_message().to_string(),
                    severity: Severity::Warning,
                    inner_span: None,
                }),
            _ => (),
        };
    }
}

/// Checks if the given identifier refers to a boolean const.
fn is_boolean_const(
    db: &dyn SemanticGroup,
    id: &TerminalIdentifier,
    module_item: &ModuleItemId,
) -> bool {
    let lookup_item = LookupItemId::ModuleItem(*module_item);
    let resolved_item = db.lookup_resolved_concrete_item_by_ptr(lookup_item, id.stable_ptr(db));

    if_chain!(
        if let Some(ResolvedConcreteItem::Constant(const_id)) = resolved_item;
        if let ConstValue::Enum(variant, _) = db.lookup_intern_const_value(const_id);
        if variant.concrete_enum_id == core_bool_enum(db);

        then {
            true
        } else {
            false
        }
    )
}
