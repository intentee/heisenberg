use crate::constants::CLIPPY_TOOL;

#[must_use]
pub fn clippy_lint_path(lint: &str) -> String {
    format!("{CLIPPY_TOOL}::{lint}")
}
