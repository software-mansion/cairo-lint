use crate::{
    context::{CairoLintKind, Lint},
    queries::get_all_function_bodies,
};
use cairo_lang_defs::{
    ids::{LanguageElementId, ModuleItemId},
    plugin::PluginDiagnostic,
};
use cairo_lang_diagnostics::Severity;
use cairo_lang_filesystem::ids::FileId;
use cairo_lang_semantic::{db::SemanticGroup, Expr};
use cairo_lang_syntax::node::{ast, TypedStablePtr};
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
}

pub fn check_redundant_brackets_in_enum_call(
    db: &dyn SemanticGroup,
    item: &ModuleItemId,
    diagnostics: &mut Vec<PluginDiagnostic>,
) {
    let file_id = item.untyped_stable_ptr(db.upcast()).file_id(db.upcast());

    let function_bodies = get_all_function_bodies(db, item);
    for function_body in function_bodies.iter() {
        for (_, expr) in &function_body.arenas.exprs {
            if is_redundant_enum_brackets_call(expr, db, file_id) {
                diagnostics.push(PluginDiagnostic {
                    stable_ptr: expr.stable_ptr().untyped(),
                    message: RedundantBracketsInEnumCall.diagnostic_message().to_string(),
                    severity: Severity::Warning,
                });
            }
        }
    }
}

fn is_redundant_enum_brackets_call(expr: &Expr, db: &dyn SemanticGroup, file_id: FileId) -> bool {
    if_chain! {
        // TODO: (#284)
        // Only consider expressions from the same file to ensure they originate from the user-written code.
        if file_id == expr.stable_ptr().untyped().file_id(db.upcast());

        // Check if the expression is a constructor call for an enum variant,
        if let Expr::EnumVariantCtor(enum_expr) = expr;

        // Check if the type of the enum variant is of unit type `()`.
        if enum_expr.variant.ty.is_unit(db.upcast());

        let node = enum_expr.stable_ptr.lookup(db.upcast());
        if let ast::Expr::FunctionCall(_) = node;

        then {
            return true;
        }
    }

    false
}
