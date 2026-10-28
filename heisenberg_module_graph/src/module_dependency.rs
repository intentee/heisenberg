use crate::module_path::ModulePath;
use crate::source_location::SourceLocation;

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct ModuleDependency {
    pub from: ModulePath,
    pub location: SourceLocation,
    pub to: ModulePath,
}
