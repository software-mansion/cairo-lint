use cairo_lang_defs::ids::{LookupItemId, ModuleId, ModuleItemId, TraitFunctionId};
use cairo_lang_defs::plugin::PluginDiagnostic;
use cairo_lang_diagnostics::Severity;
use cairo_lang_semantic::db::SemanticGroup;
use cairo_lang_semantic::items::functions::GenericFunctionId;
use cairo_lang_semantic::items::imp::ImplHead;
use cairo_lang_semantic::{Arenas, Expr, ExprFunctionCall, ExprFunctionCallArg};
use cairo_lang_syntax::node::ast::{Expr as AstExpr, ExprBinary};
use cairo_lang_syntax::node::db::SyntaxGroup;
use cairo_lang_syntax::node::{SyntaxNode, TypedStablePtr, TypedSyntaxNode};
use if_chain::if_chain;

use crate::context::{CairoLintKind, Lint};
use crate::corelib::CorelibContext;
use crate::fixer::InternalFix;
use crate::helper::is_item_ancestor_of_module;
use crate::queries::{get_all_function_bodies, get_all_function_calls};

pub struct IntegerGreaterEqualPlusOne;

/// ## What it does
///
/// Check for unnecessary add operation in integer >= comparison.
///
/// ## Example
///
/// ```cairo
/// fn main() {
///     let x: u32 = 1;
///     let y: u32 = 1;
///     if x >= y + 1 {}
/// }
/// ```
///
/// Can be simplified to:
///
/// ```cairo
/// fn main() {
///     let x: u32 = 1;
///     let y: u32 = 1;
///     if x > y {}
/// }
/// ```
impl Lint for IntegerGreaterEqualPlusOne {
    fn allowed_name(&self) -> &'static str {
        "int_ge_plus_one"
    }

    fn diagnostic_message(&self) -> &'static str {
        "Unnecessary add operation in integer >= comparison. Use simplified comparison."
    }

    fn kind(&self) -> CairoLintKind {
        CairoLintKind::IntGePlusOne
    }

    fn has_fixer(&self) -> bool {
        true
    }

    fn fix<'db>(
        &self,
        db: &'db dyn SemanticGroup,
        node: SyntaxNode<'db>,
    ) -> Option<InternalFix<'db>> {
        fix_int_ge_plus_one(db, node)
    }

    fn fix_message(&self) -> Option<&'static str> {
        Some("Replace with simplified '>' comparison")
    }
}

pub struct IntegerGreaterEqualMinusOne;

/// ## What it does
///
/// Check for unnecessary sub operation in integer >= comparison.
///
/// ## Example
///
/// ```cairo
/// fn main() {
///     let x: u32 = 1;
///     let y: u32 = 1;
///     if x - 1 >= y {}
/// }
/// ```
///
/// Can be simplified to:
///
/// ```cairo
/// fn main() {
///     let x: u32 = 1;
///     let y: u32 = 1;
///     if x > y {}
/// }
/// ```
impl Lint for IntegerGreaterEqualMinusOne {
    fn allowed_name(&self) -> &'static str {
        "int_ge_min_one"
    }

    fn diagnostic_message(&self) -> &'static str {
        "Unnecessary sub operation in integer >= comparison. Use simplified comparison."
    }

    fn kind(&self) -> CairoLintKind {
        CairoLintKind::IntGeMinOne
    }

    fn has_fixer(&self) -> bool {
        true
    }

    fn fix<'db>(
        &self,
        db: &'db dyn SemanticGroup,
        node: SyntaxNode<'db>,
    ) -> Option<InternalFix<'db>> {
        fix_int_ge_min_one(db, node)
    }

    fn fix_message(&self) -> Option<&'static str> {
        Some("Replace with simplified '>' comparison")
    }
}

pub struct IntegerLessEqualPlusOne;

