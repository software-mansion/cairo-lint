use std::collections::HashSet;

use cairo_lang_defs::ids::ModuleItemId;
use cairo_lang_defs::plugin::PluginDiagnostic;
use cairo_lang_diagnostics::Severity;
use cairo_lang_semantic::db::SemanticGroup;
use cairo_lang_semantic::{
    Arenas, Expr, ExprFunctionCall, ExprFunctionCallArg, ExprLogicalOperator, LogicalOperator,
};
use cairo_lang_syntax::node::ast::{BinaryOperator, Expr as AstExpr};
use cairo_lang_syntax::node::db::SyntaxGroup;
use cairo_lang_syntax::node::helpers::QueryAttrs;
use cairo_lang_syntax::node::{SyntaxNode, TypedStablePtr, TypedSyntaxNode};

use super::function_trait_name_from_fn_id;
use crate::lints::{EQ, GE, GT, LE, LT};
use crate::queries::{get_all_function_bodies, get_all_logical_operator_expressions};

pub const ALLOWED_NAME: &str = "double_comparison";

pub const SIMPLIFIABLE_COMPARISON: &str = "This double comparison can be simplified.";
pub const REDUNDANT_COMPARISON: &str =
    "Redundant double comparison found. Consider simplifying to a single comparison.";
pub const CONTRADICTORY_COMPARISON: &str =
    "This double comparison is contradictory and always false.";
pub const IMPOSSIBLE_COMPARISON: &str = "Impossible condition, always false";

pub fn check_double_comparison(
    db: &dyn SemanticGroup,
    item: &ModuleItemId,
    diagnostics: &mut Vec<PluginDiagnostic>,
) {
    let function_bodies = get_all_function_bodies(db, item);
    for function_body in function_bodies.iter() {
        let logical_operator_exprs = get_all_logical_operator_expressions(function_body);
        let arenas = &function_body.arenas;
        for logical_operator_expr in logical_operator_exprs.iter() {
            check_single_double_comparison(db, logical_operator_expr, arenas, diagnostics);
        }
    }
}

