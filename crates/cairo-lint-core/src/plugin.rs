use cairo_lang_defs::ids::{FunctionWithBodyId, LanguageElementId, ModuleId, ModuleItemId};
use cairo_lang_defs::plugin::PluginDiagnostic;
use cairo_lang_filesystem::ids::FileLongId;
use cairo_lang_semantic::db::SemanticGroup;
use cairo_lang_semantic::items::attribute::SemanticQueryAttrs;
use cairo_lang_semantic::plugin::{AnalyzerPlugin, PluginSuite};
use cairo_lang_semantic::{Expr, Statement};
use cairo_lang_syntax::node::ast::Expr as AstExpr;
use cairo_lang_syntax::node::kind::SyntaxKind;
use cairo_lang_syntax::node::{TypedStablePtr, TypedSyntaxNode};
use cairo_lang_utils::LookupIntern;

use crate::lints::ifs::*;
use crate::lints::loops::{loop_for_while, loop_match_pop_front};
use crate::lints::manual::*;
use crate::lints::{
    bitwise_for_parity_check, bool_comparison, breaks, double_comparison, double_parens,
    duplicate_underscore_args, eq_op, erasing_op, int_op_one, panic, performance, single_match,
};
use crate::LINT_CONTEXT;

pub fn cairo_lint_plugin_suite() -> PluginSuite {
    let mut suite = PluginSuite::default();
    suite.add_analyzer_plugin::<CairoLint>();
    suite
}

#[derive(Debug, Default)]
pub struct CairoLint {
    include_compiler_generated_files: bool,
}

impl CairoLint {
    pub fn new(include_compiler_generated_files: bool) -> Self {
        Self {
            include_compiler_generated_files,
        }
    }
}

impl AnalyzerPlugin for CairoLint {
    fn declared_allows(&self) -> Vec<String> {
        LINT_CONTEXT
            .get_unique_allowed_names()
            .iter()
            .map(ToString::to_string)
            .collect()
    }

