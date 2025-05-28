use crate::context::{CairoLintKind, Lint};
use crate::fixes::InternalFix;
use crate::helper::find_module_file_containing_node;
use crate::queries::{get_all_function_bodies, get_all_function_calls};
use cairo_lang_defs::ids::{
    FreeFunctionLongId, FunctionWithBodyId, ImplFunctionLongId, ModuleFileId, ModuleItemId,
    TraitFunctionLongId,
};
use cairo_lang_defs::plugin::PluginDiagnostic;
use cairo_lang_diagnostics::Severity;
use cairo_lang_semantic::db::SemanticGroup;
use cairo_lang_semantic::items::function_with_body::SemanticExprLookup;
use cairo_lang_semantic::types::peel_snapshots;
use cairo_lang_semantic::{Expr, ExprFunctionCall};
use cairo_lang_syntax::node::kind::SyntaxKind;
use cairo_lang_syntax::node::{ast, SyntaxNode, TypedStablePtr, TypedSyntaxNode};
use cairo_lang_utils::Intern;
use itertools::Itertools;

const T_COPY_CLONE: &str = "core::clone::TCopyClone";

pub struct CloneOnCopy;

/// ## What it does
///
/// Checks for usage of `.clone()` on a `Copy` type.
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
        "using `clone` on type which implements `Copy` trait"
    }

    fn kind(&self) -> CairoLintKind {
        CairoLintKind::CloneOnCopy
    }

    fn has_fixer(&self) -> bool {
        true
    }

    fn fix(&self, db: &dyn SemanticGroup, node: SyntaxNode) -> Option<InternalFix> {
        fix_clone_on_copy(db, node)
    }
}

pub fn check_clone_on_copy(
    db: &dyn SemanticGroup,
    item: &ModuleItemId,
    diagnostics: &mut Vec<PluginDiagnostic>,
) {
    let function_bodies = get_all_function_bodies(db, item);
    for function_body in function_bodies.iter() {
        let function_call_exprs = get_all_function_calls(function_body);
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
            relative_span: None,
            inner_span: None,
        });
    }
}

fn fix_clone_on_copy(db: &dyn SemanticGroup, node: SyntaxNode) -> Option<InternalFix> {
    let ast_expr_binary = ast::ExprBinary::cast(db.upcast(), node)?;

    let module_file_id = find_module_file_containing_node(db, &node)?;

    let ast_expr = ast_expr_binary.lhs(db.upcast());

    let expr_semantic = get_expr_semantic(db, module_file_id, &ast_expr_binary)
        .expect("Failed to find semantic expression.");

    // Extract the number of `@` snapshots from the type.
    // Each `@` will later be represented as a `*` prefix in the output.
    let (mut snapshot_count, _) = peel_snapshots(db, expr_semantic.ty());

    // `clone(self: @T)` expects an `@`, so the compiler will automatically insert
    // an `@` into the type if it was not explicitly provided by the user.
    // In such cases, the expression will be of type `Expr::Snapshot`,
    // meaning that `peel_snapshots` would count one coercion too many.
    //
    // However, if the `@` was explicitly written by the user,
    // the expression will be of another type, such as `Expr::Var`,
    // and `peel_snapshots` will have already counted the correct number of `@`.
    //
    // Therefore, we need to manually subtract one from the snapshot count
    // when the expression is a `Expr::Snapshot` to correct this.
    if let Expr::Snapshot(_) = expr_semantic {
        snapshot_count -= 1;
    };

    let fixed_expr = format!(
        "{}{}",
        "*".repeat(snapshot_count),
        ast_expr.as_syntax_node().get_text(db.upcast())
    );

    Some(InternalFix {
        node,
        suggestion: fixed_expr,
        import_addition_paths: None,
    })
}

fn get_expr_semantic(
    db: &dyn SemanticGroup,
    module_file_id: ModuleFileId,
    ast_expr_binary: &ast::ExprBinary,
) -> Option<Expr> {
    let ast_expr = ast_expr_binary.lhs(db.upcast());

    let expr_ptr = ast_expr.stable_ptr(db.upcast());

    // Traverses up the syntax tree to find the nearest enclosing function (trait, impl, or free) that owns the expression.
    // If found, retrieves the corresponding semantic expression.
    ast_expr
        .as_syntax_node()
        .ancestors_with_self(db.upcast())
        .find_map(|ancestor| {
            let function_id = get_function_with_body_id(db, module_file_id, ancestor)?;

            db.lookup_expr_by_ptr(function_id, expr_ptr)
                .or_else(|_| {
                    // If the expression is not found using the expr_ptr (the pointer from the left-hand side of the binary expression),
                    // it means the pointer should be created from the entire binary expression instead.
                    let expr_binary_ptr =
                        ast::ExprPtr(ast_expr_binary.stable_ptr(db.upcast()).untyped());
                    db.lookup_expr_by_ptr(function_id, expr_binary_ptr)
                })
                .ok()
                .map(|id| db.expr_semantic(function_id, id))
        })
}

fn get_function_with_body_id(
    db: &dyn SemanticGroup,
    module_file_id: ModuleFileId,
    ancestor: SyntaxNode,
) -> Option<FunctionWithBodyId> {
    if let Some(trait_func) = ast::TraitItemFunction::cast(db.upcast(), ancestor) {
        let ptr = trait_func.stable_ptr(db.upcast());
        Some(FunctionWithBodyId::Trait(
            TraitFunctionLongId(module_file_id, ptr).intern(db),
        ))
    } else if let Some(func_with_body) = ast::FunctionWithBody::cast(db.upcast(), ancestor) {
        let ptr = func_with_body.stable_ptr(db.upcast());

        let function_with_body_id = if ancestor
            .ancestor_of_kind(db.upcast(), SyntaxKind::ItemImpl)
            .is_some()
        {
            FunctionWithBodyId::Impl(ImplFunctionLongId(module_file_id, ptr).intern(db))
        } else {
            FunctionWithBodyId::Free(FreeFunctionLongId(module_file_id, ptr).intern(db))
        };

        Some(function_with_body_id)
    } else {
        None
    }
}
