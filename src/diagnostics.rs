use cairo_lang_compiler::db::RootDatabase;
use cairo_lang_diagnostics::format_diagnostics as cairo_format_diagnostics;
use cairo_lang_diagnostics::DiagnosticEntry;
use cairo_lang_semantic::SemanticDiagnostic;
use cairo_lang_utils::Upcast;

pub fn format_diagnostic<'a>(diagnostic: &'a SemanticDiagnostic, db: &'a RootDatabase) -> String {
    cairo_format_diagnostics(
        db.upcast(),
        &diagnostic.format(db),
        diagnostic.location(db.upcast()),
    )
}
