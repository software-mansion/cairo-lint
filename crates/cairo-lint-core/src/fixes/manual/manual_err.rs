use cairo_lang_syntax::node::{db::SyntaxGroup, SyntaxNode};

use super::helpers::fix_manual;

/// Rewrites a manual implementation of err
pub fn fix_manual_err(db: &dyn SyntaxGroup, node: SyntaxNode) -> Option<(SyntaxNode, String)> {
    Some((node.clone(), fix_manual("err", db, node)))
}
