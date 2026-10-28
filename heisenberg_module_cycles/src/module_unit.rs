use std::fmt;
use std::fmt::Display;
use std::fmt::Formatter;

use heisenberg_module_graph::module_path::ModulePath;

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum ModuleUnit {
    OwnCode(ModulePath),
    Subtree(ModulePath),
}

impl ModuleUnit {
    #[must_use]
    pub fn within(container: &ModulePath, module: &ModulePath) -> Self {
        container
            .child_towards(module)
            .map_or_else(|| Self::OwnCode(container.clone()), Self::Subtree)
    }
}

impl Display for ModuleUnit {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::OwnCode(module) => write!(formatter, "{module} (own code)"),
            Self::Subtree(module) => write!(formatter, "{module}"),
        }
    }
}
