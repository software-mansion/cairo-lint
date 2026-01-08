use cairo_lang_defs::ids::{GenericTypeId, ModuleItemId};
use cairo_lang_defs::plugin::PluginDiagnostic;
use cairo_lang_diagnostics::Severity;
use cairo_lang_semantic::items::functions::GenericFunctionId;
use cairo_lang_semantic::{
    Arenas, ExprFunctionCall, ExprFunctionCallArg, GenericArgumentId, TypeId, TypeLongId,
};
use cairo_lang_syntax::node::TypedStablePtr;
use salsa::Database;

use crate::LinterGroup;
use crate::context::{CairoLintKind, Lint};
use crate::queries::{get_all_function_bodies, get_all_function_calls};

pub struct RedundantInto;

/// ## What it does
///
/// Detects redundant calls to `into()` or `try_into()` where the input and output
/// types are the same, i.e., the conversion is a no-op and can be removed.
///
/// ## Example
///
/// ```cairo
/// fn f(x: u128) -> u128 {
///     // redundant - `x` is already an u128
///     x.into()
/// }
/// ```
impl Lint for RedundantInto {
    fn allowed_name(&self) -> &'static str {
        "redundant_into"
    }

    fn diagnostic_message(&self) -> &'static str {
        "Redundant conversion: input and output types are the same."
    }

    fn kind(&self) -> CairoLintKind {
        CairoLintKind::RedundantInto
    }
}

#[tracing::instrument(skip_all, level = "trace")]
pub fn check_redundant_into<'db>(
    db: &'db dyn Database,
    item: &ModuleItemId<'db>,
    diagnostics: &mut Vec<PluginDiagnostic<'db>>,
) {
    let function_bodies = get_all_function_bodies(db, item);
    for function_body in function_bodies.iter() {
        let arenas = &function_body.arenas;
        let function_call_exprs = get_all_function_calls(function_body);
        for function_call_expr in function_call_exprs {
            check_single_redundant_into(db, &function_call_expr, arenas, diagnostics);
        }
    }
}

fn check_single_redundant_into<'db>(
    db: &'db dyn Database,
    expr_func: &ExprFunctionCall<'db>,
    arenas: &Arenas<'db>,
    diagnostics: &mut Vec<PluginDiagnostic<'db>>,
) {
    let corelib_context = db.corelib_context();
    let into_fn_id = corelib_context.get_into_trait_function_id();
    let try_into_fn_id = corelib_context.get_try_into_trait_function_id();

    let GenericFunctionId::Impl(impl_generic_func_id) =
        expr_func.function.get_concrete(db).generic_function
    else {
        return;
    };

    let function = impl_generic_func_id.function;

    let target_ty: TypeId = if function == into_fn_id {
        expr_func.ty
    } else if function == try_into_fn_id {
        if let Some(ok_ty) = result_ok_type(db, expr_func.ty) {
            ok_ty
        } else {
            return;
        }
    } else {
        return;
    };

    let Some(first_arg) = expr_func.args.first() else {
        return;
    };

    let input_ty: TypeId = if let ExprFunctionCallArg::Value(expr_id) = first_arg {
        arenas.exprs[*expr_id].ty()
    } else {
        // Not possible, since the Into/TryInto function doesn't take mutable args
        return;
    };

    if input_ty == target_ty {
        diagnostics.push(PluginDiagnostic {
            stable_ptr: expr_func.stable_ptr.untyped(),
            message: RedundantInto.diagnostic_message().to_string(),
            severity: Severity::Warning,
            error_code: None,
            inner_span: None,
        });
    }
}

/// Extracts T from `core::option::Option::<T, E>`
fn result_ok_type<'db>(db: &'db dyn Database, ty: TypeId<'db>) -> Option<TypeId<'db>> {
    if let TypeLongId::Concrete(conc) = ty.long(db) {
        let generic_ty = conc.generic_type(db);
        let corelib_context = db.corelib_context();
        let option_enum_id = corelib_context.get_option_enum_id();
        let type_id = if let GenericTypeId::Enum(enum_id) = generic_ty {
            enum_id
        } else {
            return None;
        };

        if type_id == option_enum_id {
            let mut args = conc.generic_args(db).into_iter();
            if let Some(GenericArgumentId::Type(ok_ty)) = args.next() {
                return Some(ok_ty);
            }
        }
    }
    None
}
