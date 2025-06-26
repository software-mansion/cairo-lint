use cairo_lang_defs::ids::TopLevelLanguageElementId;
use cairo_lang_semantic::db::SemanticGroup;
use cairo_lang_semantic::{
    Arenas, Condition, Expr, ExprIf, FixedSizeArrayItems, Pattern, Statement, VarId,
};
use cairo_lang_syntax::node::ast::{
    BlockOrIf, Condition as AstCondition, Expr as AstExpr, ExprIf as AstExprIf,
    ExprMatch as AstExprMatch, OptionElseClause, Statement as AstStatement,
};
use cairo_lang_syntax::node::db::SyntaxGroup;
use cairo_lang_syntax::node::kind::SyntaxKind;
use cairo_lang_syntax::node::{SyntaxNode, TypedSyntaxNode};
use if_chain::if_chain;
use num_bigint::BigInt;

use super::is_expected_variant;
use crate::lints::{function_trait_name_from_fn_id, ARRAY_NEW, DEFAULT, FALSE, NEVER};

/// Checks if the input statement is a `FunctionCall` then checks if the function name is the
/// expected function name
pub fn is_expected_function(expr: &Expr, db: &dyn SemanticGroup, func_name: &str) -> bool {
    let Expr::FunctionCall(func_call) = expr else {
        return false;
    };
    func_call.function.full_path(db).as_str() == func_name
}

/// Checks if the inner_pattern in the input `Pattern::Enum` matches the given argument name.
///
/// # Arguments
/// * `pattern` - The pattern to check.
/// * `db` - Reference to the `SyntaxGroup` for syntax tree access.
/// * `arg_name` - The target name.
///
/// # Returns
/// * `true` if the argument name matches, otherwise `false`.
pub fn pattern_check_enum_arg(pattern: &Pattern, arg: &VarId, arenas: &Arenas) -> bool {
    let Pattern::EnumVariant(enum_var_pattern) = pattern else {
        return false;
    };
    let Some(inner_pattern) = enum_var_pattern.inner_pattern else {
        return false;
    };
    let Pattern::Variable(enum_destruct_var) = &arenas.patterns[inner_pattern] else {
        return false;
    };
    let VarId::Local(expected_var) = arg else {
        return false;
    };
    expected_var == &enum_destruct_var.var.id
}

/// Checks if the enum variant in the expression has the expected name and if the destructured
/// variable in the pattern is used within the expression.
///
/// This function validates two conditions for a given enum pattern and expression:
/// 1. The enum variant within the `expr` matches the `enum_name` provided.
/// 2. The destructured variable from `pattern` is used in `expr`.
///
/// # Arguments
///
/// * `expr` - A reference to an `Expr` representing the expression to check.
/// * `pattern` - A reference to a `Pattern` representing the pattern to match against.
/// * `db` - A reference to a trait object of `SemanticGroup` used for semantic analysis.
/// * `arenas` - A reference to an `Arenas` struct that provides access to allocated patterns and
///   expressions.
/// * `enum_name` - A string slice representing the expected enum variant's full path name.
///
/// # Returns
///
/// Returns `true` if:
/// - `pattern` is an enum variant pattern that matches `expr` and
/// - the destructured variable from `pattern` is used in `expr`.
///
/// Returns `false` otherwise.
///
/// # Example
///
/// Here `x` is destructured in the enum pattern and is used in the `Option::Some(x)` expression
/// ```ignore
/// match res_val {
///     Result::Ok(x) => Option::Some(x),
///     Result::Err(_) => Option::None,
/// };
/// ```
pub fn is_destructured_variable_used_and_expected_variant(
    expr: &Expr,
    pattern: &Pattern,
    db: &dyn SemanticGroup,
    arenas: &Arenas,
    enum_name: &str,
) -> bool {
    let Expr::EnumVariantCtor(enum_expr) = expr else {
        return false;
    };
    if enum_expr.variant.id.full_path(db.upcast()) != enum_name {
        return false;
    };
    let Expr::Var(return_enum_var) = &arenas.exprs[enum_expr.value_expr] else {
        return false;
    };
    pattern_check_enum_arg(pattern, &return_enum_var.var, arenas)
}