/// ## What it does
///
/// Check for unnecessary add operation in integer <= comparison.
///
/// ## Example
///
/// ```cairo
/// fn main() {
///     let x: u32 = 1;
///     let y: u32 = 1;
///     if x + 1 <= y {}
/// }
/// ```
///
/// Can be simplified to:
///
/// ```cairo
/// fn main() {
///     let x: u32 = 1;
///     let y: u32 = 1;
///     if x < y {}
/// }
/// ```
impl Lint for IntegerLessEqualPlusOne {
    fn allowed_name(&self) -> &'static str {
        "int_le_plus_one"
    }

    fn diagnostic_message(&self) -> &'static str {
        "Unnecessary add operation in integer <= comparison. Use simplified comparison."
    }

    fn kind(&self) -> CairoLintKind {
        CairoLintKind::IntLePlusOne
    }

    fn has_fixer(&self) -> bool {
        true
    }

    fn fix<'db>(
        &self,
        db: &'db dyn SemanticGroup,
        node: SyntaxNode<'db>,
    ) -> Option<InternalFix<'db>> {
        fix_int_le_plus_one(db, node)
    }

    fn fix_message(&self) -> Option<&'static str> {
        Some("Replace with simplified '<' comparison")
    }
}

pub struct IntegerLessEqualMinusOne;

/// ## What it does
///
/// Check for unnecessary sub operation in integer <= comparison.
///
/// ## Example
///
/// ```cairo
/// fn main() {
///     let x: u32 = 1;
///     let y: u32 = 1;
///     if x <= y - 1 {}
/// }
/// ```
///
/// Can be simplified to:
///
/// ```cairo
/// fn main() {
///     let x: u32 = 1;
///     let y: u32 = 1;
///     if x < y {}
/// }
/// ```
impl Lint for IntegerLessEqualMinusOne {
    fn allowed_name(&self) -> &'static str {
        "int_le_min_one"
    }

    fn diagnostic_message(&self) -> &'static str {
        "Unnecessary sub operation in integer <= comparison. Use simplified comparison."
    }

    fn kind(&self) -> CairoLintKind {
        CairoLintKind::IntLeMinOne
    }

    fn has_fixer(&self) -> bool {
        true
    }

    fn fix<'db>(
        &self,
        db: &'db dyn SemanticGroup,
        node: SyntaxNode<'db>,
    ) -> Option<InternalFix<'db>> {
        fix_int_le_min_one(db, node)
    }

    fn fix_message(&self) -> Option<&'static str> {
        Some("Replace with simplified '<' comparison")
    }
}

#[tracing::instrument(skip_all, level = "trace")]
pub fn check_int_op_one<'db>(
    db: &'db dyn SemanticGroup,
    corelib_context: &CorelibContext<'db>,
    item: &ModuleItemId<'db>,
    diagnostics: &mut Vec<PluginDiagnostic<'db>>,
) {
    let function_bodies = get_all_function_bodies(db, item);
    for function_body in function_bodies.iter() {
        let function_call_exprs = get_all_function_calls(function_body);
        let arenas = &function_body.arenas;
        for function_call_expr in function_call_exprs {
            check_single_int_op_one(
                db,
                corelib_context,
                &function_call_expr,
                arenas,
                diagnostics,
            );
        }
    }
}

