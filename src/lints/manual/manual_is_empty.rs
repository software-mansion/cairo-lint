use crate::context::{CairoLintKind, Lint};

use crate::fixer::InternalFix;
use crate::lints::{ARRAY, SPAN, U32, function_trait_name_from_fn_id};
use crate::mappings::get_originating_syntax_node_for;
use crate::queries::{
    get_all_conditions, get_all_function_bodies, syntax_node_to_str_without_all_nested_trivia,
};
use cairo_lang_defs::ids::ModuleItemId;
use cairo_lang_defs::plugin::PluginDiagnostic;
use cairo_lang_diagnostics::Severity;
use cairo_lang_semantic::{
    Arenas, Condition, Expr, ExprFunctionCall, ExprFunctionCallArg, TypeLongId,
};
use cairo_lang_syntax::node::ast::Expr as SyntaxExpr;
use cairo_lang_syntax::node::ast::ExprBinary;

use cairo_lang_syntax::node::kind::SyntaxKind;
use cairo_lang_syntax::node::{SyntaxNode, TypedStablePtr, TypedSyntaxNode};
use if_chain::if_chain;
use num_bigint::BigInt;
use salsa::Database;

const PARTIAL_EQ_TRAIT_FUNC_NAME: &str = "core::traits::PartialEq::eq";
const ARRAY_CONSTRUCTOR_TRAIT_FUNC_NAME: &str = "core::array::ArrayTrait::new";
const ARRAY_EMPTY_CREATION_VIA_MACRO: &str = "array![]";

pub struct ManualIsEmpty;

/// ## What it does
///
/// Checks for manual implementation of `is_empty` method in match and if expressions.
///
/// ## Example
///
/// ```cairo
/// fn main() {
///     let ary: Array<u32> = array![1, 2, 3];
///     let _a = match ary {
///         ArrayTrait::new() => true, // or array![], or Default::default(), or ArrayDefault::default()
///         _ => false,
///     };
///     let _b = if ary == array![] { // or ArrayTrait::new(), or `if ary.len() == 0`
///         // do stuff...
///     } else {
///         // do other stuff...
///     }
/// }
/// ```
///
/// Can be replaced with:
///
/// ```cairo
/// fn main() {
///     let res_val: Result<i32> = Result::Err('err');
///     let _a = res_val.is_empty();
///     let _b = if ary.is_empty() {
///         // do stuff...
///     } else {
///         // do other stuff...
///     }
/// }
/// ```
impl Lint for ManualIsEmpty {
    fn allowed_name(&self) -> &'static str {
        "manual_is_empty"
    }

    fn diagnostic_message(&self) -> &'static str {
        "Manual check for `is_empty` detected. Consider using `is_empty()` instead"
    }

    fn kind(&self) -> CairoLintKind {
        CairoLintKind::ManualIsEmpty
    }

    fn has_fixer(&self) -> bool {
        true
    }

    fn fix<'db>(&self, db: &'db dyn Database, node: SyntaxNode<'db>) -> Option<InternalFix<'db>> {
        fix_manual_is_empty(db, node)
    }

    fn fix_message(&self) -> Option<&'static str> {
        Some("Replace condition with `is_empty()` method")
    }
}

pub fn check_manual_is_empty<'db>(
    db: &'db dyn Database,
    item: &ModuleItemId<'db>,
    diagnostics: &mut Vec<PluginDiagnostic<'db>>,
) {
    let functions_bodies = get_all_function_bodies(db, item);
    for function_body in functions_bodies.iter() {
        let arenas = &function_body.arenas;
        for condition in get_all_conditions(function_body) {
            if_chain! {
                if let Condition::BoolExpr(expr) = condition;
                let expr = &arenas.exprs[expr];
                if let Expr::FunctionCall(func_call) = expr;

                if PARTIAL_EQ_TRAIT_FUNC_NAME == function_trait_name_from_fn_id(db, &func_call.function).as_str();
                if check_if_comparison_args_are_incorrect(db, func_call, arenas);

                then {
                    diagnostics.push(PluginDiagnostic {
                        stable_ptr: func_call.stable_ptr.untyped(),
                        message: ManualIsEmpty.diagnostic_message().to_owned(),
                        severity: Severity::Warning,
                        inner_span: None,
                        error_code: None,
                    });
                }
            }
        }
    }
}

