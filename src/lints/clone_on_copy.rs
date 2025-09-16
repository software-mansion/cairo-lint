use crate::context::{CairoLintKind, Lint};

use crate::LinterGroup;
use crate::fixer::InternalFix;
use crate::helper::find_module_file_containing_node;
use crate::queries::{get_all_function_bodies, get_all_function_calls};
use cairo_lang_defs::ids::{
    FreeFunctionLongId, FunctionWithBodyId, ImplFunctionLongId, ModuleFileId, ModuleItemId,
    TraitFunctionLongId,
};
use cairo_lang_defs::plugin::PluginDiagnostic;
use cairo_lang_diagnostics::Severity;
use cairo_lang_semantic::items::function_with_body::{
    FunctionWithBodySemantic, SemanticExprLookup,
};
use cairo_lang_semantic::items::functions::{GenericFunctionId, ImplGenericFunctionId};
use cairo_lang_semantic::items::imp::ImplHead;
use cairo_lang_semantic::types::peel_snapshots;
use cairo_lang_semantic::{Expr, ExprFunctionCall};
use cairo_lang_syntax::node::kind::SyntaxKind;
use cairo_lang_syntax::node::{SyntaxNode, TypedStablePtr, TypedSyntaxNode, ast};
use cairo_lang_utils::Intern;
use salsa::Database;

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

    fn fix<'db>(&self, db: &'db dyn Database, node: SyntaxNode<'db>) -> Option<InternalFix<'db>> {
        fix_clone_on_copy(db, node)
    }

    fn fix_message(&self) -> Option<&'static str> {
        Some("Remove redundant `.clone()`")
    }
}

#[tracing::instrument(skip_all, level = "trace")]
pub fn check_clone_on_copy<'db>(
    db: &'db dyn Database,
    item: &ModuleItemId<'db>,
    diagnostics: &mut Vec<PluginDiagnostic<'db>>,
) {
    let function_bodies = get_all_function_bodies(db, item);
    for function_body in function_bodies.iter() {
        let function_call_exprs = get_all_function_calls(function_body);
        for function_call_expr in function_call_exprs {
            check_clone_usage(db, &function_call_expr, diagnostics);
        }
    }
}

fn check_clone_usage<'db>(
    db: &'db dyn Database,
    function_call_expr: &ExprFunctionCall<'db>,
    diagnostics: &mut Vec<PluginDiagnostic<'db>>,
) {
    if let GenericFunctionId::Impl(ImplGenericFunctionId { impl_id, .. }) = function_call_expr
        .function
        .get_concrete(db)
        .generic_function
        && let Some(ImplHead::Concrete(impl_def_id)) = impl_id.head(db)
        && impl_def_id == db.corelib_context().get_t_copy_clone_impl_id()
    {
        diagnostics.push(PluginDiagnostic {
            stable_ptr: function_call_expr.stable_ptr.untyped(),
            message: CloneOnCopy.diagnostic_message().to_string(),
            severity: Severity::Warning,
            inner_span: None,
        });
    }
}

#[tracing::instrument(skip_all, level = "trace")]
fn fix_clone_on_copy<'db>(
    db: &'db dyn Database,
    node: SyntaxNode<'db>,
) -> Option<InternalFix<'db>> {
    let ast_expr_binary = ast::ExprBinary::cast(db, node)?;

    let module_file_id = find_module_file_containing_node(db, node)?;

    let ast_expr = ast_expr_binary.lhs(db);

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
        ast_expr.as_syntax_node().get_text(db)
    );

    Some(InternalFix {
        node,
        suggestion: fixed_expr,
        description: CloneOnCopy.fix_message().unwrap().to_string(),
        import_addition_paths: None,
    })
}

fn get_expr_semantic<'db>(
    db: &'db dyn Database,
    module_file_id: ModuleFileId<'db>,
    ast_expr_binary: &ast::ExprBinary<'db>,
) -> Option<Expr<'db>> {
    let ast_expr = ast_expr_binary.lhs(db);

    let expr_ptr = ast_expr.stable_ptr(db);

    // Traverses up the syntax tree to find the nearest enclosing function (trait, impl, or free) that owns the expression.
    // If found, retrieves the corresponding semantic expression.
    ast_expr
        .as_syntax_node()
        .ancestors_with_self(db)
        .find_map(|ancestor| {
            let function_id = get_function_with_body_id(db, module_file_id, ancestor)?;

            db.lookup_expr_by_ptr(function_id, expr_ptr)
                .or_else(|_| {
                    // If the expression is not found using the expr_ptr (the pointer from the left-hand side of the binary expression),
                    // it means the pointer should be created from the entire binary expression instead.
                    let expr_binary_ptr = ast::ExprPtr(ast_expr_binary.stable_ptr(db).untyped());
                    db.lookup_expr_by_ptr(function_id, expr_binary_ptr)
                })
                .ok()
                .map(|id| db.expr_semantic(function_id, id))
        })
}

fn get_function_with_body_id<'db>(
    db: &'db dyn Database,
    module_file_id: ModuleFileId<'db>,
    ancestor: SyntaxNode<'db>,
) -> Option<FunctionWithBodyId<'db>> {
    if let Some(trait_func) = ast::TraitItemFunction::cast(db, ancestor) {
        let ptr = trait_func.stable_ptr(db);
        Some(FunctionWithBodyId::Trait(
            TraitFunctionLongId(module_file_id, ptr).intern(db),
        ))
    } else if let Some(func_with_body) = ast::FunctionWithBody::cast(db, ancestor) {
        let ptr = func_with_body.stable_ptr(db);

        let function_with_body_id = if ancestor
            .ancestor_of_kind(db, SyntaxKind::ItemImpl)
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
