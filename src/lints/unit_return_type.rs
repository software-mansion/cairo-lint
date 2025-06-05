use cairo_lang_defs::{ids::ModuleItemId, plugin::PluginDiagnostic};
use cairo_lang_diagnostics::Severity;
use cairo_lang_semantic::db::SemanticGroup;
use cairo_lang_syntax::node::TypedSyntaxNode;
use cairo_lang_syntax::node::{
    ast::{FunctionSignature, OptionReturnTypeClause},
    db::SyntaxGroup,
    SyntaxNode, TypedStablePtr,
};

use crate::fixes::InternalFix;
use crate::{
    context::{CairoLintKind, Lint},
    queries::get_all_checkable_functions,
};

pub struct UnitReturnType;

/// ## What it does
///
/// Detects if the function has a unit return type, which is not needed to be specified.
///
/// ## Example
///
/// ```cairo
/// fn foo() -> () {
///     println!("Hello, world!");
/// }
/// ```
///
/// Can be simplified to just:
///
/// ```cairo
/// fn foo() {
///     println!("Hello, world!");
/// }
/// ```
impl Lint for UnitReturnType {
    fn allowed_name(&self) -> &'static str {
        "unit_return_type"
    }

    fn diagnostic_message(&self) -> &'static str {
        "unnecessary declared unit return type `()`"
    }

    fn kind(&self) -> CairoLintKind {
        CairoLintKind::UnitReturnType
    }

    fn has_fixer(&self) -> bool {
        true
    }

    fn fix(&self, db: &dyn SemanticGroup, node: SyntaxNode) -> Option<InternalFix> {
        fix_unit_return_type(db, node)
    }

    fn fix_message(&self) -> Option<&'static str> {
        Some("Remove explicit unit return type")
    }
}

pub fn check_unit_return_type(
    db: &dyn SemanticGroup,
    item: &ModuleItemId,
    diagnostics: &mut Vec<PluginDiagnostic>,
) {
    let functions = get_all_checkable_functions(db, item);
    for function in functions {
        let function_signature = db.function_with_body_signature(function).unwrap();
        let return_type = function_signature.return_type;

        // Check if the return type is unit.
        if return_type.is_unit(db) {
            let ast_function_signature = function_signature.stable_ptr.lookup(db);

            // Checks if the return type is explicitly declared as `()`.
            if let OptionReturnTypeClause::ReturnTypeClause(_) = ast_function_signature.ret_ty(db) {
                diagnostics.push(PluginDiagnostic {
                    stable_ptr: function_signature.stable_ptr.untyped(),
                    message: UnitReturnType.diagnostic_message().to_string(),
                    severity: Severity::Warning,
                    relative_span: None,
                    inner_span: None,
                });
            }
        }
    }
}

pub fn fix_unit_return_type(db: &dyn SyntaxGroup, node: SyntaxNode) -> Option<InternalFix> {
    let function_signature = FunctionSignature::from_syntax_node(db, node);
    let return_type_clause = function_signature.ret_ty(db);
    if let OptionReturnTypeClause::ReturnTypeClause(type_clause) = return_type_clause {
        let fixed = node.get_text(db);
        let type_clause_text = type_clause.as_syntax_node().get_text(db);
        let fixed = fixed.replace(&type_clause_text, "");

        if type_clause_text.ends_with(" ") {
            return Some(InternalFix {
                node,
                suggestion: fixed,
                description: UnitReturnType.fix_message().unwrap().to_string(),
                import_addition_paths: None,
            });
        }

        // In case the `()` type doesn't have a space after it, like `fn foo() -> ();`, we trim the end.
        return Some(InternalFix {
            node,
            suggestion: fixed.trim_end().to_string(),
            description: UnitReturnType.fix_message().unwrap().to_string(),
            import_addition_paths: None,
        });
    }
    panic!("Expected a function signature with a return type clause.");
}
