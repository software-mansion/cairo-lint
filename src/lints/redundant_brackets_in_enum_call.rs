use crate::{
    context::{CairoLintKind, Lint},
    corelib::CorelibContext,
    fixer::InternalFix,
    queries::get_all_function_bodies,
};
use cairo_lang_defs::{ids::ModuleItemId, plugin::PluginDiagnostic};
use cairo_lang_diagnostics::Severity;
use cairo_lang_semantic::{ConcreteVariant, Expr, db::SemanticGroup};
use cairo_lang_syntax::node::{
    SyntaxNode, Terminal, TypedStablePtr, TypedSyntaxNode,
    ast::{self, OptionTypeClause},
    db::SyntaxGroup,
};
use if_chain::if_chain;

pub struct RedundantBracketsInEnumCall;

/// ## What it does
///
/// Detects calls to enum variant constructors with redundant parentheses
///
/// ## Example
///
/// ```cairo
/// enum MyEnum {
///     Data: u8,
///     Empty,
/// }
///
/// fn main() {
///     let a = MyEnum::Empty(()); // redundant parentheses
/// }
/// ```
///
/// Can be simplified to:
///
/// ```cairo
/// enum MyEnum {
///     Data: u8,
///     Empty,
/// }
///
/// fn main() {
///     let a = MyEnum::Empty;
/// }
/// ```
impl Lint for RedundantBracketsInEnumCall {
    fn allowed_name(&self) -> &'static str {
        "redundant_brackets_in_enum_call"
    }

    fn diagnostic_message(&self) -> &'static str {
        "redundant parentheses in enum call"
    }

    fn kind(&self) -> CairoLintKind {
        CairoLintKind::EnumEmptyVariantBrackets
    }

    fn has_fixer(&self) -> bool {
        true
    }

    fn fix<'db>(
        &self,
        db: &'db dyn SemanticGroup,
        node: SyntaxNode<'db>,
    ) -> Option<InternalFix<'db>> {
        fix_redundant_brackets_in_enum_call(db, node)
    }

    fn fix_message(&self) -> Option<&'static str> {
        Some("Remove redundant parentheses in enum call")
    }
}

#[tracing::instrument(skip_all, level = "trace")]
pub fn check_redundant_brackets_in_enum_call<'db>(
    db: &'db dyn SemanticGroup,
    _corelib_context: &CorelibContext<'db>,
    item: &ModuleItemId<'db>,
    diagnostics: &mut Vec<PluginDiagnostic<'db>>,
) {
    let function_bodies = get_all_function_bodies(db, item);
    for function_body in function_bodies.iter() {
        for (_, expr) in &function_body.arenas.exprs {
            if is_redundant_enum_brackets_call(expr, db) {
                diagnostics.push(PluginDiagnostic {
                    stable_ptr: expr.stable_ptr().untyped(),
                    message: RedundantBracketsInEnumCall.diagnostic_message().to_string(),
                    severity: Severity::Warning,
                    inner_span: None,
                });
            }
        }
    }
}

fn is_redundant_enum_brackets_call(expr: &Expr, db: &dyn SemanticGroup) -> bool {
    if_chain! {
        // Check if the expression is a constructor call for an enum variant,
        if let Expr::EnumVariantCtor(enum_expr) = expr;

        // Check if the type of the enum variant is of unit type `()`.
        if enum_expr.variant.ty.is_unit(db);

        // Without the parentheses at the end, it would not be defined as a function call.
        if let ast::Expr::FunctionCall(func_call) = expr.stable_ptr().lookup(db);

        let mut args = func_call.arguments(db).arguments(db).elements(db);

        // There should be exactly one argument which is the `()`
        if args.len() == 1;

        // Verify the argument explicitly match unit syntax `()` (empty tuple) and not just semantically unit type.
        if let ast::ArgClause::Unnamed(unnamed_clause) = args.next().unwrap().arg_clause(db);
        if let ast::Expr::Tuple(tuple) = unnamed_clause.value(db);
        if tuple.expressions(db).elements(db).len() == 0;

        // Check if the variant's type clause depends on the enum's generic parameters
        if match find_generic_param_with_index(&enum_expr.variant, db) {
            // No generics - parentheses are redundant
            None =>  true,
            // Only keep () if the generic param, at given index, isn't unit.
            Some((index, generic_param_name)) => has_unit_generic_arg_at_index(&func_call, index, generic_param_name, db),
        };

        then {
            return true;
        }
    }

    false
}

