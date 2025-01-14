use cairo_lang_syntax::node::{
    ast::{BlockOrIf, Condition, Expr, ExprIf, ExprMatch, OptionElseClause, Statement},
    db::SyntaxGroup,
    kind::SyntaxKind,
    SyntaxNode, TypedSyntaxNode,
};

pub(crate) fn fix_manual(func_name: &str, db: &dyn SyntaxGroup, node: SyntaxNode) -> String {
    match node.kind(db) {
        SyntaxKind::ExprMatch => {
            let expr_match = ExprMatch::from_syntax_node(db, node.clone());

            let option_var_name = expr_match
                .expr(db)
                .as_syntax_node()
                .get_text_without_trivia(db);

            format!("{option_var_name}.{func_name}()")
        }
        SyntaxKind::ExprIf => {
            let expr_if = ExprIf::from_syntax_node(db, node.clone());

            let var_name = if let Condition::Let(condition_let) = expr_if.condition(db) {
                condition_let
                    .expr(db)
                    .as_syntax_node()
                    .get_text_without_trivia(db)
            } else {
                panic!("Expected an ConditionLet condition")
            };

            format!("{var_name}.{func_name}()")
        }
        _ => panic!("SyntaxKind should be either ExprIf or ExprMatch"),
    }
}

pub(crate) fn expr_match_get_var_name_and_err(
    expr_match: ExprMatch,
    db: &dyn SyntaxGroup,
    arm_index: usize,
) -> (String, String) {
    let option_var_name = expr_match
        .expr(db)
        .as_syntax_node()
        .get_text_without_trivia(db);

    let arms = expr_match.arms(db).elements(db);
    if arms.len() != 2 {
        panic!("Expected exactly two arms in the match expression");
    }

    if arm_index > 1 {
        panic!("Invalid arm index. Expected 0 for first arm or 1 for second arm.");
    }

    let Expr::FunctionCall(func_call) = &arms[arm_index].expression(db) else {
        panic!("Expected a function call expression");
    };

    let args = func_call.arguments(db).arguments(db).elements(db);
    let arg = args.first().expect("Should have arg");

    let none_arm_err = arg.as_syntax_node().get_text_without_trivia(db).to_string();

    (option_var_name, none_arm_err)
}

pub(crate) fn expr_if_get_var_name_and_err(
    expr_if: ExprIf,
    db: &dyn SyntaxGroup,
) -> (String, String) {
    let Condition::Let(condition_let) = expr_if.condition(db) else {
        panic!("Expected a ConditionLet condition");
    };
    let option_var_name = condition_let
        .expr(db)
        .as_syntax_node()
        .get_text_without_trivia(db);

    let OptionElseClause::ElseClause(else_clause) = expr_if.else_clause(db) else {
        panic!("Expected a non-empty else clause");
    };

    let BlockOrIf::Block(expr_block) = else_clause.else_block_or_if(db) else {
        panic!("Expected a BlockOrIf block in else clause");
    };

    let Statement::Expr(statement_expr) = expr_block.statements(db).elements(db)[0].clone() else {
        panic!("Expected a StatementExpr statement");
    };

    let Expr::FunctionCall(func_call) = statement_expr.expr(db) else {
        panic!("Expected a function call expression");
    };

    let args = func_call.arguments(db).arguments(db).elements(db);
    let arg = args.first().expect("Should have arg");
    let err = arg.as_syntax_node().get_text_without_trivia(db).to_string();

    (option_var_name, err)
}