/// Rewrites a manual implementation of is_empty
#[tracing::instrument(skip_all, level = "trace")]
pub fn fix_manual_is_empty<'db>(
    db: &'db dyn Database,
    node: SyntaxNode<'db>,
) -> Option<InternalFix<'db>> {
    let typed_node = ExprBinary::cast(db, node)?;
    let node_for_wrapping: SyntaxNode = match (typed_node.lhs(db), typed_node.rhs(db)) {
        (SyntaxExpr::Binary(expr_binary), SyntaxExpr::Literal(_))
        | (SyntaxExpr::Literal(_), SyntaxExpr::Binary(expr_binary)) => {
            expr_binary.lhs(db).as_syntax_node()
        }
        (SyntaxExpr::FunctionCall(call_lhs), SyntaxExpr::FunctionCall(call_rhs)) => {
            // Disambiguate which call we want to wrap with `is_empty()` call - it could be either of them
            let call_lhs_path = call_lhs
                .path(db)
                .as_syntax_node()
                .get_text_without_trivia(db);
            let call_to_replace = if call_lhs_path
                .to_string(db)
                .contains(ARRAY_CONSTRUCTOR_TRAIT_FUNC_NAME)
            {
                call_rhs
            } else {
                call_lhs
            };

            call_to_replace.as_syntax_node()
        }
        (SyntaxExpr::FunctionCall(_) | SyntaxExpr::InlineMacro(_), expr)
        | (expr, SyntaxExpr::FunctionCall(_) | SyntaxExpr::InlineMacro(_)) => expr.as_syntax_node(),
        _ => return None,
    };

    Some(InternalFix {
        node,
        suggestion: format!(
            "{}.is_empty()",
            node_for_wrapping.get_text_without_trivia(db).to_string(db)
        )
        .parse()
        .unwrap(),
        description: ManualIsEmpty.fix_message().unwrap().to_string(),
        import_addition_paths: None,
    })
}

fn check_if_comparison_args_are_incorrect<'db>(
    db: &'db dyn Database,
    comparison: &ExprFunctionCall<'db>,
    arenas: &Arenas<'db>,
) -> bool {
    assert_eq!(comparison.args.len(), 2); // Sanity check
    let (lhs, rhs) = (&comparison.args[0], &comparison.args[1]);
    if_chain! {
        if let ExprFunctionCallArg::Value(expr_id_lhs) = lhs;
        if let ExprFunctionCallArg::Value(expr_id_rhs) = rhs;
        if let Expr::Snapshot(snapshot_lhs) = &arenas.exprs[*expr_id_lhs];
        if let Expr::Snapshot(snapshot_rhs) = &arenas.exprs[*expr_id_rhs];

        then {
            let lhs_inner = &arenas.exprs[snapshot_lhs.inner];
            let rhs_inner = &arenas.exprs[snapshot_rhs.inner];

            // array.len() == 0
            if expr_is_zero_literal(db, rhs_inner) && expr_is_collection_length_call(db, lhs_inner, arenas) {
                return true
            }

            //  0 == array.len()
            if expr_is_zero_literal(db, lhs_inner) && expr_is_collection_length_call(db, rhs_inner, arenas) {
                return true
            }

            // x == array![] or array![] == x
            if expr_is_empty_collection(db, lhs_inner) || expr_is_empty_collection(db, rhs_inner) {
                return true;
            }
        }
    }

    false
}

fn expr_is_empty_collection(db: &dyn Database, expr: &Expr) -> bool {
    // ArrayTrait::new()
    if_chain! {
        if let Expr::FunctionCall(func_call) = expr;
        if func_call.args.is_empty();
        if function_trait_name_from_fn_id(db, &func_call.function) == ARRAY_CONSTRUCTOR_TRAIT_FUNC_NAME;
        then {
            return true;
        }
    }

    // array![]
    if_chain! {
        if let Some(origin_node) = get_originating_syntax_node_for(db, &expr.stable_ptr().0);
        if origin_node.ancestors_with_self(db).any(|node|
            {
                node.kind(db) == SyntaxKind::ExprInlineMacro
                && syntax_node_to_str_without_all_nested_trivia(db, node) == ARRAY_EMPTY_CREATION_VIA_MACRO
            }
        );

        then {
            return true;
        }
    }

    false
}

fn expr_is_zero_literal(db: &dyn Database, expr: &Expr) -> bool {
    if_chain! {
        if let Expr::Literal(literal) = expr;
        if literal.ty.format(db) == U32;
        if literal.value == BigInt::ZERO;

        then {
            return true;
        }
    }

    false
}

fn expr_is_collection_length_call(db: &dyn Database, expr: &Expr, arenas: &Arenas) -> bool {
    if_chain! {
        if let Expr::FunctionCall(func_call) = expr;
        if func_call.args.len() == 1;
        let func_name = func_call.function.name(db);
        if func_name.ends_with("::len\"");
        if let ExprFunctionCallArg::Value(expr) = &func_call.args[0];

        let arg_type = &arenas.exprs[*expr].ty().long(db);
        if is_std_collection_type(db, arg_type);

        then {
            return true;
        }
    }

    false
}

fn is_std_collection_type(db: &dyn Database, type_long_id: &TypeLongId) -> bool {
    match type_long_id {
        TypeLongId::Snapshot(type_id) => {
            let underlying_type = type_id.long(db);
            is_std_collection_type(db, underlying_type)
        }
        TypeLongId::Concrete(concrete_type_id) => {
            let generic_type_name = concrete_type_id.generic_type(db).format(db);
            [ARRAY, SPAN].contains(&generic_type_name.as_str())
        }
        _ => false,
    }
}