fn check_single_int_op_one<'db>(
    db: &'db dyn SemanticGroup,
    corelib_context: &CorelibContext<'db>,
    function_call_expr: &ExprFunctionCall<'db>,
    arenas: &Arenas<'db>,
    diagnostics: &mut Vec<PluginDiagnostic<'db>>,
) {
    // Check if the function call is part of the implementation.
    let GenericFunctionId::Impl(impl_generic_func_id) = function_call_expr
        .function
        .get_concrete(db)
        .generic_function
    else {
        return;
    };

    // Check if the function call is the bool greater or equal (>=) or lower or equal (<=).
    if impl_generic_func_id.function != corelib_context.get_partial_ord_ge_trait_function_id()
        && impl_generic_func_id.function != corelib_context.get_partial_ord_le_trait_function_id()
    {
        return;
    }

    // Check if the function call is part of the corelib integer module.
    let is_part_of_corelib_integer =
        if let Some(ImplHead::Concrete(impl_def_id)) = impl_generic_func_id.impl_id.head(db) {
            is_item_ancestor_of_module(
                db,
                &LookupItemId::ModuleItem(ModuleItemId::Impl(impl_def_id)),
                ModuleId::Submodule(corelib_context.get_integer_module_id()),
            )
        } else {
            false
        };

    if !is_part_of_corelib_integer {
        return;
    }

    let lhs = &function_call_expr.args[0];
    let rhs = &function_call_expr.args[1];

    let add_trait_function_id = corelib_context.get_add_trait_function_id();
    let sub_trait_function_id = corelib_context.get_sub_trait_function_id();
    let partial_ord_ge_trait_function_id = corelib_context.get_partial_ord_ge_trait_function_id();
    let partial_ord_le_trait_function_id = corelib_context.get_partial_ord_le_trait_function_id();

    // x >= y + 1
    if check_is_variable(lhs, arenas)
        && check_is_add_or_sub_one(
            db,
            rhs,
            arenas,
            is_part_of_corelib_integer,
            add_trait_function_id,
        )
        && impl_generic_func_id.function == partial_ord_ge_trait_function_id
    {
        diagnostics.push(PluginDiagnostic {
            stable_ptr: function_call_expr.stable_ptr.untyped(),
            message: IntegerGreaterEqualPlusOne.diagnostic_message().to_string(),
            severity: Severity::Warning,
            inner_span: None,
        })
    }

    // x - 1 >= y
    if check_is_add_or_sub_one(
        db,
        lhs,
        arenas,
        is_part_of_corelib_integer,
        sub_trait_function_id,
    ) && check_is_variable(rhs, arenas)
        && impl_generic_func_id.function == partial_ord_ge_trait_function_id
    {
        diagnostics.push(PluginDiagnostic {
            stable_ptr: function_call_expr.stable_ptr.untyped(),
            message: IntegerGreaterEqualMinusOne.diagnostic_message().to_string(),
            severity: Severity::Warning,
            inner_span: None,
        })
    }

    // x + 1 <= y
    if check_is_add_or_sub_one(
        db,
        lhs,
        arenas,
        is_part_of_corelib_integer,
        add_trait_function_id,
    ) && check_is_variable(rhs, arenas)
        && impl_generic_func_id.function == partial_ord_le_trait_function_id
    {
        diagnostics.push(PluginDiagnostic {
            stable_ptr: function_call_expr.stable_ptr.untyped(),
            message: IntegerLessEqualPlusOne.diagnostic_message().to_string(),
            severity: Severity::Warning,
            inner_span: None,
        })
    }

    // x <= y - 1
    if check_is_variable(lhs, arenas)
        && check_is_add_or_sub_one(
            db,
            rhs,
            arenas,
            is_part_of_corelib_integer,
            sub_trait_function_id,
        )
        && impl_generic_func_id.function == partial_ord_le_trait_function_id
    {
        diagnostics.push(PluginDiagnostic {
            stable_ptr: function_call_expr.stable_ptr.untyped(),
            message: IntegerLessEqualMinusOne.diagnostic_message().to_string(),
            severity: Severity::Warning,
            inner_span: None,
        })
    }
}

fn check_is_variable<'db>(arg: &ExprFunctionCallArg<'db>, arenas: &Arenas<'db>) -> bool {
    if let ExprFunctionCallArg::Value(val_expr) = arg {
        matches!(arenas.exprs[*val_expr], Expr::Var(_))
    } else {
        false
    }
}

fn check_is_add_or_sub_one<'db>(
    db: &'db dyn SemanticGroup,
    arg: &ExprFunctionCallArg<'db>,
    arenas: &Arenas<'db>,
    is_part_of_corelib_integer: bool,
    operation_function_trait_id: TraitFunctionId<'db>,
) -> bool {
    let ExprFunctionCallArg::Value(v) = arg else {
        return false;
    };
    let Expr::FunctionCall(ref func_call) = arenas.exprs[*v] else {
        return false;
    };

    let GenericFunctionId::Impl(impl_generic_func_id) =
        func_call.function.get_concrete(db).generic_function
    else {
        return false;
    };

    // Check is addition or substraction
    if !is_part_of_corelib_integer && impl_generic_func_id.function != operation_function_trait_id
        || func_call.args.len() != 2
    {
        return false;
    }

    let lhs = &func_call.args[0];
    let rhs = &func_call.args[1];

    // Check lhs is var
    if let ExprFunctionCallArg::Value(v) = lhs {
        let Expr::Var(_) = arenas.exprs[*v] else {
            return false;
        };
    };

    // Check rhs is 1
    if_chain! {
        if let ExprFunctionCallArg::Value(v) = rhs;
        if let Expr::Literal(ref litteral_expr) = arenas.exprs[*v];
        if litteral_expr.value == 1.into();
        then {
            return true;
        }
    }

    false
}

