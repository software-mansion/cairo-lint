use cairo_lang_syntax::node::{db::SyntaxGroup, SyntaxNode};

use super::helpers::fix_manual;

/// Rewrites a manual implementation of is_err
pub fn fix_manual_is_err(db: &dyn SyntaxGroup, node: SyntaxNode) -> Option<(SyntaxNode, String)> {
    Some((node.clone(), fix_manual("is_err", db, node)))
}
