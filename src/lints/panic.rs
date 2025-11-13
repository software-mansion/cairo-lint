use cairo_lang_defs::diagnostic_utils::StableLocation;
use cairo_lang_defs::ids::ModuleItemId;
use cairo_lang_defs::plugin::PluginDiagnostic;
use cairo_lang_diagnostics::Severity;
use cairo_lang_filesystem::db::get_originating_location;
use cairo_lang_parser::db::ParserGroup;
use cairo_lang_semantic::ExprFunctionCall;
use cairo_lang_semantic::items::functions::GenericFunctionId;
use cairo_lang_syntax::node::{TypedStablePtr, TypedSyntaxNode};
use if_chain::if_chain;
use itertools::Itertools;

use crate::context::{CairoLintKind, Lint};

use crate::LinterGroup;
use crate::helper::ASSERT_FORMATTER_NAME;
use crate::queries::{get_all_function_bodies, get_all_function_calls};
use cairo_lang_filesystem::ids::SpanInFile;
use salsa::Database;

pub struct PanicInCode;

/// ## What it does
///
/// Checks for panic usages.
///
/// ## Example
///
/// ```cairo
/// fn main() {
///     panic!("panic");
/// }
/// ```
impl Lint for PanicInCode {
    fn allowed_name(&self) -> &'static str {
        "panic"
    }

    fn diagnostic_message(&self) -> &'static str {
        "Leaving `panic` in the code is discouraged."
    }

    fn kind(&self) -> CairoLintKind {
        CairoLintKind::Panic
    }

    fn is_enabled(&self) -> bool {
        false
    }
}

#[tracing::instrument(skip_all, level = "trace")]
pub fn check_panic_usage<'db>(
    db: &'db dyn Database,
    item: &ModuleItemId<'db>,
    diagnostics: &mut Vec<PluginDiagnostic<'db>>,
) {
    let function_bodies = get_all_function_bodies(db, item);
    for function_body in function_bodies.iter() {
        let function_call_exprs = get_all_function_calls(function_body);
        for function_call_expr in function_call_exprs.unique() {
            check_single_panic_usage(db, &function_call_expr, diagnostics);
        }
    }
}

fn check_single_panic_usage<'db>(
    db: &'db dyn Database,
    function_call_expr: &ExprFunctionCall<'db>,
    diagnostics: &mut Vec<PluginDiagnostic<'db>>,
) {
    let init_node = function_call_expr.stable_ptr.lookup(db).as_syntax_node();

    let concrete_function_id = function_call_expr
        .function
        .get_concrete(db)
        .generic_function;

    let corelib_context = db.corelib_context();

    // If the function is the panic function from the corelib.
    let is_panic = if let GenericFunctionId::Extern(id) = concrete_function_id
        && id == corelib_context.get_panic_function_id()
    {
        true
    } else {
        false
    };

    // If the function is the panic_with_byte_array function from the corelib.
    let is_panic_with_byte_array = if let GenericFunctionId::Free(id) = concrete_function_id
        && id == corelib_context.get_panic_with_byte_array_function_id()
    {
        true
    } else {
        false
    };

    // We check if the panic comes from the `assert!` macro.
    let is_assert_panic = is_panic_with_byte_array
        && function_call_expr
            .stable_ptr
            .lookup(db)
            .as_syntax_node()
            .get_text(db)
            .contains(ASSERT_FORMATTER_NAME);

    // We skip non panic calls, and panic calls in assert macros.
    if !(is_panic || is_panic_with_byte_array) || is_assert_panic {
        return;
    }

    // Get the origination location of this panic as there is a `panic!` macro that generates virtual
    // files
    let initial_file_id = StableLocation::new(function_call_expr.stable_ptr.untyped()).file_id(db);
    let SpanInFile { file_id, span } = get_originating_location(
        db,
        SpanInFile {
            file_id: initial_file_id,
            span: function_call_expr
                .stable_ptr
                .lookup(db)
                .as_syntax_node()
                .span(db),
        },
        None,
    );
    // If the panic comes from a real file (macros generate code in new virtual files)
    if initial_file_id == file_id {
        diagnostics.push(PluginDiagnostic {
            stable_ptr: init_node.stable_ptr(db),
            message: PanicInCode.diagnostic_message().to_owned(),
            severity: Severity::Warning,
            inner_span: None,
        });
    } else {
        // If the originating location is a different file get the syntax node that generated the
        // code that contains a panic.
        if_chain! {
            if let Some(text_position) = span.position_in_file(db, file_id);
            if let Ok(file_node) = db.file_syntax(file_id);
            then {
                let syntax_node = file_node.lookup_position(db, text_position.start);
                diagnostics.push(PluginDiagnostic {
                    stable_ptr: syntax_node.stable_ptr(db),
                    message: PanicInCode.diagnostic_message().to_owned(),
                    severity: Severity::Warning,
                    inner_span: None
                });
            }
        }
    }
}
