use cairo_lang_defs::ids::ModuleItemId;
use cairo_lang_defs::plugin::PluginDiagnostic;
use cairo_lang_diagnostics::Severity;
use cairo_lang_semantic::db::SemanticGroup;
use cairo_lang_semantic::{Arenas, StatementBreak};
use cairo_lang_syntax::node::db::SyntaxGroup;
use cairo_lang_syntax::node::{SyntaxNode, TypedStablePtr};
use if_chain::if_chain;

use crate::context::{CairoLintKind, Lint};

use crate::LinterGroup;
use crate::fixer::InternalFix;
use crate::queries::{get_all_break_statements, get_all_function_bodies};

pub struct BreakUnit;

/// ## What it does
///
/// Checks for `break ();` statements and suggests removing the parentheses.
///
/// ## Example
///
/// ```cairo
/// fn main() {
///     loop {
///         break ();
///     }
/// }
/// ```
///
/// Can be fixed by removing the parentheses:
///
/// ```cairo
/// fn main() {
///     loop {
///         break;
///     }
/// }
/// ```
impl Lint for BreakUnit {
    fn allowed_name(&self) -> &'static str {
        "break_unit"
    }

    fn diagnostic_message(&self) -> &'static str {
        "unnecessary double parentheses found after break. Consider removing them."
    }

    fn kind(&self) -> CairoLintKind {
        CairoLintKind::BreakUnit
    }

    fn has_fixer(&self) -> bool {
        true
    }

    fn fix<'db>(
        &self,
        db: &'db dyn LinterGroup,
        node: SyntaxNode<'db>,
    ) -> Option<InternalFix<'db>> {
        fix_break_unit(db, node)
    }

    fn fix_message(&self) -> Option<&'static str> {
        Some("Remove unnecessary parentheses from break")
    }
}

#[tracing::instrument(skip_all, level = "trace")]
pub fn check_break<'db>(
    db: &'db dyn LinterGroup,
    item: &ModuleItemId<'db>,
    diagnostics: &mut Vec<PluginDiagnostic<'db>>,
) {
    let function_bodies = get_all_function_bodies(db, item);
    for function_body in function_bodies.iter() {
        let break_exprs = get_all_break_statements(function_body);
        for break_expr in break_exprs.iter() {
            check_single_break(db, break_expr, &function_body.arenas, diagnostics)
        }
    }
}

fn check_single_break<'db>(
    db: &'db dyn SemanticGroup,
    break_expr: &StatementBreak<'db>,
    arenas: &Arenas<'db>,
    diagnostics: &mut Vec<PluginDiagnostic<'db>>,
) {
    if_chain! {
        if let Some(expr) = break_expr.expr_option;
        if arenas.exprs[expr].ty().is_unit(db);
        then {
            diagnostics.push(PluginDiagnostic {
                stable_ptr: break_expr.stable_ptr.untyped(),
                message: BreakUnit.diagnostic_message().to_string(),
                severity: Severity::Warning,
                inner_span: None
            });
        }
    }
}

/// Rewrites `break ();` as `break;` given the node text contains it.
#[tracing::instrument(skip_all, level = "trace")]
pub fn fix_break_unit<'db>(
    db: &'db dyn SyntaxGroup,
    node: SyntaxNode<'db>,
) -> Option<InternalFix<'db>> {
    Some(InternalFix {
        node,
        suggestion: node.get_text(db).replace("break ();", "break;").to_string(),
        description: BreakUnit.fix_message().unwrap().to_string(),
        import_addition_paths: None,
    })
}
