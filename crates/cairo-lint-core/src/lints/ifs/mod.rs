pub mod collapsible_if_else;
pub mod equatable_if_let;
pub const ALLOWED: [&str; 2] = [collapsible_if_else::LINT_NAME, equatable_if_let::LINT_NAME];