fn check_single_double_comparison(
    db: &dyn SemanticGroup,
    logical_operator_exprs: &ExprLogicalOperator,
    arenas: &Arenas,
    diagnostics: &mut Vec<PluginDiagnostic>,
) {
    // Checks if the lint is allowed in an upper scope.
    let mut current_node = logical_operator_exprs
        .stable_ptr
        .lookup(db.upcast())
        .as_syntax_node();
    while let Some(node) = current_node.parent() {
        if node.has_attr_with_arg(db.upcast(), "allow", ALLOWED_NAME) {
            return;
        }
        current_node = node;
    }

    let Expr::FunctionCall(lhs_comparison) = &arenas.exprs[logical_operator_exprs.lhs] else {
        return;
    };
    // If it's not 2 args it cannot be a regular comparison
    if lhs_comparison.args.len() != 2 {
        return;
    }

    let Expr::FunctionCall(rhs_comparison) = &arenas.exprs[logical_operator_exprs.rhs] else {
        return;
    };
    // If it's not 2 args it cannot be a regular comparison
    if rhs_comparison.args.len() != 2 {
        return;
    }
    // Get the full name of the function used (trait name)
    let (lhs_fn_trait_name, rhs_fn_trait_name) = (
        function_trait_name_from_fn_id(db, &lhs_comparison.function),
        function_trait_name_from_fn_id(db, &rhs_comparison.function),
    );

    // Check the impossible comparison
    if check_impossible_comparison(
        lhs_comparison,
        rhs_comparison,
        &lhs_fn_trait_name,
        &rhs_fn_trait_name,
        logical_operator_exprs,
        db,
        arenas,
    ) {
        diagnostics.push(PluginDiagnostic {
            message: IMPOSSIBLE_COMPARISON.to_string(),
            stable_ptr: logical_operator_exprs.stable_ptr.untyped(),
            severity: Severity::Error,
        })
    }

    // The comparison functions don't work with refs so should only be value
    let (llhs, rlhs) = match (&lhs_comparison.args[0], &lhs_comparison.args[1]) {
        (ExprFunctionCallArg::Value(l_expr_id), ExprFunctionCallArg::Value(r_expr_id)) => {
            (&arenas.exprs[*l_expr_id], &arenas.exprs[*r_expr_id])
        }
        _ => {
            return;
        }
    };
    let (lrhs, rrhs) = match (&rhs_comparison.args[0], &rhs_comparison.args[1]) {
        (ExprFunctionCallArg::Value(l_expr_id), ExprFunctionCallArg::Value(r_expr_id)) => {
            (&arenas.exprs[*l_expr_id], &arenas.exprs[*r_expr_id])
        }
        _ => return,
    };
    // Get all the operands
    let llhs_var = llhs
        .stable_ptr()
        .lookup(db.upcast())
        .as_syntax_node()
        .get_text_without_trivia(db.upcast());
    let rlhs_var = rlhs
        .stable_ptr()
        .lookup(db.upcast())
        .as_syntax_node()
        .get_text_without_trivia(db.upcast());
    let lrhs_var = lrhs
        .stable_ptr()
        .lookup(db.upcast())
        .as_syntax_node()
        .get_text_without_trivia(db.upcast());
    let rrhs_var = rrhs
        .stable_ptr()
        .lookup(db.upcast())
        .as_syntax_node()
        .get_text_without_trivia(db.upcast());
    // Put them in a hashset to check equality without order
    let lhs: HashSet<String> = HashSet::from_iter([llhs_var, rlhs_var]);
    let rhs: HashSet<String> = HashSet::from_iter([lrhs_var, rrhs_var]);
    if lhs != rhs {
        return;
    }

    // TODO: support other expressions like tuples and literals
    let should_return = match (llhs, rlhs) {
        (Expr::Snapshot(llhs), Expr::Snapshot(rlhs)) => {
            matches!(arenas.exprs[llhs.inner], Expr::FunctionCall(_))
                || matches!(arenas.exprs[rlhs.inner], Expr::FunctionCall(_))
        }
        (Expr::Var(_), Expr::Var(_)) => false,
        (Expr::Snapshot(llhs), Expr::Var(_)) => {
            matches!(arenas.exprs[llhs.inner], Expr::FunctionCall(_))
        }
        (Expr::Var(_), Expr::Snapshot(rlhs)) => {
            matches!(arenas.exprs[rlhs.inner], Expr::FunctionCall(_))
        }
        _ => return,
    };
    if should_return {
        return;
    }

    if is_simplifiable_double_comparison(
        &lhs_fn_trait_name,
        &rhs_fn_trait_name,
        &logical_operator_exprs.op,
    ) {
        diagnostics.push(PluginDiagnostic {
            message: SIMPLIFIABLE_COMPARISON.to_string(),
            stable_ptr: logical_operator_exprs.stable_ptr.untyped(),
            severity: Severity::Warning,
        });
    } else if is_redundant_double_comparison(
        &lhs_fn_trait_name,
        &rhs_fn_trait_name,
        &logical_operator_exprs.op,
    ) {
        diagnostics.push(PluginDiagnostic {
            message: REDUNDANT_COMPARISON.to_string(),
            stable_ptr: logical_operator_exprs.stable_ptr.untyped(),
            severity: Severity::Warning,
        });
    } else if is_contradictory_double_comparison(
        &lhs_fn_trait_name,
        &rhs_fn_trait_name,
        &logical_operator_exprs.op,
    ) {
        diagnostics.push(PluginDiagnostic {
            message: CONTRADICTORY_COMPARISON.to_string(),
            stable_ptr: logical_operator_exprs.stable_ptr.untyped(),
            severity: Severity::Error,
        });
    }
}

