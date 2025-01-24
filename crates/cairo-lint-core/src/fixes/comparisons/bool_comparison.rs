use cairo_lang_syntax::node::{
    ast::ExprBinary, db::SyntaxGroup, kind::SyntaxKind, SyntaxNode, TypedSyntaxNode,
};

/// Rewrites a bool comparison to a simple bool. Ex: `some_bool == false` would be rewritten to
/// `!some_bool`
pub fn fix_bool_comparison(db: &dyn SyntaxGroup, node: SyntaxNode) -> Option<(SyntaxNode, String)> {
    let node = ExprBinary::from_syntax_node(db, node);
    let lhs = node.lhs(db).as_syntax_node().get_text(db);
    let rhs = node.rhs(db).as_syntax_node().get_text(db);

    let result = generate_fixed_text_for_comparison(db, lhs.as_str(), rhs.as_str(), node.clone());
    Some((node.as_syntax_node(), result))
}

/// Generates the fixed boolean for a boolean comparison. It will transform `x == false` to `!x`
fn generate_fixed_text_for_comparison(
    db: &dyn SyntaxGroup,
    lhs: &str,
    rhs: &str,
    node: ExprBinary,
) -> String {
    let op_kind = node.op(db).as_syntax_node().kind(db);
    let lhs = lhs.trim();
    let rhs = rhs.trim();

    match (lhs, rhs, op_kind) {
        // lhs
        ("false", _, SyntaxKind::TerminalEqEq | SyntaxKind::TokenEqEq) => format!("!{} ", rhs),
        ("true", _, SyntaxKind::TerminalEqEq | SyntaxKind::TokenEqEq) => format!("{} ", rhs),
        ("false", _, SyntaxKind::TerminalNeq) => format!("!{} ", rhs),
        ("true", _, SyntaxKind::TerminalNeq) => format!("!{} ", rhs),

        // rhs
        (_, "false", SyntaxKind::TerminalEqEq | SyntaxKind::TokenEqEq) => format!("!{} ", lhs),
        (_, "true", SyntaxKind::TerminalEqEq | SyntaxKind::TokenEqEq) => format!("{} ", lhs),
        (_, "false", SyntaxKind::TerminalNeq) => format!("!{} ", lhs),
        (_, "true", SyntaxKind::TerminalNeq) => format!("!{} ", lhs),

        _ => node.as_syntax_node().get_text(db).to_string(),
    }
}
