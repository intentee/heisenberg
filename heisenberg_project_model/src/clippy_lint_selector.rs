use crate::clippy_lint::ClippyLint;
use crate::constants::CLIPPY_DENIED_LINTS_GROUP;

pub struct ClippyLintSelector<'selector> {
    pub name: &'selector str,
}

impl ClippyLintSelector<'_> {
    #[must_use]
    pub fn selects(&self, ClippyLint { former_names, name }: &ClippyLint) -> bool {
        self.name == CLIPPY_DENIED_LINTS_GROUP
            || self.name == *name
            || former_names.contains(&self.name)
    }
}
