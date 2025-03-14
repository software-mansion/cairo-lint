use cairo_lang_defs::ids::{ModuleItemId, TopLevelLanguageElementId};
use cairo_lang_defs::plugin::PluginDiagnostic;
use cairo_lang_diagnostics::Severity;
use cairo_lang_semantic::db::SemanticGroup;
use cairo_lang_semantic::{
    Arenas, Expr, ExprBlock, ExprId, ExprLoop, ExprMatch, Pattern, PatternEnumVariant, Statement,
};
use cairo_lang_syntax::node::db::SyntaxGroup;
use cairo_lang_syntax::node::SyntaxNode;
use cairo_lang_syntax::node::{
    ast::{
        Expr as AstExpr, ExprLoop as AstExprLoop, OptionPatternEnumInnerPattern,
        Pattern as AstPattern, Statement as AstStatement,
    },
    TypedStablePtr, TypedSyntaxNode,
};
use if_chain::if_chain;

use crate::context::{CairoLintKind, Lint};
use crate::helper::indent_snippet;
use crate::lints::{NONE, SOME};
use crate::queries::{get_all_function_bodies, get_all_loop_expressions};

const SPAN_MATCH_POP_FRONT: &str = "\"SpanImpl::pop_front\"";

pub struct LoopMatchPopFront;

/// ## What it does
///
/// Checks for loops that are used to iterate over a span using `pop_front`.
///
/// ## Example
///
/// ```cairo
/// let a: Span<u32> = array![1, 2, 3].span();
/// loop {
///     match a.pop_front() {
///         Option::Some(val) => {do_smth(val); },
///         Option::None => { break; }
///     }
/// }
/// ```
///
/// Which can be rewritten as
///
/// ```cairo
/// let a: Span<u32> = array![1, 2, 3].span();
/// for val in a {
///     do_smth(val);
/// }
/// ```
impl Lint for LoopMatchPopFront {
    fn allowed_name(&self) -> &'static str {
        "loop_match_pop_front"
    }

    fn diagnostic_message(&self) -> &'static str {
        "you seem to be trying to use `loop` for iterating over a span. Consider using `for in`"
    }

    fn kind(&self) -> CairoLintKind {
        CairoLintKind::LoopMatchPopFront
    }

    fn has_fixer(&self) -> bool {
        true
    }

    fn fix(&self, db: &dyn SyntaxGroup, node: SyntaxNode) -> Option<(SyntaxNode, String)> {
        fix_loop_match_pop_front(db, node)
    }
}

pub fn check_loop_match_pop_front(
    db: &dyn SemanticGroup,
    item: &ModuleItemId,
    diagnostics: &mut Vec<PluginDiagnostic>,
) {
    let function_bodies = get_all_function_bodies(db, item);
    for function_body in function_bodies.iter() {
        let loop_exprs = get_all_loop_expressions(function_body);
        let arenas = &function_body.arenas;
        for loop_expr in loop_exprs.iter() {
            check_single_loop_match_pop_front(db, loop_expr, diagnostics, arenas);
        }
    }
}

fn check_single_loop_match_pop_front(
    db: &dyn SemanticGroup,
    loop_expr: &ExprLoop,
    diagnostics: &mut Vec<PluginDiagnostic>,
    arenas: &Arenas,
) {
    // Checks that the loop doesn't return anything
    if !loop_expr.ty.is_unit(db) {
        return;
    }
    let Expr::Block(expr_block) = &arenas.exprs[loop_expr.body] else {
        return;
    };

    // Case where there's no statements only an expression in the tail.
    if_chain! {
        if expr_block.statements.is_empty();
        if let Some(tail) = &expr_block.tail;
        // Get the function call and check that it's the span match pop front function from the corelib
        if let Expr::Match(expr_match) = &arenas.exprs[*tail];
        if let Expr::FunctionCall(func_call) = &arenas.exprs[expr_match.matched_expr];
        if func_call.function.name(db) == SPAN_MATCH_POP_FRONT;
        then {
            // Check that something is done only in the Some branch of the match
            if !check_single_match(db, expr_match, arenas) {
                return;
            }
            diagnostics.push(PluginDiagnostic {
                stable_ptr: loop_expr.stable_ptr.into(),
                message: LoopMatchPopFront.diagnostic_message().to_owned(),
                severity: Severity::Warning,
            });
            return;
        }
    }

    // If the loop contains multiple statements.
    if_chain! {
        if !expr_block.statements.is_empty();
        // If the first statement is the match we're looking for. the order is important
        if let Statement::Expr(stmt_expr) = &arenas.statements[expr_block.statements[0]];
        if let Expr::Match(expr_match) = &arenas.exprs[stmt_expr.expr];
        then {
            // Checks that we're only doing something in the some branch
            if !check_single_match(db, expr_match, arenas) {
                return;
            }
            let Expr::FunctionCall(func_call) = &arenas.exprs[expr_match.matched_expr] else {
                return;
            };
            if func_call.function.name(db) == SPAN_MATCH_POP_FRONT {
                diagnostics.push(PluginDiagnostic {
                    stable_ptr: loop_expr.stable_ptr.into(),
                    message: LoopMatchPopFront.diagnostic_message().to_owned(),
                    severity: Severity::Warning,
                })
            }
        }
    }
}

const OPTION_TYPE: &str = "core::option::Option::<";

