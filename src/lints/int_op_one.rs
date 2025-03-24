use cairo_lang_defs::ids::ModuleItemId;
use cairo_lang_defs::plugin::PluginDiagnostic;
use cairo_lang_diagnostics::Severity;
use cairo_lang_semantic::db::SemanticGroup;
use cairo_lang_semantic::{Arenas, Expr, ExprFunctionCall, ExprFunctionCallArg};
use cairo_lang_syntax::node::ast::{Expr as AstExpr, ExprBinary};
use cairo_lang_syntax::node::db::SyntaxGroup;
use cairo_lang_syntax::node::{SyntaxNode, TypedStablePtr, TypedSyntaxNode};
use if_chain::if_chain;

use crate::context::{CairoLintKind, Lint};
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

    fn fix(&self, db: &dyn SemanticGroup, node: SyntaxNode) -> Option<(SyntaxNode, String)> {
        fix_int_ge_plus_one(db.upcast(), node)
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

    fn fix(&self, db: &dyn SemanticGroup, node: SyntaxNode) -> Option<(SyntaxNode, String)> {
        fix_int_ge_min_one(db.upcast(), node)
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

    fn fix(&self, db: &dyn SemanticGroup, node: SyntaxNode) -> Option<(SyntaxNode, String)> {
        fix_int_le_plus_one(db.upcast(), node)
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

    fn fix(&self, db: &dyn SemanticGroup, node: SyntaxNode) -> Option<(SyntaxNode, String)> {
        fix_int_le_min_one(db.upcast(), node)
    }
}

pub fn check_int_op_one(
    db: &dyn SemanticGroup,
    item: &ModuleItemId,
    diagnostics: &mut Vec<PluginDiagnostic>,
) {
    let function_bodies = get_all_function_bodies(db, item);
    for function_body in function_bodies.iter() {
        let function_call_exprs = get_all_function_calls(function_body);
        let arenas = &function_body.arenas;
        for function_call_expr in function_call_exprs.iter() {
            check_single_int_op_one(db, function_call_expr, arenas, diagnostics);
        }
    }
}

fn check_single_int_op_one(
    db: &dyn SemanticGroup,
    function_call_expr: &ExprFunctionCall,
    arenas: &Arenas,
    diagnostics: &mut Vec<PluginDiagnostic>,
) {
    // Check if the function call is the bool greater or equal (>=) or lower or equal (<=).
    let full_name = function_call_expr.function.full_path(db);
    if !full_name.contains("core::integer::")
        || (!full_name.contains("PartialOrd::ge") && !full_name.contains("PartialOrd::le"))
    {
        return;
    }

    let lhs = &function_call_expr.args[0];
    let rhs = &function_call_expr.args[1];

    // x >= y + 1
    if check_is_variable(lhs, arenas)
        && check_is_add_or_sub_one(db, rhs, arenas, "::add")
        && function_call_expr.function.full_path(db).contains("::ge")
    {
        diagnostics.push(PluginDiagnostic {
            stable_ptr: function_call_expr.stable_ptr.untyped(),
            message: IntegerGreaterEqualPlusOne.diagnostic_message().to_string(),
            severity: Severity::Warning,
        })
    }

    // x - 1 >= y
    if check_is_add_or_sub_one(db, lhs, arenas, "::sub")
        && check_is_variable(rhs, arenas)
        && function_call_expr.function.full_path(db).contains("::ge")
    {
        diagnostics.push(PluginDiagnostic {
            stable_ptr: function_call_expr.stable_ptr.untyped(),
            message: IntegerGreaterEqualMinusOne.diagnostic_message().to_string(),
            severity: Severity::Warning,
        })
    }

    // x + 1 <= y
    if check_is_add_or_sub_one(db, lhs, arenas, "::add")
        && check_is_variable(rhs, arenas)
        && function_call_expr.function.full_path(db).contains("::le")
    {
        diagnostics.push(PluginDiagnostic {
            stable_ptr: function_call_expr.stable_ptr.untyped(),
            message: IntegerLessEqualPlusOne.diagnostic_message().to_string(),
            severity: Severity::Warning,
        })
    }

    // x <= y - 1
    if check_is_variable(lhs, arenas)
        && check_is_add_or_sub_one(db, rhs, arenas, "::sub")
        && function_call_expr.function.full_path(db).contains("::le")
    {
        diagnostics.push(PluginDiagnostic {
            stable_ptr: function_call_expr.stable_ptr.untyped(),
            message: IntegerLessEqualMinusOne.diagnostic_message().to_string(),
            severity: Severity::Warning,
        })
    }
}

fn check_is_variable(arg: &ExprFunctionCallArg, arenas: &Arenas) -> bool {
    if let ExprFunctionCallArg::Value(val_expr) = arg {
        matches!(arenas.exprs[*val_expr], Expr::Var(_))
    } else {
        false
    }
}

fn check_is_add_or_sub_one(
    db: &dyn SemanticGroup,
    arg: &ExprFunctionCallArg,
    arenas: &Arenas,
    operation: &str,
) -> bool {
    let ExprFunctionCallArg::Value(v) = arg else {
        return false;
    };
    let Expr::FunctionCall(ref func_call) = arenas.exprs[*v] else {
        return false;
    };

    // Check is addition or substraction
    let full_name = func_call.function.full_path(db);
    if !full_name.contains("core::integer::") && !full_name.contains(operation)
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
pub fn fix_int_ge_plus_one(db: &dyn SyntaxGroup, node: SyntaxNode) -> Option<(SyntaxNode, String)> {
    let node = ExprBinary::from_syntax_node(db, node);
    let lhs = node.lhs(db).as_syntax_node().get_text(db);

    let AstExpr::Binary(rhs_exp) = node.rhs(db) else {
        panic!("should be addition")
    };
    let rhs = rhs_exp.lhs(db).as_syntax_node().get_text(db);

    let fix = format!("{} > {} ", lhs.trim(), rhs.trim());
    Some((node.as_syntax_node(), fix))
}

/// Rewrites a manual implementation of int ge min one x - 1 >= y
pub fn fix_int_ge_min_one(db: &dyn SyntaxGroup, node: SyntaxNode) -> Option<(SyntaxNode, String)> {
    let node = ExprBinary::from_syntax_node(db, node);
    let AstExpr::Binary(lhs_exp) = node.lhs(db) else {
        panic!("should be substraction")
    };
    let rhs = node.rhs(db).as_syntax_node().get_text(db);

    let lhs = lhs_exp.lhs(db).as_syntax_node().get_text(db);

    let fix = format!("{} > {} ", lhs.trim(), rhs.trim());
    Some((node.as_syntax_node(), fix))
}

/// Rewrites a manual implementation of int le plus one x + 1 <= y
pub fn fix_int_le_plus_one(db: &dyn SyntaxGroup, node: SyntaxNode) -> Option<(SyntaxNode, String)> {
    let node = ExprBinary::from_syntax_node(db, node);
    let AstExpr::Binary(lhs_exp) = node.lhs(db) else {
        panic!("should be addition")
    };
    let rhs = node.rhs(db).as_syntax_node().get_text(db);

    let lhs = lhs_exp.lhs(db).as_syntax_node().get_text(db);

    let fix = format!("{} < {} ", lhs.trim(), rhs.trim());
    Some((node.as_syntax_node(), fix))
}

/// Rewrites a manual implementation of int le min one x <= y -1
pub fn fix_int_le_min_one(db: &dyn SyntaxGroup, node: SyntaxNode) -> Option<(SyntaxNode, String)> {
    let node = ExprBinary::from_syntax_node(db, node);
    let lhs = node.lhs(db).as_syntax_node().get_text(db);

    let AstExpr::Binary(rhs_exp) = node.rhs(db) else {
        panic!("should be substraction")
    };
    let rhs = rhs_exp.lhs(db).as_syntax_node().get_text(db);

    let fix = format!("{} < {} ", lhs.trim(), rhs.trim());
    Some((node.as_syntax_node(), fix))
}
