use std::collections::{HashSet, VecDeque};

use cairo_lang_defs::{
    ids::{FunctionWithBodyId, LookupItemId, ModuleId, ModuleItemId},
    plugin::PluginDiagnostic,
};
use cairo_lang_diagnostics::Severity;
use cairo_lang_filesystem::{
    db::{get_parent_and_mapping, translate_location},
    ids::{FileId, FileLongId},
};
use cairo_lang_parser::macro_helpers::AsLegacyInlineMacro;
use cairo_lang_semantic::{
    ConcreteTypeId, Expr, TypeLongId,
    corelib::core_bool_enum,
    db::SemanticGroup,
    items::{constant::ConstValue, function_with_body::SemanticExprLookup},
    resolve::ResolvedConcreteItem,
};
use cairo_lang_syntax::node::{
    SyntaxNode, Terminal, TypedSyntaxNode,
    ast::{
        Arg, ArgClause, ExprInlineMacro, PathSegment, TerminalIdentifier, TokenNode, TokenTree,
        WrappedArgList, WrappedTokenTree,
    },
    kind::SyntaxKind,
};
use cairo_lang_utils::{LookupIntern, ordered_hash_set::OrderedHashSet};
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
        let tokens = match assert_call.arguments(db).subtree(db) {
            WrappedTokenTree::Braced(subtree) => subtree.tokens(db),
            WrappedTokenTree::Bracketed(subtree) => subtree.tokens(db),
            WrappedTokenTree::Parenthesized(subtree) => subtree.tokens(db),
            WrappedTokenTree::Missing(_) => return,
        };

        // Envisioned solution, DOESN'T WORK!
        if is_const_expr(db, &assert_call, item) {
            diagnostics.push(PluginDiagnostic {
                stable_ptr: tokens.as_syntax_node().stable_ptr(db),
                message: AssertOnConst.diagnostic_message().to_string(),
                severity: Severity::Warning,
                inner_span: None,
            })
        }

        // Alternative solution:

        // We only look for a single identifier or a boolean literal.
        let Some(TokenTree::Token(argument_token)) = tokens.elements(db).last() else {
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

fn is_const_expr(
    db: &dyn SemanticGroup,
    macro_call: &ExprInlineMacro,
    module_item: &ModuleItemId,
) -> bool {
    let argument_resultants = if_chain!(
        if let Some(legacy_call) = macro_call.as_legacy_inline_macro(db);

        let arguments = match legacy_call.arguments(db) {
            WrappedArgList::BracketedArgList(args) => args.arguments(db),
            WrappedArgList::ParenthesizedArgList(args) => args.arguments(db),
            WrappedArgList::BracedArgList(args) => args.arguments(db),
            WrappedArgList::Missing(_) => return false,
        };

        // First argument is the expression we assert on.
        if let Some(argument) = &arguments.elements(db).next();
        if let Some(argument_resultants) = get_node_resultants(db, argument.as_syntax_node());

        then {
            argument_resultants
        } else {
            return false;
        }
    );

    let ModuleItemId::FreeFunction(free_function_id) = *module_item else {
        return false;
    };

    let function_with_body_id = FunctionWithBodyId::Free(free_function_id);

    let maybe_semantic_expr = argument_resultants
        .into_iter()
        .filter_map(|node| {
            let ArgClause::Unnamed(arg) = node.cast::<Arg>(db)?.arg_clause(db) else {
                return None;
            };

            let semantic_expr_id = db
                .lookup_expr_by_ptr(function_with_body_id, arg.value(db).stable_ptr(db))
                .ok()?;

            let semantic_expr = db.expr_semantic(function_with_body_id, semantic_expr_id);

            Some(semantic_expr)
        })
        .next();

    let Some(semantic_expr) = maybe_semantic_expr else {
        return false;
    };

    let ty = match semantic_expr {
        Expr::Literal(expr_literal) => expr_literal.ty,
        Expr::Constant(expr_constant) => expr_constant.ty,
        _ => return false,
    };

    let bool_long_id = TypeLongId::Concrete(ConcreteTypeId::Enum(core_bool_enum(db)));
    let bool_id = db.intern_type(bool_long_id);

    ty == bool_id
}

/// Checks if the given identifier refers to a boolean const.
fn is_boolean_const(
    db: &dyn SemanticGroup,
    id: &TerminalIdentifier,
    module_item: &ModuleItemId,
) -> bool {
    let lookup_item = LookupItemId::ModuleItem(*module_item);

    let resultants = get_node_resultants(db, id.as_syntax_node()).unwrap_or_default();
    let resolved_item = resultants
        .into_iter()
        .filter_map(|node| {
            let id = node.cast::<TerminalIdentifier>(db)?;
            db.lookup_resolved_concrete_item_by_ptr(lookup_item, id.stable_ptr(db))
        })
        .next();

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

// Temporarily copied from LS. To be deleted before merging, as soon as the new query group appears.

fn get_node_resultants(db: &dyn SemanticGroup, node: SyntaxNode) -> Option<Vec<SyntaxNode>> {
    let main_file = node.stable_ptr(db).file_id(db);

    let (mut files, _) = file_and_subfiles_with_corresponding_modules(db, main_file)?;

    files.remove(&main_file);

    let files: Vec<_> = files.into_iter().collect();
    let resultants = find_generated_nodes(db, &files, node);

    Some(resultants.into_iter().collect())
}

fn file_and_subfiles_with_corresponding_modules(
    db: &dyn SemanticGroup,
    file: FileId,
) -> Option<(HashSet<FileId>, HashSet<ModuleId>)> {
    let mut modules: HashSet<_> = db.file_modules(file).ok()?.iter().copied().collect();
    let mut files = HashSet::from([file]);
    let mut modules_queue: VecDeque<_> = modules.iter().copied().collect();
    while let Some(module_id) = modules_queue.pop_front() {
        for file_id in db.module_files(module_id).ok()?.iter() {
            if files.insert(*file_id) {
                for module_id in db.file_modules(*file_id).ok()?.iter() {
                    if modules.insert(*module_id) {
                        modules_queue.push_back(*module_id);
                    }
                }
            }
        }
    }
    Some((files, modules))
}

fn find_generated_nodes(
    db: &dyn SemanticGroup,
    node_descendant_files: &[FileId],
    node: SyntaxNode,
) -> OrderedHashSet<SyntaxNode> {
    let start_file = node.stable_ptr(db).file_id(db);

    let mut result = OrderedHashSet::default();

    let mut is_replaced = false;

    for &file in node_descendant_files {
        let Some((parent, mappings)) = get_parent_and_mapping(db, file) else {
            continue;
        };

        if parent != start_file {
            continue;
        }

        let Ok(file_syntax) = db.file_syntax(file) else {
            continue;
        };

        let is_replacing_og_item = match file.lookup_intern(db) {
            FileLongId::Virtual(vfs) => vfs.original_item_removed,
            FileLongId::External(id) => db.ext_as_virtual(id).original_item_removed,
            _ => unreachable!(),
        };

        let mut new_nodes: OrderedHashSet<_> = Default::default();

        for token in file_syntax.tokens(db) {
            // Skip end of the file terminal, which is also a syntax tree leaf.
            // As `ModuleItemList` and `TerminalEndOfFile` have the same parent,
            // which is the `SyntaxFile`, so we don't want to take the `SyntaxFile`
            // as an additional resultant.
            if token.kind(db) == SyntaxKind::TerminalEndOfFile {
                continue;
            }
            let nodes: Vec<_> = token
                .ancestors_with_self(db)
                .map_while(|new_node| {
                    translate_location(&mappings, new_node.span(db))
                        .map(|span_in_parent| (new_node, span_in_parent))
                })
                .take_while(|(_, span_in_parent)| node.span(db).contains(*span_in_parent))
                .collect();

            if let Some((last_node, _)) = nodes.last().cloned() {
                let (new_node, _) = nodes
                    .into_iter()
                    .rev()
                    .take_while(|(node, _)| node.span(db) == last_node.span(db))
                    .last()
                    .unwrap();

                new_nodes.insert(new_node);
            }
        }

        // If there is no node found, don't mark it as potentially replaced.
        if !new_nodes.is_empty() {
            is_replaced = is_replaced || is_replacing_og_item;
        }

        for new_node in new_nodes {
            result.extend(find_generated_nodes(db, node_descendant_files, new_node));
        }
    }

    if !is_replaced {
        result.insert(node);
    }

    result
}
