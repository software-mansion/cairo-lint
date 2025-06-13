use cairo_lang_defs::ids::ModuleItemId;
use cairo_lang_defs::plugin::PluginDiagnostic;
use cairo_lang_diagnostics::Severity;
use cairo_lang_semantic::db::SemanticGroup;
use cairo_lang_semantic::{Arenas, Expr, ExprFunctionCall, ExprFunctionCallArg};
use cairo_lang_syntax::node::db::SyntaxGroup;
use cairo_lang_syntax::node::{SyntaxNode, TypedStablePtr, TypedSyntaxNode};
use if_chain::if_chain;

use crate::context::{CairoLintKind, Lint};
use crate::queries::{get_all_function_bodies, get_all_function_calls};

use super::{function_trait_name_from_fn_id, AND, DIV, EQ, GE, GT, LE, LT, NE, NOT, OR, SUB, XOR};

pub struct DivisionEqualityOperation;

/// ## What it does
///
/// Checks for division with identical operands.
///
/// ## Example
///
/// ```cairo
/// fn foo(a: u256) -> u256 {
///     a / a
/// }
/// ```
///
/// Could be simplified by replacing the entire expression with 1:
///
/// ```cairo
/// fn foo(a: u256) -> u256 {
///     1
/// }
/// ```
impl Lint for DivisionEqualityOperation {
    fn allowed_name(&self) -> &'static str {
        "div_eq_op"
    }

    fn diagnostic_message(&self) -> &'static str {
        "Division with identical operands, this operation always results in one (except for zero) and \
                         may indicate a logic error"
    }

    fn kind(&self) -> CairoLintKind {
        CairoLintKind::EqualityOperation
    }
}

pub struct EqualComparisonOperation;

/// ## What it does
///
/// Checks for comparison with identical operands.
///
/// ## Example
///
/// ```cairo
/// fn foo(a: u256) -> bool {
///     a == a
/// }
/// ```
///
/// Could be simplified by replacing the entire expression with true:
///
/// ```cairo
/// fn foo(a: u256) -> bool {
///     true
/// }
/// ```
impl Lint for EqualComparisonOperation {
    fn allowed_name(&self) -> &'static str {
        "eq_comp_op"
    }

    fn diagnostic_message(&self) -> &'static str {
        "Comparison with identical operands, this operation always results in true and may indicate a logic error"
    }

    fn kind(&self) -> CairoLintKind {
        CairoLintKind::EqualityOperation
    }
}

pub struct NotEqualComparisonOperation;

/// ## What it does
///
/// Checks for arithmetical comparison with identical operands.
///
/// ## Example
///
/// ```cairo
/// fn foo(a: u256) -> bool {
///     let _z = a != a;
///     let _y = a > a;
///     a < a
/// }
/// ```
///
/// Could be simplified by replacing the entire expression with false:
///
/// ```cairo
/// fn foo(a: u256) -> bool {
///     let _z = false;
///     let _y = false;
///     false
/// }
/// ```
impl Lint for NotEqualComparisonOperation {
    fn allowed_name(&self) -> &'static str {
        "neq_comp_op"
    }

    fn diagnostic_message(&self) -> &'static str {
        "Comparison with identical operands, this operation always results in false and may indicate a logic error"
    }

    fn kind(&self) -> CairoLintKind {
        CairoLintKind::EqualityOperation
    }
}

pub struct DifferenceEqualityOperation;

/// ## What it does
///
/// Checks for subtraction with identical operands.
///
/// ## Example
///
/// ```cairo
/// fn foo(a: u256) -> u256 {
///     a - a
/// }
/// ```
///
/// Could be simplified by replacing the entire expression with zero:
///
/// ```cairo
/// fn foo(a: u256) -> u256 {
///     0
/// }
/// ```
impl Lint for DifferenceEqualityOperation {
    fn allowed_name(&self) -> &'static str {
        "eq_diff_op"
    }

    fn diagnostic_message(&self) -> &'static str {
        "Subtraction with identical operands, this operation always results in zero and may indicate a logic error"
    }

    fn kind(&self) -> CairoLintKind {
        CairoLintKind::EqualityOperation
    }
}

pub struct BitwiseEqualityOperation;

/// ## What it does
///
/// Checks for bitwise operation with identical operands.
///
/// ## Example
///
/// ```cairo
/// fn foo(a: u256) -> u256 {
///     a & a
/// }
/// ```
///
/// Could be simplified by replacing the entire expression with the operand:
///
/// ```cairo
/// fn foo(a: u256) -> u256 {
///     a
/// }
/// ```
impl Lint for BitwiseEqualityOperation {
    fn allowed_name(&self) -> &'static str {
        "eq_bitwise_op"
    }

    fn diagnostic_message(&self) -> &'static str {
        "Bitwise operation with identical operands, this operation always results in the same \
                             value and may indicate a logic error"
    }

    fn kind(&self) -> CairoLintKind {
        CairoLintKind::EqualityOperation
    }
}

