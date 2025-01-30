use cairo_lang_defs::ids::ModuleItemId;
use cairo_lang_defs::plugin::PluginDiagnostic;
use cairo_lang_diagnostics::Severity;
use cairo_lang_semantic::db::SemanticGroup;
use cairo_lang_semantic::{Arenas, Condition, Expr, ExprWhile};

use crate::context::{CairoLintKind, Lint};
use crate::queries::{get_all_function_bodies, get_all_while_expressions};

const INEFFICIENT_WHILE_COMP: &str = "using [`<`, `<=`, `>=`, `>`] exit conditions is inefficient. Consider \
                                              switching to `!=` or using ArrayTrait::multi_pop_front.";
const INEFFICIENT_WHILE_COMP_LINT_NAME: &str = "inefficient_while_comp";

pub struct InefficientWhileComparison;

impl Lint for InefficientWhileComparison {
    fn allowed_name(self: &Self) -> &'static str {
        INEFFICIENT_WHILE_COMP_LINT_NAME
    }

    fn diagnostic_message(self: &Self) -> &'static str {
        INEFFICIENT_WHILE_COMP
    }

    fn kind(self: &Self) -> CairoLintKind {
        CairoLintKind::Performance
    }
}

// Match all types implementing PartialOrd
const PARTIAL_ORD_PATTERNS: [&str; 4] = [
    "PartialOrd::lt\"",
    "PartialOrd::le\"",
    "PartialOrd::gt\"",
    "PartialOrd::ge\"",
];

pub fn check_inefficient_while_comp(
    db: &dyn SemanticGroup,
    item: &ModuleItemId,
    diagnostics: &mut Vec<PluginDiagnostic>,
) {
    let function_bodies = get_all_function_bodies(db, item);
    for function_body in function_bodies.iter() {
        let while_exprs = get_all_while_expressions(function_body);
        let arenas = &function_body.arenas;
        for while_expr in while_exprs.iter() {
            check_single_inefficient_while_comp(db, while_expr, diagnostics, arenas);
        }
    }
}

fn check_single_inefficient_while_comp(
    db: &dyn SemanticGroup,
    while_expr: &ExprWhile,
    diagnostics: &mut Vec<PluginDiagnostic>,
    arenas: &Arenas,
) {
    // It might be a false positive, because there can be cases when:
    //  - The rhs arguments is changed in the loop body
    //  - The lhs argument can "skip" the moment where lhs == rhs
    if let Condition::BoolExpr(expr_cond) = while_expr.condition {
        check_expression(db, &arenas.exprs[expr_cond], diagnostics, arenas);
    }
}

fn check_expression(
    db: &dyn SemanticGroup,
    expr: &Expr,
    diagnostics: &mut Vec<PluginDiagnostic>,
    arenas: &Arenas,
) {
    match expr {
        Expr::FunctionCall(func_call) => {
            let func_name = func_call.function.name(db);
            if PARTIAL_ORD_PATTERNS.iter().any(|p| func_name.ends_with(p)) {
                diagnostics.push(PluginDiagnostic {
                    stable_ptr: func_call.stable_ptr.into(),
                    message: INEFFICIENT_WHILE_COMP.to_owned(),
                    severity: Severity::Warning,
                });
            }
        }
        Expr::LogicalOperator(expr_logical) => {
            check_expression(db, &arenas.exprs[expr_logical.lhs], diagnostics, arenas);
            check_expression(db, &arenas.exprs[expr_logical.rhs], diagnostics, arenas);
        }
        _ => {}
    }
}
