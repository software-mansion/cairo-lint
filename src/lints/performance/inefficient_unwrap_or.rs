use cairo_lang_defs::{
    ids::{FunctionWithBodyId, ModuleItemId, NamedLanguageElementId},
    plugin::PluginDiagnostic,
};
use cairo_lang_diagnostics::Severity;
use cairo_lang_semantic::{
    Expr, ExprFunctionCall, ExprFunctionCallArg, ExprId, FunctionBody, corelib,
    items::{
        function_with_body::FunctionWithBodySemantic, functions::GenericFunctionId,
        imp::ImplSemantic,
    },
};
use cairo_lang_syntax::node::{SyntaxNode, TypedStablePtr, TypedSyntaxNode, ast};
use salsa::Database;

use crate::{
    LinterGroup,
    context::{CairoLintKind, Lint},
    fixer::InternalFix,
    queries::{get_all_function_bodies_with_ids, get_all_function_calls},
};

pub struct InefficientUnwrapOr;

/// ## What it does
///
/// Finds calls of `Option::unwrap_or` or `Result::unwrap_or`
/// which can be optimized by lazy-evaluation, using `unwrap_or_else`.
///
/// ## Example
///
/// ```cairo
/// fn foo() -> usize {
///     // Some heavy computation here
///     0
/// }
///
/// let x: Option<i32> = None;
/// let y = x.unwrap_or(foo());
/// ```
///
/// Can be optimized:
///
/// ```cairo
/// let y = x.unwrap_or_else(|| foo());
/// ```
impl Lint for InefficientUnwrapOr {
    fn allowed_name(&self) -> &'static str {
        "inefficient_unwrap_or"
    }

    fn diagnostic_message(&self) -> &'static str {
        "Inefficient `unwrap_or` detected. Consider using `unwrap_or_else()` instead."
    }

    fn kind(&self) -> CairoLintKind {
        CairoLintKind::InefficientUnwrapOr
    }

    fn has_fixer(&self) -> bool {
        true
    }

    fn fix<'db>(&self, db: &'db dyn Database, node: SyntaxNode<'db>) -> Option<InternalFix<'db>> {
        fix_inefficient_unwrap_or(db, node)
    }

    fn fix_message(&self) -> Option<&'static str> {
        Some("Use `unwrap_or_else()` instead of `unwrap_or()`")
    }
}

#[tracing::instrument(skip_all)]
pub fn check_inefficient_unwrap_or<'db>(
    db: &'db dyn Database,
    item: &ModuleItemId<'db>,
    diagnostics: &mut Vec<PluginDiagnostic<'db>>,
) {
    let function_bodies = get_all_function_bodies_with_ids(db, item);

    for (function_id, function_body) in function_bodies {
        for unwrap_or_call in get_all_unwrap_or_calls(db, function_body) {
            let Some(argument) = &unwrap_or_call.args.get(1) else {
                continue;
            };

            let ExprFunctionCallArg::Value(argument_expr_id) = argument else {
                continue;
            };

            if contains_nontrivial_expression(db, *argument_expr_id, function_id) {
                diagnostics.push(PluginDiagnostic {
                    stable_ptr: unwrap_or_call.stable_ptr.untyped(),
                    message: InefficientUnwrapOr.diagnostic_message().to_owned(),
                    severity: Severity::Warning,
                    inner_span: None,
                    error_code: None,
                });
            }
        }
    }
}

fn get_all_unwrap_or_calls<'db>(
    db: &'db dyn Database,
    function_body: &'db FunctionBody<'db>,
) -> impl Iterator<Item = ExprFunctionCall<'db>> {
    let option_trait = db.corelib_context().get_option_trait(db).long(db);
    let result_trait = db.corelib_context().get_result_trait(db).long(db);

    get_all_function_calls(function_body).filter(move |function_call| {
        let generic_function = function_call.function.get_concrete(db).generic_function;
        let GenericFunctionId::Impl(impl_function) = generic_function else {
            return false;
        };
        let Ok(concrete_trait) = db.impl_concrete_trait(impl_function.impl_id) else {
            return false;
        };

        let function_name = impl_function.function.name(db).long(db);
        let is_unwrap_or = function_name == "unwrap_or";

        let trait_id = concrete_trait.trait_id(db).long(db);
        let is_from_option_trait = trait_id == option_trait;
        let is_from_result_trait = trait_id == result_trait;

        is_unwrap_or && (is_from_option_trait || is_from_result_trait)
    })
}