pub struct LogicalEqualityOperation;

/// ## What it does
///
/// Checks for logical operation with identical operands.
///
/// ## Example
///
/// ```cairo
/// fn foo(a: u256) -> u256 {
///     a & a
/// }
/// ```
///
/// Could be simplified by replacing the entire expression with the operand:
///
/// ```cairo
/// fn foo(a: u256) -> u256 {
///     a
/// }
/// ```
impl Lint for LogicalEqualityOperation {
    fn allowed_name(&self) -> &'static str {
        "eq_logical_op"
    }

    fn diagnostic_message(&self) -> &'static str {
        "Logical operation with identical operands, this operation always results in the same \
                             value and may indicate a logic error"
    }

    fn kind(&self) -> CairoLintKind {
        CairoLintKind::EqualityOperation
    }
}

pub fn check_eq_op(
    db: &dyn SemanticGroup,
    item: &ModuleItemId,
    diagnostics: &mut Vec<PluginDiagnostic>,
) {
    let function_bodies = get_all_function_bodies(db, item);
    for function_body in function_bodies.iter() {
        let function_call_exprs = get_all_function_calls(function_body);
        let arenas = &function_body.arenas;
        for function_call_expr in function_call_exprs {
            check_single_eq_op(db, &function_call_expr, arenas, diagnostics);
        }
    }
}

fn check_single_eq_op(
    db: &dyn SemanticGroup,
    expr_func: &ExprFunctionCall,
    arenas: &Arenas,
    diagnostics: &mut Vec<PluginDiagnostic>,
) {
    // We're looking for binary operations
    if expr_func.args.len() != 2 {
        return;
    }
    // Get lhs syntax node to check the text
    let lhs = match &expr_func.args[0] {
        ExprFunctionCallArg::Reference(val) => val.stable_ptr(),
        ExprFunctionCallArg::Value(val) => {
            let expr = &arenas.exprs[*val];
            // If the operands are funtion calls don't lint because the function might have a side effect
            if matches!(expr, Expr::FunctionCall(_)) {
                return;
            }

            if_chain! {
                if let Expr::Snapshot(snapshot) = expr;
                if matches!(arenas.exprs[snapshot.inner], Expr::FunctionCall(_));
                then {
                    return;
                }
            }

            expr.stable_ptr()
        }
    }
    .lookup(db.upcast())
    .as_syntax_node();

    // Get rhs syntax node to check the text
    let rhs = match &expr_func.args[1] {
        ExprFunctionCallArg::Reference(val) => val.stable_ptr(),
        ExprFunctionCallArg::Value(val) => {
            let expr = &arenas.exprs[*val];
            // If the operands are funtion calls don't lint because the function might have a side effect
            if matches!(expr, Expr::FunctionCall(_)) {
                return;
            }

            if_chain! {
                if let Expr::Snapshot(snapshot) = expr;
                if matches!(arenas.exprs[snapshot.inner], Expr::FunctionCall(_));
                then {
                    return;
                }
            }

            expr.stable_ptr()
        }
    }
    .lookup(db.upcast())
    .as_syntax_node();

    let op = function_trait_name_from_fn_id(db, &expr_func.function);

    if are_operands_equal(db.upcast(), lhs, rhs) {
        if let Some(message) = get_diagnostic_message(&op) {
            diagnostics.push(PluginDiagnostic {
                stable_ptr: expr_func.stable_ptr.untyped(),
                message: message.to_owned(),
                severity: Severity::Warning,
                inner_span: None,
            });
        }
    }
}

fn are_operands_equal(db: &dyn SyntaxGroup, lhs: SyntaxNode, rhs: SyntaxNode) -> bool {
    lhs.get_text_without_trivia(db) == rhs.get_text_without_trivia(db)
}

fn get_diagnostic_message(op: &str) -> Option<&'static str> {
    match op {
        EQ | LE | GE => Some(EqualComparisonOperation.diagnostic_message()),
        NE | LT | GT => Some(NotEqualComparisonOperation.diagnostic_message()),
        AND | OR => Some(LogicalEqualityOperation.diagnostic_message()),
        XOR | NOT => Some(BitwiseEqualityOperation.diagnostic_message()),
        SUB => Some(DifferenceEqualityOperation.diagnostic_message()),
        DIV => Some(DivisionEqualityOperation.diagnostic_message()),
        _ => None,
    }
}