    fn diagnostics(&self, db: &dyn SemanticGroup, module_id: ModuleId) -> Vec<PluginDiagnostic> {
        let mut diags = Vec::new();
        // let syntax_db = db.upcast();
        let Ok(items) = db.module_items(module_id) else {
            return diags;
        };
        for item in &*items {
            // Skip compiler generated files. By default it checks whether the item is inside the virtual or external file.
            if !self.include_compiler_generated_files
                && matches!(
                    item.stable_location(db.upcast())
                        .file_id(db.upcast())
                        .lookup_intern(db),
                    FileLongId::Virtual(_) | FileLongId::External(_)
                )
            {
                continue;
            }

            let function_nodes = match item {
                ModuleItemId::Constant(constant_id) => constant_id
                    .stable_ptr(db.upcast())
                    .lookup(syntax_db)
                    .as_syntax_node(),
                ModuleItemId::FreeFunction(free_function_id) => {
                    let func_id = FunctionWithBodyId::Free(*free_function_id);
                    check_function(db, func_id, &mut diags);
                    free_function_id
                        .stable_ptr(db.upcast())
                        .lookup(syntax_db)
                        .as_syntax_node()
                }
                ModuleItemId::Impl(impl_id) => {
                    let Ok(functions) = db.impl_functions(*impl_id) else {
                        continue;
                    };
                    for (_fn_name, fn_id) in functions.iter() {
                        let func_id = FunctionWithBodyId::Impl(*fn_id);
                        check_function(db, func_id, &mut diags);
                    }
                    impl_id
                        .stable_ptr(db.upcast())
                        .lookup(syntax_db)
                        .as_syntax_node()
                }
                _ => continue,
            }
            .descendants(syntax_db);

            // for node in function_nodes {
            //     match node.kind(syntax_db) {
            //         SyntaxKind::ExprParenthesized => double_parens::check_double_parens(
            //             db.upcast(),
            //             &AstExpr::from_syntax_node(db.upcast(), node.clone()),
            //             &mut diags,
            //         ),
            //         SyntaxKind::ExprIf => {}
            //         SyntaxKind::ExprMatch => {}
            //         _ => continue,
            //     }
            // }
        }
        diags
    }
}
fn check_function(
    db: &dyn SemanticGroup,
    func_id: FunctionWithBodyId,
    diagnostics: &mut Vec<PluginDiagnostic>,
) {
    // if let Ok(false) = func_id.has_attr_with_arg(db, "allow", "duplicate_underscore_args") {
    //     duplicate_underscore_args::check_duplicate_underscore_args(
    //         db.function_with_body_signature(func_id).unwrap().params,
    //         diagnostics,
    //     );
    // }

    let Ok(function_body) = db.function_body(func_id) else {
        return;
    };
    for (_expression_id, expression) in &function_body.arenas.exprs {
        match &expression {
            Expr::Match(expr_match) => {
                single_match::check_single_matches(
                    db,
                    expr_match,
                    diagnostics,
                    &function_body.arenas,
                );
                manual_ok_or::check_manual_ok_or(
                    db,
                    &function_body.arenas,
                    expr_match,
                    diagnostics,
                );
                manual_ok::check_manual_ok(db, &function_body.arenas, expr_match, diagnostics);
                manual_err::check_manual_err(db, &function_body.arenas, expr_match, diagnostics);
                manual_is::check_manual_is(db, &function_body.arenas, expr_match, diagnostics);
                manual_expect::check_manual_expect(
                    db,
                    &function_body.arenas,
                    expr_match,
                    diagnostics,
                );
                manual_expect_err::check_manual_expect_err(
                    db,
                    &function_body.arenas,
                    expr_match,
                    diagnostics,
                );
                manual_unwrap_or_default::check_manual_unwrap_or_default(
                    db,
                    &function_body.arenas,
                    expr_match,
                    diagnostics,
                );
            }
            Expr::Loop(expr_loop) => {
                loop_match_pop_front::check_loop_match_pop_front(
                    db,
                    expr_loop,
                    diagnostics,
                    &function_body.arenas,
                );
                loop_for_while::check_loop_for_while(
                    db,
                    expr_loop,
                    &function_body.arenas,
                    diagnostics,
                );
            }
            Expr::FunctionCall(expr_func) => {
                panic::check_panic_usage(db, expr_func, diagnostics);
                bool_comparison::check_bool_comparison(
                    db,
                    expr_func,
                    &function_body.arenas,
                    diagnostics,
                );
                int_op_one::check_int_op_one(db, expr_func, &function_body.arenas, diagnostics);
                bitwise_for_parity_check::check_bitwise_for_parity(
                    db,
                    expr_func,
                    &function_body.arenas,
                    diagnostics,
                );
                eq_op::check_eq_op(db, expr_func, &function_body.arenas, diagnostics);
                erasing_op::check_erasing_operation(
                    db,
                    expr_func,
                    &function_body.arenas,
                    diagnostics,
                );
            }

            Expr::LogicalOperator(expr_logical) => {
                double_comparison::check_double_comparison(
                    db,
                    expr_logical,
                    &function_body.arenas,
                    diagnostics,
                );
            }
            Expr::If(expr_if) => {
                equatable_if_let::check_equatable_if_let(
                    db,
                    expr_if,
                    &function_body.arenas,
                    diagnostics,
                );
                collapsible_if_else::check_collapsible_if_else(
                    db,
                    expr_if,
                    &function_body.arenas,
                    diagnostics,
                );
                collapsible_if::check_collapsible_if(
                    db,
                    expr_if,
                    &function_body.arenas,
                    diagnostics,
                );
                ifs_same_cond::check_duplicate_if_condition(
                    db,
                    expr_if,
                    &function_body.arenas,
                    diagnostics,
                );
                manual_is::check_manual_if_is(db, &function_body.arenas, expr_if, diagnostics);
                manual_expect::check_manual_if_expect(
                    db,
                    &function_body.arenas,
                    expr_if,
                    diagnostics,
                );
                manual_ok_or::check_manual_if_ok_or(
                    db,
                    &function_body.arenas,
                    expr_if,
                    diagnostics,
                );
                manual_ok::check_manual_if_ok(db, &function_body.arenas, expr_if, diagnostics);
                manual_err::check_manual_if_err(db, &function_body.arenas, expr_if, diagnostics);
                manual_expect_err::check_manual_if_expect_err(
                    db,
                    &function_body.arenas,
                    expr_if,
                    diagnostics,
                );
                manual_unwrap_or_default::check_manual_if_unwrap_or_default(
                    db,
                    &function_body.arenas,
                    expr_if,
                    diagnostics,
                );
            }
            Expr::While(expr_while) => performance::check_inefficient_while_comp(
                db,
                expr_while,
                diagnostics,
                &function_body.arenas,
            ),
            _ => (),
        };
    }
    for (_stmt_id, stmt) in &function_body.arenas.statements {
        if let Statement::Break(stmt_break) = &stmt {
            breaks::check_break(db, stmt_break, &function_body.arenas, diagnostics)
        }
    }
}
