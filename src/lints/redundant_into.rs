use cairo_lang_defs::ids::{ModuleItemId, TopLevelLanguageElementId};
use cairo_lang_defs::plugin::PluginDiagnostic;
use cairo_lang_diagnostics::Severity;
use cairo_lang_semantic::{
    Arenas, ExprFunctionCall, ExprFunctionCallArg, GenericArgumentId, TypeId, TypeLongId,
};
use cairo_lang_syntax::node::TypedStablePtr;
use salsa::Database;

use super::{OPTION, function_trait_name_from_fn_id};
use crate::context::{CairoLintKind, Lint};
use crate::lints::{INTO, TRY_INTO};
use crate::queries::{get_all_function_bodies, get_all_function_calls};

pub struct RedundantInto;

/// Detects redundant calls to `into()` or `try_into()` where the input and output
/// types are the same, i.e., the conversion is a no-op and can be removed.
///
/// Example
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
    // Removes cases with unresolved generic args
    let func = function_trait_name_from_fn_id(db, &expr_func.function);

    let target_ty: TypeId = match func.as_str() {
        INTO => expr_func.ty,
        TRY_INTO => {
            if let Some(ok_ty) = result_ok_type(db, expr_func.ty) {
                ok_ty
            } else {
                return;
            }
        }
        _ => return,
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
            inner_span: None,
        });
    }
}

/// Extracts T from `core::option::Option::<T, E>`
fn result_ok_type<'db>(db: &'db dyn Database, ty: TypeId<'db>) -> Option<TypeId<'db>> {
    if let TypeLongId::Concrete(conc) = ty.long(db) {
        let generic_ty = conc.generic_type(db);
        if generic_ty.full_path(db) == OPTION {
            let mut args = conc.generic_args(db).into_iter();
            if let Some(GenericArgumentId::Type(ok_ty)) = args.next() {
                return Some(ok_ty);
            }
        }
    }
    None
}