fn contains_nontrivial_expression<'db>(
    db: &'db dyn Database,
    expr_id: ExprId,
    function_id: FunctionWithBodyId<'db>,
) -> bool {
    let argument_semantic = db.expr_semantic(function_id, expr_id);

    match argument_semantic {
        // Non-trivial expressions that should be evaluated lazily.
        Expr::FunctionCall(_)
        | Expr::Match(_)
        | Expr::If(_)
        | Expr::Block(_)
        | Expr::Loop(_)
        | Expr::While(_)
        | Expr::For(_)
        | Expr::FixedSizeArray(_) => true,

        // Non-trivial if the inner type is non-trivial.
        Expr::Tuple(expr_tuple) => expr_tuple
            .items
            .into_iter()
            .any(|item| contains_nontrivial_expression(db, item, function_id)),

        Expr::Snapshot(expr_snapshot) => {
            contains_nontrivial_expression(db, expr_snapshot.inner, function_id)
        }

        Expr::Desnap(expr_desnap) => {
            contains_nontrivial_expression(db, expr_desnap.inner, function_id)
        }

        Expr::StructCtor(expr_struct_ctor) => expr_struct_ctor
            .members
            .into_iter()
            .any(|(member_expr, _)| contains_nontrivial_expression(db, member_expr, function_id)),

        Expr::EnumVariantCtor(expr_enum_variant_ctor) => {
            contains_nontrivial_expression(db, expr_enum_variant_ctor.value_expr, function_id)
        }

        // Cairo compiler cannot const-fold expressions like `function_call() && false`.
        // We consider the logical operator trivial if both arguments are constant.
        Expr::LogicalOperator(expr_logical_operator) => {
            let lhs_semantic = db.expr_semantic(function_id, expr_logical_operator.lhs);
            let rhs_semantic = db.expr_semantic(function_id, expr_logical_operator.rhs);
            !is_bool_literal(db, lhs_semantic) || !is_bool_literal(db, rhs_semantic)
        }

        // Access via Deref can be arbitrarily complex, thus it's non-trivial
        Expr::MemberAccess(expr_member_access) => {
            // If member_path is None, the access is performed via Deref.
            expr_member_access.member_path.is_none()
        }

        // Trivial expressions that can be evaluated eagerly.
        Expr::Constant(_)
        | Expr::Var(_)
        | Expr::Literal(_)
        | Expr::StringLiteral(_)
        | Expr::ExprClosure(_)
        | Expr::PropagateError(_) => false,

        // Irrelevant.
        Expr::Assignment(_) | Expr::Missing(_) => false,
    }
}

fn is_bool_literal<'db>(db: &'db dyn Database, expr: Expr<'db>) -> bool {
    let Expr::EnumVariantCtor(enum_variant_constructor) = expr else {
        return false;
    };

    let enum_id = enum_variant_constructor.variant.concrete_enum_id.long(db);
    let bool_enum_id = corelib::core_bool_enum(db).long(db);

    enum_id == bool_enum_id
}

#[tracing::instrument(skip_all)]
fn fix_inefficient_unwrap_or<'db>(
    db: &'db dyn Database,
    node: SyntaxNode<'db>,
) -> Option<InternalFix<'db>> {
    let unwrap_or_call_on_object = ast::ExprBinary::from_syntax_node(db, node);
    let receiver_object = unwrap_or_call_on_object.lhs(db);
    let op = unwrap_or_call_on_object.op(db);
    let ast::Expr::FunctionCall(unwrap_or_call) = unwrap_or_call_on_object.rhs(db) else {
        return None;
    };

    let first_argument = unwrap_or_call
        .arguments(db)
        .arguments(db)
        .elements(db)
        .next()?;

    let suggestion = format!(
        "{}{}unwrap_or_else(|| {})",
        receiver_object.as_syntax_node().get_text(db),
        op.as_syntax_node().get_text(db),
        first_argument.as_syntax_node().get_text(db)
    );

    Some(InternalFix {
        node,
        suggestion,
        description: InefficientUnwrapOr.fix_message().unwrap().to_string(),
        import_addition_paths: None,
    })
}
