use guppy::Version;

use crate::build_side::BuildSide;

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct BuildUnit {
    pub name: String,
    pub package: String,
    pub side: BuildSide,
    pub version: Version,
}
