use cargo_util_schemas::manifest::TomlLintLevel;
use cargo_util_schemas::manifest::TomlLints;

use crate::constants::LINT_NAME_SEPARATOR;
use crate::constants::LINT_NAME_SEPARATOR_ALIAS;
use crate::constants::RUST_WARNINGS_LINT;
use crate::required_lint::RequiredLint;

fn applied_after(earlier: TomlLintLevel, later: TomlLintLevel) -> TomlLintLevel {
    if earlier == TomlLintLevel::Forbid {
        earlier
    } else {
        later
    }
}

pub struct LintLevel<'lints> {
    pub lint: &'lints RequiredLint,
    pub lints: &'lints TomlLints,
}

impl LintLevel<'_> {
    pub fn is_denied(&self) -> bool {
        match self.level() {
            TomlLintLevel::Allow => false,
            TomlLintLevel::Deny | TomlLintLevel::Forbid => true,
            TomlLintLevel::Warn => matches!(
                LintLevel {
                    lint: &RUST_WARNINGS_LINT,
                    lints: self.lints,
                }
                .level(),
                TomlLintLevel::Deny | TomlLintLevel::Forbid
            ),
        }
    }

    fn level(&self) -> TomlLintLevel {
        let mut applying: Vec<_> = self
            .lints
            .get(self.lint.tool.name())
            .into_iter()
            .flatten()
            .filter(|(name, _)| {
                self.lint
                    .is_selected_by(&name.replace(LINT_NAME_SEPARATOR_ALIAS, LINT_NAME_SEPARATOR))
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
