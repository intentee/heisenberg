use crate::lint_tool::LintTool;

pub struct RequiredLint {
    pub former_names: &'static [&'static str],
    pub groups: &'static [&'static str],
    pub name: &'static str,
    pub tool: LintTool,
}

impl RequiredLint {
    #[must_use]
    pub fn is_named_by(&self, path: &[String]) -> bool {
        match path {
            [name] => self.tool == LintTool::Rust && self.is_selected_by(name),
            [tool, name] => *tool == self.tool.name() && self.is_selected_by(name),
            _ => false,
        }
    }

    #[must_use]
    pub fn is_selected_by(&self, selector: &str) -> bool {
        self.name == selector
            || self.former_names.contains(&selector)
            || self.groups.contains(&selector)
    }

    #[must_use]
    pub fn path(&self) -> String {
        match self.tool {
            LintTool::Clippy => format!("{}::{}", self.tool.name(), self.name),
            LintTool::Rust => self.name.to_owned(),
        }
    }
}
