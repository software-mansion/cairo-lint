use crate::{
    context::{CairoLintKind, Lint},
    fixer::InternalFix,
    queries::{get_all_function_bodies, get_all_function_calls},
};
use cairo_lang_defs::ids::{NamedLanguageElementId, TopLevelLanguageElementId};
use cairo_lang_defs::{ids::ModuleItemId, plugin::PluginDiagnostic};
use cairo_lang_diagnostics::Severity;
use cairo_lang_semantic::items::functions::{GenericFunctionId, ImplGenericFunctionId};
use cairo_lang_semantic::items::imp::ImplHead;
use cairo_lang_semantic::{
    Arenas, ExprFunctionCall, ExprFunctionCallArg, GenericArgumentId, TypeId, TypeLongId,
};
use cairo_lang_syntax::node::{SyntaxNode, TypedStablePtr, TypedSyntaxNode, ast};
use salsa::Database;

pub struct UnwrapSyscall;

const UNWRAP_SYSCALL_TRAIT_PATH: &str = "starknet::SyscallResultTrait";

/// ## What it does
///
/// Detects if the function uses `unwrap` on a `SyscallResult` object.
///
/// ## Example
///
/// ```cairo
/// use starknet::storage_access::{storage_address_from_base, storage_base_address_from_felt252};
/// use starknet::syscalls::storage_read_syscall;
///
/// fn main() {
///     let storage_address = storage_base_address_from_felt252(3534535754756246375475423547453);
///     let result = storage_read_syscall(0, storage_address_from_base(storage_address));
///     result.unwrap();
/// }
/// ```
///
/// Can be changed to:
///
/// ```cairo
/// use starknet::SyscallResultTrait;
/// use starknet::storage_access::{storage_address_from_base, storage_base_address_from_felt252};
/// use starknet::syscalls::storage_read_syscall;
///
/// fn main() {
///     let storage_address = storage_base_address_from_felt252(3534535754756246375475423547453);
///     let result = storage_read_syscall(0, storage_address_from_base(storage_address));
///     result.unwrap_syscall();
/// }
/// ```
impl Lint for UnwrapSyscall {
    fn allowed_name(&self) -> &'static str {
        "unwrap_syscall"
    }

    fn diagnostic_message(&self) -> &'static str {
        "consider using `unwrap_syscall` instead of `unwrap`"
    }

    fn kind(&self) -> CairoLintKind {
        CairoLintKind::UnwrapSyscall
    }

    fn has_fixer(&self) -> bool {
        true
    }

    fn fix<'db>(&self, db: &'db dyn Database, node: SyntaxNode<'db>) -> Option<InternalFix<'db>> {
        fix_unwrap_syscall(db, node)
    }

    fn fix_message(&self) -> Option<&'static str> {
        Some("Replace with `unwrap_syscall()` for syscall results")
    }
}

#[tracing::instrument(skip_all, level = "trace")]
pub fn check_unwrap_syscall<'db>(
    db: &'db dyn Database,
    item: &ModuleItemId<'db>,
    diagnostics: &mut Vec<PluginDiagnostic<'db>>,
) {
    let function_bodies = get_all_function_bodies(db, item);
    for function_body in function_bodies {
        let function_call_exprs = get_all_function_calls(function_body);
        let arenas = &function_body.arenas;
        for function_call_expr in function_call_exprs {
            check_single_unwrap_syscall(db, &function_call_expr, arenas, diagnostics);
        }
    }
}

fn check_single_unwrap_syscall<'db>(
    db: &'db dyn Database,
    expr: &ExprFunctionCall<'db>,
    arenas: &Arenas<'db>,
    diagnostics: &mut Vec<PluginDiagnostic<'db>>,
) {
    if is_result_trait_impl_unwrap_call(db, expr)
        && let Some(ExprFunctionCallArg::Value(expr_id)) = expr.args.first()
        && let receiver_expr = &arenas.exprs[*expr_id]
        && is_syscall_result_type(db, receiver_expr.ty())
    {
        diagnostics.push(PluginDiagnostic {
            stable_ptr: receiver_expr
                .stable_ptr()
                .lookup(db)
                .as_syntax_node()
                .parent(db)
                .unwrap()
                .stable_ptr(db),
            message: UnwrapSyscall.diagnostic_message().to_string(),
            severity: Severity::Warning,
            inner_span: None,
        });
    }
}

/// Check if this function call expression calls `core::result::ResultTraitImpl<_>::unwrap`.
fn is_result_trait_impl_unwrap_call(db: &dyn Database, expr: &ExprFunctionCall) -> bool {
    if let GenericFunctionId::Impl(ImplGenericFunctionId { impl_id, function }) =
        expr.function.get_concrete(db).generic_function
        && function.name(db).long(db).as_str() == "unwrap"
        && let Some(ImplHead::Concrete(impl_def_id)) = impl_id.head(db)
        && impl_def_id.full_path(db) == "core::result::ResultTraitImpl"
    {
        true
    } else {
        false
    }
}

// Checks if the type is `Result<T, Array<felt252>>`.
fn is_syscall_result_type(db: &dyn Database, ty: TypeId) -> bool {
    is_specific_concrete_generic_type(db, ty, "core::result::Result", |[_, arg_e]| {
        if let GenericArgumentId::Type(arg_e) = arg_e
            && is_specific_concrete_generic_type(db, arg_e, "core::array::Array", |[arg]| {
                if let GenericArgumentId::Type(arg) = arg
                    && is_specific_concrete_type(db, arg, "core::felt252")
                {
                    true
                } else {
                    false
                }
            })
        {
            true
        } else {
            false
        }
    })
}

fn is_specific_concrete_type(db: &dyn Database, ty: TypeId, full_path: &str) -> bool {
    if let TypeLongId::Concrete(concrete_type_long_id) = ty.long(db)
        && concrete_type_long_id.generic_type(db).full_path(db) == full_path
    {
        true
    } else {
        false
    }
}

fn is_specific_concrete_generic_type<'db, const N: usize>(
    db: &'db dyn Database,
    ty: TypeId<'db>,
    full_path: &str,
    generic_args_matcher: impl FnOnce([GenericArgumentId<'db>; N]) -> bool,
) -> bool {
    if let TypeLongId::Concrete(concrete_type_long_id) = ty.long(db)
        && concrete_type_long_id.generic_type(db).full_path(db) == full_path
        && let Ok(generic_args) =
            <[GenericArgumentId; N]>::try_from(concrete_type_long_id.generic_args(db))
        && generic_args_matcher(generic_args)
    {
        true
    } else {
        false
    }
}

#[tracing::instrument(skip_all, level = "trace")]
fn fix_unwrap_syscall<'db>(
    db: &'db dyn Database,
    node: SyntaxNode<'db>,
) -> Option<InternalFix<'db>> {
    let ast_expr_binary = ast::ExprBinary::cast(db, node).unwrap_or_else(|| {
        panic!(
          "Expected a binary expression for unwrap called on SyscallResult. Actual node text: {:?}",
          node.get_text(db)
        )
    });

    let fixed = format!(
        "{}{}unwrap_syscall()",
        ast_expr_binary.lhs(db).as_syntax_node().get_text(db),
        ast_expr_binary.op(db).as_syntax_node().get_text(db)
    );
    Some(InternalFix {
        node,
        suggestion: fixed,
        description: UnwrapSyscall.fix_message().unwrap().to_string(),
        import_addition_paths: Some(vec![UNWRAP_SYSCALL_TRAIT_PATH.to_string()]),
    })
}