fn check_impossible_comparison(
    lhs_comparison: &ExprFunctionCall,
    rhs_comparison: &ExprFunctionCall,
    lhs_op: &str,
    rhs_op: &str,
    logical_operator_exprs: &ExprLogicalOperator,
    db: &dyn SemanticGroup,
    arenas: &Arenas,
) -> bool {
    let (lhs_var, lhs_litteral) = match (&lhs_comparison.args[0], &lhs_comparison.args[1]) {
        (ExprFunctionCallArg::Value(l_expr_id), ExprFunctionCallArg::Value(r_expr_id)) => {
            match (&arenas.exprs[*l_expr_id], &arenas.exprs[*r_expr_id]) {
                (Expr::Var(var), Expr::Literal(litteral)) => (var, litteral),
                (Expr::Literal(litteral), Expr::Var(var)) => (var, litteral),
                _ => {
                    return false;
                }
            }
        }
        _ => {
            return false;
        }
    };
    let (rhs_var, rhs_litteral) = match (&rhs_comparison.args[0], &rhs_comparison.args[1]) {
        (ExprFunctionCallArg::Value(l_expr_id), ExprFunctionCallArg::Value(r_expr_id)) => {
            match (&arenas.exprs[*l_expr_id], &arenas.exprs[*r_expr_id]) {
                (Expr::Var(var), Expr::Literal(litteral)) => (var, litteral),
                (Expr::Literal(litteral), Expr::Var(var)) => (var, litteral),
                _ => {
                    return false;
                }
            }
        }
        _ => {
            return false;
        }
    };

    if lhs_var
        .stable_ptr
        .lookup(db.upcast())
        .as_syntax_node()
        .get_text_without_trivia(db.upcast())
        != rhs_var
            .stable_ptr
            .lookup(db.upcast())
            .as_syntax_node()
            .get_text_without_trivia(db.upcast())
    {
        return false;
    }

    match (lhs_op, &logical_operator_exprs.op, rhs_op) {
        (GT, LogicalOperator::AndAnd, LT) => lhs_litteral.value >= rhs_litteral.value,
        (GT, LogicalOperator::AndAnd, LE) => lhs_litteral.value >= rhs_litteral.value,
        (GE, LogicalOperator::AndAnd, LT) => lhs_litteral.value >= rhs_litteral.value,
        (GE, LogicalOperator::AndAnd, LE) => lhs_litteral.value > rhs_litteral.value,
        (LT, LogicalOperator::AndAnd, GT) => lhs_litteral.value <= rhs_litteral.value,
        (LT, LogicalOperator::AndAnd, GE) => lhs_litteral.value <= rhs_litteral.value,
        (LE, LogicalOperator::AndAnd, GT) => lhs_litteral.value <= rhs_litteral.value,
        (LE, LogicalOperator::AndAnd, GE) => lhs_litteral.value < rhs_litteral.value,
        _ => false,
    }
}

fn is_simplifiable_double_comparison(
    lhs_op: &str,
    rhs_op: &str,
    middle_op: &LogicalOperator,
) -> bool {
    matches!(
        (lhs_op, middle_op, rhs_op),
        (LE, LogicalOperator::AndAnd, GE)
            | (GE, LogicalOperator::AndAnd, LE)
            | (LT, LogicalOperator::OrOr, EQ)
            | (EQ, LogicalOperator::OrOr, LT)
            | (GT, LogicalOperator::OrOr, EQ)
            | (EQ, LogicalOperator::OrOr, GT)
    )
}

fn is_redundant_double_comparison(lhs_op: &str, rhs_op: &str, middle_op: &LogicalOperator) -> bool {
    matches!(
        (lhs_op, middle_op, rhs_op),
        (LE, LogicalOperator::OrOr, GE)
            | (GE, LogicalOperator::OrOr, LE)
            | (LT, LogicalOperator::OrOr, GT)
            | (GT, LogicalOperator::OrOr, LT)
    )
}

