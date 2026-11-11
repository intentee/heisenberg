use crate::build_unit::BuildUnit;

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct BuildLink {
    pub development_only: bool,
    pub from: BuildUnit,
    pub to: BuildUnit,
}