/// Rewrites a manual implementation of int ge plus one x >= y + 1
#[tracing::instrument(skip_all, level = "trace")]
pub fn fix_int_ge_plus_one<'db>(
    db: &'db dyn SyntaxGroup,
    node: SyntaxNode<'db>,
) -> Option<InternalFix<'db>> {
    let node = ExprBinary::from_syntax_node(db, node);
    let lhs = node.lhs(db).as_syntax_node().get_text(db);

    let AstExpr::Binary(rhs_exp) = node.rhs(db) else {
        panic!("should be addition")
    };
    let rhs = rhs_exp.lhs(db).as_syntax_node().get_text(db);

    let fix = format!("{} > {} ", lhs.trim(), rhs.trim());
    Some(InternalFix {
        node: node.as_syntax_node(),
        suggestion: fix,
        description: IntegerGreaterEqualPlusOne
            .fix_message()
            .unwrap()
            .to_string(),
        import_addition_paths: None,
    })
}

/// Rewrites a manual implementation of int ge min one x - 1 >= y
#[tracing::instrument(skip_all, level = "trace")]
pub fn fix_int_ge_min_one<'db>(
    db: &'db dyn SyntaxGroup,
    node: SyntaxNode<'db>,
) -> Option<InternalFix<'db>> {
    let node = ExprBinary::from_syntax_node(db, node);
    let AstExpr::Binary(lhs_exp) = node.lhs(db) else {
        panic!("should be substraction")
    };
    let rhs = node.rhs(db).as_syntax_node().get_text(db);

    let lhs = lhs_exp.lhs(db).as_syntax_node().get_text(db);

    let fix = format!("{} > {} ", lhs.trim(), rhs.trim());
    Some(InternalFix {
        node: node.as_syntax_node(),
        suggestion: fix,
        description: IntegerGreaterEqualMinusOne
            .fix_message()
            .unwrap()
            .to_string(),
        import_addition_paths: None,
    })
}

/// Rewrites a manual implementation of int le plus one x + 1 <= y
#[tracing::instrument(skip_all, level = "trace")]
pub fn fix_int_le_plus_one<'db>(
    db: &'db dyn SyntaxGroup,
    node: SyntaxNode<'db>,
) -> Option<InternalFix<'db>> {
    let node = ExprBinary::from_syntax_node(db, node);
    let AstExpr::Binary(lhs_exp) = node.lhs(db) else {
        panic!("should be addition")
    };
    let rhs = node.rhs(db).as_syntax_node().get_text(db);

    let lhs = lhs_exp.lhs(db).as_syntax_node().get_text(db);

    let fix = format!("{} < {} ", lhs.trim(), rhs.trim());
    Some(InternalFix {
        node: node.as_syntax_node(),
        suggestion: fix,
        description: IntegerLessEqualPlusOne.fix_message().unwrap().to_string(),
        import_addition_paths: None,
    })
}

/// Rewrites a manual implementation of int le min one x <= y -1
#[tracing::instrument(skip_all, level = "trace")]
pub fn fix_int_le_min_one<'db>(
    db: &'db dyn SyntaxGroup,
    node: SyntaxNode<'db>,
) -> Option<InternalFix<'db>> {
    let node = ExprBinary::from_syntax_node(db, node);
    let lhs = node.lhs(db).as_syntax_node().get_text(db);

    let AstExpr::Binary(rhs_exp) = node.rhs(db) else {
        panic!("should be substraction")
    };
    let rhs = rhs_exp.lhs(db).as_syntax_node().get_text(db);

    let fix = format!("{} < {} ", lhs.trim(), rhs.trim());
    Some(InternalFix {
        node: node.as_syntax_node(),
        suggestion: fix,
        description: IntegerLessEqualMinusOne.fix_message().unwrap().to_string(),
        import_addition_paths: None,
    })
}
