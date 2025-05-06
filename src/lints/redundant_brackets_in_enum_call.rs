use crate::{
    context::{CairoLintKind, Lint},
    queries::get_all_function_bodies,
};
use cairo_lang_defs::{
    ids::{ModuleItemId, VariantId},
    plugin::PluginDiagnostic,
};
use cairo_lang_diagnostics::Severity;
use cairo_lang_semantic::{db::SemanticGroup, Expr};
use cairo_lang_syntax::node::{
    ast::{self, OptionTypeClause, OptionWrappedGenericParamList},
    db::SyntaxGroup,
    helpers::{GenericParamEx, IsDependentType},
    SyntaxNode, TypedStablePtr, TypedSyntaxNode,
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

    fn fix(&self, db: &dyn SemanticGroup, node: SyntaxNode) -> Option<(SyntaxNode, String)> {
        fix_redundant_brackets_in_enum_call(db.upcast(), node)
    }
}

pub fn check_redundant_brackets_in_enum_call(
    db: &dyn SemanticGroup,
    item: &ModuleItemId,
    diagnostics: &mut Vec<PluginDiagnostic>,
) {
    let function_bodies = get_all_function_bodies(db, item);
    for function_body in function_bodies.iter() {
        for (_, expr) in &function_body.arenas.exprs {
            if is_redundant_enum_brackets_call(expr, db) {
                diagnostics.push(PluginDiagnostic {
                    stable_ptr: expr.stable_ptr().untyped(),
                    message: RedundantBracketsInEnumCall.diagnostic_message().to_string(),
                    severity: Severity::Warning,
                    relative_span: None,
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

        let node = enum_expr.stable_ptr.lookup(db);
        if let ast::Expr::FunctionCall(_) = node;

        // Check if the variant's type clause depends on the enum's generic parameters
        if !type_clause_uses_generics(enum_expr.variant.id, db);

        then {
            return true;
        }
    }

    false
}

fn type_clause_uses_generics(variant_id: VariantId, db: &dyn SemanticGroup) -> bool {
    let variant_ast = variant_id.stable_ptr(db).lookup(db);

    // Extract type clause (e.g., in `VariantName: T`, this matches `: T`)
    let OptionTypeClause::TypeClause(clause) = variant_ast.type_clause(db) else {
        return false;
    };

    let enum_ast = variant_id.enum_id(db).stable_ptr(db).lookup(db);

    // Extract generic parameters, if present
    let OptionWrappedGenericParamList::WrappedGenericParamList(generic_list) =
        enum_ast.generic_params(db)
    else {
        return false;
    };

    // Collect generic parameter names.
    // e.g., for `enum Result<T, E>`, the result will be ["T", "E"]
    let identifiers: Vec<String> = generic_list
        .generic_params(db)
        .elements(db)
        .iter()
        .filter_map(|param| {
            param
                .name(db)
                .map(|name| name.token(db).as_syntax_node().get_text_without_trivia(db))
        })
        .collect();

    let identifiers_refs: Vec<&str> = identifiers.iter().map(String::as_str).collect();

    clause.ty(db).is_dependent_type(db, &identifiers_refs)
}

fn fix_redundant_brackets_in_enum_call(
    db: &dyn SyntaxGroup,
    node: SyntaxNode,
) -> Option<(SyntaxNode, String)> {
    let ast_expr = ast::Expr::from_syntax_node(db, node);

    let ast::Expr::FunctionCall(call_expr) = &ast_expr else {
        panic!("Expr should be a FunctionCall");
    };

    // Retrieve parentheses that can be removed
    let arguments = call_expr.arguments(db).as_syntax_node().get_text(db);

    let fixed_expr = ast_expr
        .as_syntax_node()
        .get_text(db)
        .strip_suffix(&arguments)?
        .to_string();

    Some((node, fixed_expr))
}