fn check_single_match(db: &dyn SemanticGroup, match_expr: &ExprMatch, arenas: &Arenas) -> bool {
    let arms = &match_expr.arms;

    // Check that we're in a setup with 2 arms that return unit
    if arms.len() == 2 && match_expr.ty.is_unit(db) {
        let first_arm = &arms[0];
        let second_arm = &arms[1];
        let is_first_arm_correct = if let Some(pattern) = first_arm.patterns.first() {
            match &arenas.patterns[*pattern] {
                // If the first arm is `_ => smth` it's incorrect
                Pattern::Otherwise(_) => false,
                // Check if the variant is of type option and if it's `None` checks that it only contains `{ break; }`
                // without comments`
                Pattern::EnumVariant(enum_pat) => {
                    check_enum_pattern(db, enum_pat, arenas, first_arm.expression)
                }
                _ => false,
            }
        } else {
            false
        };
        let is_second_arm_correct = if let Some(pattern) = second_arm.patterns.first() {
            match &arenas.patterns[*pattern] {
                // If the 2nd arm is `_ => smth`, checks that smth is `{ break; }`
                Pattern::Otherwise(_) => {
                    if let Expr::Block(expr_block) = &arenas.exprs[second_arm.expression] {
                        check_block_is_break(db, expr_block, arenas)
                    } else {
                        return false;
                    }
                }
                // Check if the variant is of type option and if it's `None` checks that it only contains `{ break; }`
                // without comments`
                Pattern::EnumVariant(enum_pat) => {
                    check_enum_pattern(db, enum_pat, arenas, second_arm.expression)
                }
                _ => false,
            }
        } else {
            false
        };
        is_first_arm_correct && is_second_arm_correct
    } else {
        false
    }
}
fn check_enum_pattern(
    db: &dyn SemanticGroup,
    enum_pat: &PatternEnumVariant,
    arenas: &Arenas,
    arm_expression: ExprId,
) -> bool {
    // Checks that the variant is from the option type.
    if !enum_pat.ty.format(db.upcast()).starts_with(OPTION_TYPE) {
        return false;
    }

    // Check if the variant is the None variant
    if_chain! {
        if enum_pat.variant.id.full_path(db.upcast()) == NONE;
        // Get the expression of the None variant and checks if it's a block expression.
        if let Expr::Block(expr_block) = &arenas.exprs[arm_expression];
        // If it's a block expression checks that it only contains `break;`
        if check_block_is_break(db, expr_block, arenas);
      then {
          return true;
      }
    }
    enum_pat.variant.id.full_path(db.upcast()) == SOME
}
/// Checks that the block only contains `break;` without comments
fn check_block_is_break(db: &dyn SemanticGroup, expr_block: &ExprBlock, arenas: &Arenas) -> bool {
    if_chain! {
        if expr_block.statements.len() == 1;
        if let Statement::Break(break_stmt) = &arenas.statements[expr_block.statements[0]];
        then {
            let break_node = break_stmt.stable_ptr.lookup(db.upcast()).as_syntax_node();
            // Checks that the trimmed text == the text without trivia which would mean that there is no comment
            let break_text = break_node.get_text(db.upcast()).trim().to_string();
            if break_text == break_node.get_text_without_trivia(db.upcast())
                && (break_text == "break;" || break_text == "break ();")
            {
                return true;
            }
        }
    }
    false
}

/// Rewrites this:
///
/// ```ignore
/// loop {
///     match some_span.pop_front() {
///         Option::Some(val) => do_smth(val),
///         Option::None => break;
///     }
/// }
/// ```
/// to this:
/// ```ignore
/// for val in span {
///     do_smth(val);
/// };
/// ```
pub fn fix_loop_match_pop_front(
    db: &dyn SyntaxGroup,
    node: SyntaxNode,
) -> Option<(SyntaxNode, String)> {
    let expr_loop = AstExprLoop::from_syntax_node(db, node.clone());
    let body = expr_loop.body(db);
    let AstStatement::Expr(expr) = &body.statements(db).elements(db)[0] else {
        panic!(
            "Wrong statement type. This is probably a bug in the lint detection. Please report it"
        )
    };
    let AstExpr::Match(expr_match) = expr.expr(db) else {
        panic!(
            "Wrong expression type. This is probably a bug in the lint detection. Please report it"
        )
    };
    let val = expr_match.expr(db);
    let span_name = match val {
        AstExpr::FunctionCall(func_call) => func_call.arguments(db).arguments(db).elements(db)[0]
            .arg_clause(db)
            .as_syntax_node()
            .get_text(db),
        AstExpr::Binary(dot_call) => dot_call.lhs(db).as_syntax_node().get_text(db),
        _ => panic!(
            "Wrong expression type. This is probably a bug in the lint detection. Please report it"
        ),
    };
    let mut elt_name = "".to_owned();
    let mut some_arm = "".to_owned();
    let arms = expr_match.arms(db).elements(db);

    let mut loop_span = node.span(db);
    loop_span.end = node.span_start_without_trivia(db);
    let indent = node
        .get_text(db)
        .chars()
        .take_while(|c| c.is_whitespace())
        .collect::<String>();
    let trivia = node
        .clone()
        .get_text_of_span(db, loop_span)
        .trim()
        .to_string();
    let trivia = if trivia.is_empty() {
        trivia
    } else {
        format!("{indent}{trivia}\n")
    };
    for arm in arms {
        if_chain! {
            if let AstPattern::Enum(enum_pattern) = &arm.patterns(db).elements(db)[0];
            if let OptionPatternEnumInnerPattern::PatternEnumInnerPattern(var) = enum_pattern.pattern(db);
            then {
                elt_name = var.pattern(db).as_syntax_node().get_text(db);
                some_arm = if let AstExpr::Block(block_expr) = arm.expression(db) {
                    block_expr.statements(db).as_syntax_node().get_text(db)
                } else {
                    arm.expression(db).as_syntax_node().get_text(db)
                }
            }
        }
    }
    Some((
        node,
        indent_snippet(
            &format!("{trivia}for {elt_name} in {span_name} {{\n{some_arm}\n}};\n"),
            indent.len() / 4,
        ),
    ))
}
