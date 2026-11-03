use crate::foreign_module::ForeignModule;
use crate::source_location::SourceLocation;

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct UnresolvedForeignReference {
    pub location: SourceLocation,
    pub module: ForeignModule,
}