/// Checks if the inner pattern of a conditional `if` expression's pattern matches a block
/// statement that returns a variable associated with the destructured variable in the pattern.
///
/// This function validates whether an `if` expression contains a destructured pattern that
/// follows through the `if` block, ensuring that:
/// 1. The `if` condition pattern is an enum variant pattern with a variable as its inner pattern.
/// 2. The block within the `if` statement has a tail expression returning the same variable as the
///    one destructured in the pattern.
///
/// # Arguments
///
/// * `expr` - A reference to an `ExprIf` representing the conditional `if` expression to check.
/// * `arenas` - A reference to an `Arenas` struct that provides access to allocated patterns and
///   expressions for detailed analysis.
///
/// # Returns
///
/// Returns `true` if:
/// - The `if` condition is an enum variant pattern with an inner variable pattern.
/// - The `if` block contains a tail expression that returns the destructured variable.
///
/// Returns `false` otherwise, indicating the pattern does not match.
pub fn if_expr_pattern_matches_tail_var(expr: &ExprIf, arenas: &Arenas) -> bool {
    // Checks if it's an `if-let`
    if_chain! {
        if let Condition::Let(_condition_let, patterns) = &expr.condition;
        // Checks if the pattern is an Enum pattern
        if let Pattern::EnumVariant(enum_pattern) = &arenas.patterns[patterns[0]];
        // Checks if the enum pattern has an inner pattern
        if let Some(inner_pattern) = enum_pattern.inner_pattern;
        // Checks if the pattern is a variable
        if let Pattern::Variable(destruct_var) = &arenas.patterns[inner_pattern];
        then {
            let Expr::Block(if_block) = &arenas.exprs[expr.if_block] else { return false };
            let Some(tail_expr) = if_block.tail else { return false };
            // Checks that the tail expression of the block is a variable.
            let Expr::Var(return_var) = &arenas.exprs[tail_expr] else { return false };
            // Checks that it's a local variable (defined in this scope)
            let VarId::Local(local_return_var) = return_var.var else { return false };
            // Checks that it's the exact variable that was created in the enum pattern
            return destruct_var.var.id == local_return_var;
        }
    }
    false
}

/// Checks if the condition pattern in an `if` expression contains an enum variant pattern
/// that matches an enum variant in the `if` block's tail expression.
///
/// This function verifies two conditions:
/// 1. The condition of the `ExprIf` expression (`expr`) contains an enum variant pattern with an
///    inner pattern.
/// 2. The tail expression in the `if` block matches the same inner pattern and corresponds to the
///    specified `enum_name`.
///
/// # Arguments
///
/// * `expr` - The `ExprIf` expression containing the enum variant pattern and the `if` block to
///   check.
/// * `db` - A reference to the `SemanticGroup`, which provides access to the syntax tree.
/// * `arenas` - A reference to the `Arenas` structure, used for accessing allocated expressions and
///   patterns.
/// * `enum_name` - The expected enum variant name to match within the `if` block's statement.
///
/// # Returns
///
/// * `true` if the inner pattern in the enum variant condition matches the first argument of the
///   enum variant with `enum_name` in the tail expression of the `if` block; otherwise, `false`.
///
/// # Example
///
/// ```ignore
/// if let EnumVariant(x) = condition {
///     EnumName(x)
/// }
/// ```
/// Checks if `x` in the condition matches `x` in the `if` block's enum pattern.
pub fn if_expr_condition_and_block_match_enum_pattern(
    expr: &ExprIf,
    db: &dyn SemanticGroup,
    arenas: &Arenas,
    enum_name: &str,
) -> bool {
    if_chain! {
        if let Condition::Let(_expr_id, patterns) = &expr.condition;
        if let Pattern::EnumVariant(enum_pattern) = &arenas.patterns[patterns[0]];
        if let Some(inner_pattern) = enum_pattern.inner_pattern;
        if let Pattern::Variable(variable_pattern) = &arenas.patterns[inner_pattern];
        if let Expr::Block(if_block) = &arenas.exprs[expr.if_block];
        if let Some(tail_expr_id) = if_block.tail;
        if let Expr::EnumVariantCtor(enum_var) = &arenas.exprs[tail_expr_id];
        if is_expected_variant(&arenas.exprs[tail_expr_id], db, enum_name);
        if let Expr::Var(var) = &arenas.exprs[enum_var.value_expr];
        if let VarId::Local(return_var) = var.var;
        then {
            return return_var == variable_pattern.var.id;
        }
    }
    false
}

