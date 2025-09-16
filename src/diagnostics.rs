use cairo_lang_diagnostics::DiagnosticEntry;
use cairo_lang_diagnostics::format_diagnostics as cairo_format_diagnostics;
use cairo_lang_semantic::SemanticDiagnostic;
use salsa::Database;

pub fn format_diagnostic(diagnostic: &SemanticDiagnostic, db: &dyn Database) -> String {
    cairo_format_diagnostics(db, &diagnostic.format(db), diagnostic.location(db))
}
