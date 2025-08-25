use cairo_lang_diagnostics::Maybe;
use cairo_lang_filesystem::ids::{FileId, FileKind};
use cairo_lang_parser::db::ParserGroup;
use cairo_lang_syntax::node::{SyntaxNode, TypedSyntaxNode};

pub fn file_syntax<'db>(db: &'db dyn ParserGroup, file: FileId<'db>) -> Maybe<SyntaxNode<'db>> {
    match file.kind(db) {
        FileKind::Expr => db.file_expr_syntax(file).map(|a| a.as_syntax_node()),
        FileKind::Module => db.file_module_syntax(file).map(|a| a.as_syntax_node()),
        FileKind::StatementList => db
            .file_statement_list_syntax(file)
            .map(|a| a.as_syntax_node()),
    }
}