/// Checks if the input `Expr` is a default of the expr kind.
///
/// # Arguments
/// * `db` - Reference to the `SyntaxGroup` for syntax tree access.
/// * `expr` - The target expr.
///
/// # Returns
/// * `true` if the expression is a default otherwise `false`.
pub fn check_is_default(db: &dyn SemanticGroup, expr: &Expr, arenas: &Arenas) -> bool {
    match expr {
        Expr::FunctionCall(func_call) => {
            // Checks if the function called is either default or array new.
            let trait_name = function_trait_name_from_fn_id(db, &func_call.function);
            trait_name == DEFAULT || trait_name == ARRAY_NEW
        }
        // Empty string literal
        Expr::StringLiteral(expr_str) => expr_str.value.is_empty(),
        // If we're in a block checks that it returns default and does nothing else
        Expr::Block(expr_block) => {
            // Checks that if there is a statement in the block it's to set a variable that will be returned in
            // the tail and nothing else
            let default_subscope = if expr_block.statements.len() == 1 {
                // Check for a let assignment
                let Statement::Let(stmt) = &arenas.statements[expr_block.statements[0]] else {
                    return false;
                };
                let Pattern::Variable(assigned_variable) = &arenas.patterns[stmt.pattern] else {
                    return false;
                };

                // Checks that the tail contains a variable that is exactly the one created in the statements
                let Some(tail) = expr_block.tail else {
                    return false;
                };
                let Expr::Var(return_var) = &arenas.exprs[tail] else {
                    return false;
                };
                let VarId::Local(tail_var) = return_var.var else {
                    return false;
                };

                // Checks that the value assigned in the variable is a default value
                check_is_default(db, &arenas.exprs[stmt.expr], arenas)
                    && tail_var == assigned_variable.var.id
            } else {
                false
            };
            let Some(tail_expr_id) = expr_block.tail else {
                return false;
            };
            default_subscope
                || (check_is_default(db, &arenas.exprs[tail_expr_id], arenas)
                    && expr_block.statements.is_empty())
        }
        Expr::FixedSizeArray(expr_arr) => match &expr_arr.items {
            // Case where the array is defined like that [0_u32; N]
            FixedSizeArrayItems::ValueAndSize(expr_id, _) => {
                check_is_default(db, &arenas.exprs[*expr_id], arenas)
            }
            // Case where the array is defined like that [0_u32, 0, 0, ...]
            FixedSizeArrayItems::Items(expr_ids) => expr_ids
                .iter()
                .all(|&expr| check_is_default(db, &arenas.exprs[expr], arenas)),
        },
        // Literal integer
        Expr::Literal(expr_literal) => expr_literal.value == BigInt::ZERO,
        // Boolean false
        Expr::EnumVariantCtor(enum_variant) => {
            enum_variant.variant.id.full_path(db.upcast()) == FALSE
        }
        // Tuple contains only default elements
        Expr::Tuple(expr_tuple) => expr_tuple
            .items
            .iter()
            .all(|&expr| check_is_default(db, &arenas.exprs[expr], arenas)),
        _ => false,
    }
}

pub fn fix_manual(func_name: &str, db: &dyn SyntaxGroup, node: SyntaxNode) -> String {
    match node.kind(db) {
        SyntaxKind::ExprMatch => {
            let expr_match = AstExprMatch::from_syntax_node(db, node);

            let option_var_name = expr_match.expr(db).as_syntax_node().get_text(db);

            format!("{}.{func_name}()", option_var_name.trim_end())
        }
        SyntaxKind::ExprIf => {
            let expr_if = AstExprIf::from_syntax_node(db, node);

            let var_name = if let AstCondition::Let(condition_let) = expr_if.condition(db) {
                condition_let.expr(db).as_syntax_node().get_text(db)
            } else {
                panic!("Expected an ConditionLet condition")
            };

            format!("{}.{func_name}()", var_name.trim_end())
        }
        _ => panic!("SyntaxKind should be either ExprIf or ExprMatch"),
    }
}

