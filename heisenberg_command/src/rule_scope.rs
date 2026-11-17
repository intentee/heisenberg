use std::fmt;
use std::fmt::Display;
use std::fmt::Formatter;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum RuleScope {
    CrateTarget,
    ModuleFile,
    Package,
    Workspace,
}

impl Display for RuleScope {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
        formatter.write_str(match self {
            Self::CrateTarget => "crate target",
            Self::ModuleFile => "module file",
            Self::Package => "package",
            Self::Workspace => "workspace",
        })
    }
}