/// Returns Some((index, name)) if the enum variant's type clause uses one of the enum's
/// generic parameters, returning its position and name. e.g., `T` returns (0, "T") if used and
/// the enum is declared as `enum MyEnum<T, E> { ... }`
fn find_generic_param_with_index<'db>(
    variant: &ConcreteVariant<'db>,
    db: &'db dyn SemanticGroup,
) -> Option<(usize, String)> {
    let variant_id = variant.id;
    let variant_ast = variant_id.stable_ptr(db).lookup(db);

    // Extract type clause (e.g., in `VariantName: T`, this matches `: T`)
    let OptionTypeClause::TypeClause(clause) = variant_ast.type_clause(db) else {
        return None;
    };

    // Retrieve the generic parameters from the semantic model of the enum.
    let generic_params = db
        .enum_generic_params(variant.concrete_enum_id.enum_id(db))
        .ok()?;

    let ast::Expr::Path(path) = clause.ty(db) else {
        return None;
    };

    // Iterates over path segments (e.g., `T` in `VariantName: T`) to find matches with enum generic parameters.
    path.segments(db).elements(db).find_map(|segment| {
        let ast::PathSegment::Simple(simple_segment) = segment else {
            return None;
        };

        let param_name = simple_segment.ident(db).text(db);

        // Find the position of this parameter in the enum's generic parameters list
        // and return the (index, name) if found
        generic_params
            .iter()
            .position(|param| param.id().name(db).as_ref() == Some(&param_name))
            .map(|index| (index, param_name.to_string()))
    })
}

/// Returns true if the generic argument at `index_to_match` is a unit type `()`.
/// Handles both named arguments (matching against `generic_param_name`) and
/// unnamed arguments at the specified position in the path segment's generic args.
fn has_unit_generic_arg_at_index<'db>(
    func_call: &ast::ExprFunctionCall<'db>,
    index_to_match: usize,
    generic_param_name: String,
    db: &'db dyn SemanticGroup,
) -> bool {
    for segment in func_call.path(db).segments(db).elements(db) {
        let ast::PathSegment::WithGenericArgs(path_segment) = &segment else {
            continue;
        };

        let mut args = path_segment.generic_args(db).generic_args(db).elements(db);

        if_chain! {
            if let Some(arg) = args.nth(index_to_match);

            if let Some(ast::GenericArgValue::Expr(arg_val)) = match arg {
                // Match named argument if it matches our target generic parameter
                ast::GenericArg::Named(named_arg) if named_arg.name(db).text(db) == generic_param_name => {
                    Some(named_arg.value(db))
                },
                // Skip other named arguments
                ast::GenericArg::Named(_) => None,
                // Handle unnamed arguments
                ast::GenericArg::Unnamed(unnamed_arg) => Some(unnamed_arg.value(db))
            };

            if let ast::Expr::Tuple(unit) = arg_val.expr(db);

            // Check if the tuple is empty; if it is, it means it is a unit type
            if unit.expressions(db).elements(db).len() == 0;

            then {
                return true;
            }
        }
    }

    false
}

#[tracing::instrument(skip_all, level = "trace")]
fn fix_redundant_brackets_in_enum_call<'db>(
    db: &'db dyn SyntaxGroup,
    node: SyntaxNode<'db>,
) -> Option<InternalFix<'db>> {
    let ast_expr = ast::Expr::from_syntax_node(db, node);

    let ast::Expr::FunctionCall(call_expr) = &ast_expr else {
        panic!("Expr should be a FunctionCall");
    };

    // Retrieve parentheses that can be removed
    let arguments = call_expr
        .arguments(db)
        .as_syntax_node()
        .get_text_without_trivia(db);

    let fixed_expr = ast_expr
        .as_syntax_node()
        .get_text(db)
        .strip_suffix(&arguments)?
        .to_string();

    Some(InternalFix {
        node,
        suggestion: fixed_expr,
        description: RedundantBracketsInEnumCall
            .fix_message()
            .unwrap()
            .to_string(),
        import_addition_paths: None,
    })
}
