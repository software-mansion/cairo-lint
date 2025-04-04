use cairo_lang_defs::ids::{ModuleItemId, TopLevelLanguageElementId};
use cairo_lang_defs::plugin::PluginDiagnostic;
use cairo_lang_diagnostics::Severity;
use cairo_lang_semantic::db::SemanticGroup;
use cairo_lang_semantic::{Arenas, Expr, ExprFunctionCall, ExprFunctionCallArg};
use cairo_lang_syntax::node::db::SyntaxGroup;
use cairo_lang_syntax::node::kind::SyntaxKind;
use cairo_lang_syntax::node::SyntaxNode;
use cairo_lang_syntax::node::{ast::ExprBinary, TypedStablePtr, TypedSyntaxNode};
use if_chain::if_chain;

use crate::context::{CairoLintKind, Lint};
use crate::queries::{get_all_function_bodies, get_all_function_calls};

pub struct BoolComparison;

/// ## What it does
///
/// Checks for direct variable with boolean literal like `a == true` or `a == false`.
///
/// ## Example
///
/// ```cairo
/// fn main() {
///     let x = true;
///     if x == true {
///         println!("x is true");
///     }
/// }
/// ```
///
/// Can be rewritten as:
///
/// ```cairo
/// fn main() {
///    let x = true;
///    if x {
///        println!("x is true");
///    }
/// }
/// ```
impl Lint for BoolComparison {
    fn allowed_name(&self) -> &'static str {
        "bool_comparison"
    }

    fn diagnostic_message(&self) -> &'static str {
        "Unnecessary comparison with a boolean value. Use the variable directly."
    }

    fn kind(&self) -> CairoLintKind {
        CairoLintKind::BoolComparison
    }

    fn has_fixer(&self) -> bool {
        true
    }

    fn fix(&self, db: &dyn SemanticGroup, node: SyntaxNode) -> Option<(SyntaxNode, String)> {
        fix_bool_comparison(db.upcast(), node)
    }
}

/// Checks for ` a == true`. Bool comparisons are useless and can be rewritten more clearly.
pub fn check_bool_comparison(
    db: &dyn SemanticGroup,
    item: &ModuleItemId,
    diagnostics: &mut Vec<PluginDiagnostic>,
) {
    let function_bodies = get_all_function_bodies(db, item);
    for function_body in function_bodies.iter() {
        let function_call_exprs = get_all_function_calls(function_body);
        let arenas = &function_body.arenas;
        for function_call_expr in function_call_exprs {
            check_single_bool_comparison(db, &function_call_expr, arenas, diagnostics);
        }
    }
}

fn check_single_bool_comparison(
    db: &dyn SemanticGroup,
    function_call_expr: &ExprFunctionCall,
    arenas: &Arenas,
    diagnostics: &mut Vec<PluginDiagnostic>,
) {
    // Check if the function call is the bool partial eq function (==).
    if !function_call_expr
        .function
        .full_path(db)
        .contains("core::BoolPartialEq::")
    {
        return;
    }
    // Extract the args of the function call. This function expects snapshots hence we need to
    // destructure that. Also the boolean type in cairo is an enum hence the enum ctor.
    for arg in &function_call_expr.args {
        if_chain! {
            if let ExprFunctionCallArg::Value(expr) = arg;
            if let Expr::Snapshot(snap) = &arenas.exprs[*expr];
            if let Expr::EnumVariantCtor(enum_var) = &arenas.exprs[snap.inner];
            if enum_var.variant.concrete_enum_id.enum_id(db).full_path(db.upcast()) == "core::bool";
            then {
                diagnostics.push(PluginDiagnostic {
                    stable_ptr: function_call_expr.stable_ptr.untyped(),
                    message: BoolComparison.diagnostic_message().to_string(),
                    severity: Severity::Warning,
                });
            }
        }
    }
}

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
        ("false", _, SyntaxKind::TerminalNeq) => format!("{} ", rhs),
        ("true", _, SyntaxKind::TerminalNeq) => format!("!{} ", rhs),

        // rhs
        (_, "false", SyntaxKind::TerminalEqEq | SyntaxKind::TokenEqEq) => format!("!{} ", lhs),
        (_, "true", SyntaxKind::TerminalEqEq | SyntaxKind::TokenEqEq) => format!("{} ", lhs),
        (_, "false", SyntaxKind::TerminalNeq) => format!("{} ", lhs),
        (_, "true", SyntaxKind::TerminalNeq) => format!("!{} ", lhs),

        _ => node.as_syntax_node().get_text(db).to_string(),
    }
}
