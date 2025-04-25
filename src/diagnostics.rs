use cairo_lang_diagnostics::format_diagnostics as cairo_format_diagnostics;
use cairo_lang_diagnostics::DiagnosticEntry;
use cairo_lang_semantic::db::SemanticGroup;
use cairo_lang_semantic::SemanticDiagnostic;

pub fn format_diagnostic<'a>(
    diagnostic: &'a SemanticDiagnostic,
    db: &(dyn SemanticGroup + 'static),
) -> String {
    cairo_format_diagnostics(db, &diagnostic.format(db), diagnostic.location(db))
}