fn is_contradictory_double_comparison(
    lhs_op: &str,
    rhs_op: &str,
    middle_op: &LogicalOperator,
) -> bool {
    matches!(
        (lhs_op, middle_op, rhs_op),
        (EQ, LogicalOperator::AndAnd, LT)
            | (LT, LogicalOperator::AndAnd, EQ)
            | (EQ, LogicalOperator::AndAnd, GT)
            | (GT, LogicalOperator::AndAnd, EQ)
            | (LT, LogicalOperator::AndAnd, GT)
            | (GT, LogicalOperator::AndAnd, LT)
            | (GT, LogicalOperator::AndAnd, GE)
            | (LE, LogicalOperator::AndAnd, GT)
    )
}

/// Rewrites a double comparison. Ex: `a > b || a == b` to `a >= b`
pub fn fix_double_comparison(
    db: &dyn SyntaxGroup,
    node: SyntaxNode,
) -> Option<(SyntaxNode, String)> {
    let expr = AstExpr::from_syntax_node(db, node.clone());

    if let AstExpr::Binary(binary_op) = expr {
        let lhs = binary_op.lhs(db);
        let rhs = binary_op.rhs(db);
        let middle_op = binary_op.op(db);

        if let (Some(lhs_op), Some(rhs_op)) = (
            extract_binary_operator_expr(&lhs, db),
            extract_binary_operator_expr(&rhs, db),
        ) {
            let simplified_op = determine_simplified_operator(&lhs_op, &rhs_op, &middle_op);

            if let Some(simplified_op) = simplified_op {
                if let Some(operator_to_replace) = operator_to_replace(lhs_op) {
                    let lhs_text = lhs
                        .as_syntax_node()
                        .get_text(db)
                        .replace(operator_to_replace, simplified_op);
                    return Some((node, lhs_text.to_string()));
                }
            }
        }
    }

    None
}

fn operator_to_replace(lhs_op: BinaryOperator) -> Option<&'static str> {
    match lhs_op {
        BinaryOperator::EqEq(_) => Some("=="),
        BinaryOperator::GT(_) => Some(">"),
        BinaryOperator::LT(_) => Some("<"),
        BinaryOperator::GE(_) => Some(">="),
        BinaryOperator::LE(_) => Some("<="),
        _ => None,
    }
}

fn determine_simplified_operator(
    lhs_op: &BinaryOperator,
    rhs_op: &BinaryOperator,
    middle_op: &BinaryOperator,
) -> Option<&'static str> {
    match (lhs_op, middle_op, rhs_op) {
        (BinaryOperator::LE(_), BinaryOperator::AndAnd(_), BinaryOperator::GE(_))
        | (BinaryOperator::GE(_), BinaryOperator::AndAnd(_), BinaryOperator::LE(_)) => Some("=="),

        (BinaryOperator::LT(_), BinaryOperator::OrOr(_), BinaryOperator::EqEq(_))
        | (BinaryOperator::EqEq(_), BinaryOperator::OrOr(_), BinaryOperator::LT(_)) => Some("<="),

        (BinaryOperator::GT(_), BinaryOperator::OrOr(_), BinaryOperator::EqEq(_))
        | (BinaryOperator::EqEq(_), BinaryOperator::OrOr(_), BinaryOperator::GT(_)) => Some(">="),

        (BinaryOperator::LT(_), BinaryOperator::OrOr(_), BinaryOperator::GT(_))
        | (BinaryOperator::GT(_), BinaryOperator::OrOr(_), BinaryOperator::LT(_)) => Some("!="),

        _ => None,
    }
}

fn extract_binary_operator_expr(expr: &AstExpr, db: &dyn SyntaxGroup) -> Option<BinaryOperator> {
    if let AstExpr::Binary(binary_op) = expr {
        Some(binary_op.op(db))
    } else {
        None
    }
}
