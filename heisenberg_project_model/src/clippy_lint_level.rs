use cargo_util_schemas::manifest::TomlLintLevel;
use cargo_util_schemas::manifest::TomlLints;

use crate::clippy_lint::ClippyLint;
use crate::clippy_lint_selector::ClippyLintSelector;
use crate::constants::CLIPPY_TOOL;
use crate::constants::LINT_NAME_SEPARATOR;
use crate::constants::LINT_NAME_SEPARATOR_ALIAS;
use crate::constants::RUST_TOOL;
use crate::constants::RUST_WARNINGS_GROUP;

fn applied_after(earlier: TomlLintLevel, later: TomlLintLevel) -> TomlLintLevel {
    if earlier == TomlLintLevel::Forbid {
        earlier
    } else {
        later
    }
}

pub struct ClippyLintLevel<'lints> {
    pub lint: &'lints ClippyLint,
    pub lints: &'lints TomlLints,
}

impl ClippyLintLevel<'_> {
    pub fn is_denied(&self) -> bool {
        match self.level() {
            TomlLintLevel::Allow => false,
            TomlLintLevel::Deny | TomlLintLevel::Forbid => true,
            TomlLintLevel::Warn => self
                .lints
                .get(RUST_TOOL)
                .and_then(|rust| rust.get(RUST_WARNINGS_GROUP))
                .is_some_and(|warnings| {
                    matches!(
                        warnings.level(),
                        TomlLintLevel::Deny | TomlLintLevel::Forbid
                    )
                }),
        }
    }

    fn level(&self) -> TomlLintLevel {
        let mut applying: Vec<_> = self
            .lints
            .get(CLIPPY_TOOL)
            .into_iter()
            .flatten()
            .filter(|(name, _)| {
                ClippyLintSelector {
                    name: &name.replace(LINT_NAME_SEPARATOR_ALIAS, LINT_NAME_SEPARATOR),
                }
                .selects(self.lint)
            })
            .collect();

        applying.sort_by(|(left_name, left), (right_name, right)| {
            left.priority()
                .cmp(&right.priority())
                .then_with(|| right_name.cmp(left_name))
        });
        applying
            .into_iter()
            .fold(TomlLintLevel::Allow, |level, (_, lint)| {
                applied_after(level, lint.level())
            })
    }
}
