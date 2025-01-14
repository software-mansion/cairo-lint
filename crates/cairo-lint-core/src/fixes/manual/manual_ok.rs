use cairo_lang_syntax::node::{db::SyntaxGroup, SyntaxNode};

use super::helpers::fix_manual;

/// Rewrites a manual implementation of ok
pub fn fix_manual_ok(db: &dyn SyntaxGroup, node: SyntaxNode) -> Option<(SyntaxNode, String)> {
    Some((node.clone(), fix_manual("ok", db, node)))
}
