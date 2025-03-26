use crate::context::{CairoLintKind, Lint};
use crate::helper;
use crate::queries::{get_all_function_bodies, get_all_real_function_calls};
use cairo_lang_defs::ids::{
    FunctionWithBodyId, ImplFunctionLongId, LanguageElementId, ModuleItemId,
    TopLevelLanguageElementId, TraitFunctionLongId,
};
use cairo_lang_defs::plugin::PluginDiagnostic;
use cairo_lang_diagnostics::Severity;
use cairo_lang_semantic::db::SemanticGroup;
use cairo_lang_semantic::items::function_with_body::SemanticExprLookup;
use cairo_lang_semantic::types::peel_snapshots;
use cairo_lang_semantic::ExprFunctionCall;
use cairo_lang_syntax::node::kind::SyntaxKind;
use cairo_lang_syntax::node::{ast, SyntaxNode, TypedStablePtr, TypedSyntaxNode};
use cairo_lang_utils::Intern;
use cairo_lang_utils::Upcast;
use itertools::Itertools;

const T_COPY_CLONE: &str = "core::clone::TCopyClone";

pub struct CloneOnCopy;

/// ## What it does
///
/// Checks for usage of .clone() on a Copy type.
///
/// ## Example
///
/// ```cairo
///     let a: felt252 = 'Hello';
///     let b = a.clone()
/// ```
impl Lint for CloneOnCopy {
    fn allowed_name(&self) -> &'static str {
        "clone_on_copy"
    }

    fn diagnostic_message(&self) -> &'static str {
        "using `clone` on type which implements Copy trait"
    }

    fn kind(&self) -> CairoLintKind {
        CairoLintKind::CloneOnCopy
    }

    fn has_fixer(&self) -> bool {
        true
    }

    fn fix(&self, db: &dyn SemanticGroup, node: SyntaxNode) -> Option<(SyntaxNode, String)> {
        fix_clone_on_copy(db, node)
    }
}

pub fn check_clone_on_copy(
    db: &dyn SemanticGroup,
    item: &ModuleItemId,
    diagnostics: &mut Vec<PluginDiagnostic>,
) {
    let Ok(id) = item.module_file_id(db.upcast()).file_id(db.upcast()) else {
        return;
    };

    // if let FileLongId::External(_) = id.lookup_intern(db) {
    //     return;
    // }

    let function_bodies = get_all_function_bodies(db, item);
    for function_body in function_bodies.iter() {
        let function_call_exprs = get_all_real_function_calls(db, function_body.upcast(), &id);
        for function_call_expr in function_call_exprs {
            check_clone_usage(db, &function_call_expr, diagnostics);
        }
    }
}

fn check_clone_usage(
    db: &dyn SemanticGroup,
    expr: &ExprFunctionCall,
    diagnostics: &mut Vec<PluginDiagnostic>,
) {
    let function_name = expr.function.full_path(db).split("::").take(3).join("::");

    if function_name == T_COPY_CLONE {
        diagnostics.push(PluginDiagnostic {
            stable_ptr: expr.stable_ptr.untyped(),
            message: CloneOnCopy.diagnostic_message().to_string(),
            severity: Severity::Warning,
        });
    }
}

fn fix_clone_on_copy(db: &dyn SemanticGroup, node: SyntaxNode) -> Option<(SyntaxNode, String)> {
    let expr = match node.kind(db.upcast()) {
        SyntaxKind::ExprBinary => ast::ExprBinary::from_syntax_node(db.upcast(), node.clone()),
        _ => {
            return None;
        }
    };
    let module_file_id = helper::find_module_file_containing_node(db, &node)?;

    let ty = node.ancestors_with_self().find_map(|node| {
        // node.ancestors_with_self().for_each(|n| println!("{}\n", n.kind(db.upcast())));
        if let Some(trait_item_func) = ast::TraitItemFunction::cast(db.upcast(), node.clone()) {
            let trait_fn_id = FunctionWithBodyId::Trait(
                TraitFunctionLongId(module_file_id, trait_item_func.stable_ptr()).intern(db),
            );

            return db
                .lookup_expr_by_ptr(trait_fn_id, expr.lhs(db.upcast()).stable_ptr())
                .ok()
                .map(|r| db.expr_semantic(trait_fn_id, r).ty());
        }

        if let Some(func_with_body) = ast::FunctionWithBody::cast(db.upcast(), node.clone()) {
            if let Some(_) = node.ancestor_of_kind(db.upcast(), SyntaxKind::ItemImpl) {
                let func_id = FunctionWithBodyId::Impl(
                    ImplFunctionLongId(module_file_id, func_with_body.stable_ptr()).intern(db),
                );

                return db
                    .lookup_expr_by_ptr(func_id, expr.lhs(db.upcast()).stable_ptr())
                    .ok()
                    .map(|r| db.expr_semantic(func_id, r).ty());
            }
            // if let Some(_) = node.ancestor_of_kind(db.upcast(), SyntaxKind::ItemImpl) {
            //     let func_id = FunctionWithBodyId::Impl(
            //         ImplFunctionLongId(module_file_id, func_with_body.stable_ptr()).intern(db),
            //     );
            // } else {
            //     FunctionWithBodyId::Free(
            //         FreeFunctionLongId(module_file_id, func_with_body.stable_ptr()).intern(db),
            //     )
            // };
        }

        None
    });

    match ty {
        Some(valid_ty) => {
            let (n, _) = peel_snapshots(db, valid_ty);

            // println!("{n}");
            let fixed_expr = format!(
                "{}{}",
                "*".repeat(n),
                expr.lhs(db.upcast()).as_syntax_node().get_text(db.upcast())
            );
            Some((node, fixed_expr))
        }
        None => Some((
            node,
            expr.lhs(db.upcast()).as_syntax_node().get_text(db.upcast()),
        )),
    }
}
