use crate::module_unit::ModuleUnit;

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct SiblingEdge {
    pub from: ModuleUnit,
    pub to: ModuleUnit,
}