pub fn expr_match_get_var_name_and_err(
    expr_match: AstExprMatch,
    db: &dyn SyntaxGroup,
    arm_index: usize,
) -> (String, String) {
    let option_var_name = expr_match.expr(db).as_syntax_node().get_text(db);

    let arms = expr_match.arms(db).elements(db);
    if arms.len() != 2 {
        panic!("Expected exactly two arms in the match expression");
    }

    if arm_index > 1 {
        panic!("Invalid arm index. Expected 0 for first arm or 1 for second arm.");
    }

    let args = match &arms[arm_index].expression(db) {
        AstExpr::FunctionCall(func_call) => func_call.arguments(db).arguments(db).elements(db),
        AstExpr::Block(block) => {
            if block.statements(db).elements(db).len() != 1 {
                panic!("Expected a single statement in the block");
            }

            let AstStatement::Expr(statement_expr) = &block.statements(db).elements(db)[0] else {
                panic!("Expected an expression statement in the block");
            };

            let AstExpr::FunctionCall(func_call) = statement_expr.expr(db) else {
                panic!("Expected a function call expression in the block");
            };

            func_call.arguments(db).arguments(db).elements(db)
        }
        _ => panic!("Expected a function call or block expression"),
    };

    let arg = args.first().expect("Should have arg");

    let none_arm_err = arg.as_syntax_node().get_text(db).to_string();

    (option_var_name, none_arm_err)
}

pub fn expr_if_get_var_name_and_err(expr_if: AstExprIf, db: &dyn SyntaxGroup) -> (String, String) {
    let AstCondition::Let(condition_let) = expr_if.condition(db) else {
        panic!("Expected a ConditionLet condition");
    };
    let option_var_name = condition_let.expr(db).as_syntax_node().get_text(db);

    let OptionElseClause::ElseClause(else_clause) = expr_if.else_clause(db) else {
        panic!("Expected a non-empty else clause");
    };

    let BlockOrIf::Block(expr_block) = else_clause.else_block_or_if(db) else {
        panic!("Expected a BlockOrIf block in else clause");
    };

    let AstStatement::Expr(statement_expr) = expr_block.statements(db).elements(db)[0].clone()
    else {
        panic!("Expected a StatementExpr statement");
    };

    let AstExpr::FunctionCall(func_call) = statement_expr.expr(db) else {
        panic!("Expected a function call expression");
    };

    let args = func_call.arguments(db).arguments(db).elements(db);
    let arg = args.first().expect("Should have arg");
    let err = arg.as_syntax_node().get_text(db).to_string();

    (option_var_name, err)
}

/// Returns true if the expression is a function call (or a block whose tail is a function call)
/// and the function's return type is the NEVER type.
pub fn func_call_or_block_returns_never(
    expr: &Expr,
    db: &dyn SemanticGroup,
    arenas: &Arenas,
) -> bool {
    let function_call = match expr {
        // If it is block, it is necessary to extract tail from it
        Expr::Block(expr_block) => {
            let Some(tail_expr_id) = expr_block.tail else {
                return false;
            };
            let Expr::FunctionCall(function_call) = &arenas.exprs[tail_expr_id] else {
                return false;
            };
            function_call
        }
        Expr::FunctionCall(function_call) => function_call,
        _ => return false,
    };

    function_call.ty.short_name(db) == NEVER
}

/// Checks if a match arm directly returns the variable extracted from an enum variant
pub fn match_arm_returns_extracted_var(expr: &Expr, pattern: &Pattern, arenas: &Arenas) -> bool {
    let Expr::Var(enum_destruct_var) = expr else {
        return false;
    };

    pattern_check_enum_arg(pattern, &enum_destruct_var.var, arenas)
}

/// Returns the tail expression from a block if it's the only content, otherwise returns the original expression.
/// If the block contains statements, it returns the block itself.
pub fn extract_tail_or_preserve_expr<'a>(expr: &'a Expr, arenas: &'a Arenas) -> &'a Expr {
    if_chain! {
        if let Expr::Block(expr_block) = expr;
        if expr_block.statements.is_empty();
        if let Some(tail_expr_id) = expr_block.tail;
        then {
            return &arenas.exprs[tail_expr_id]
        }
    }

    expr
}
