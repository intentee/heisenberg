use cargo::workspace::dependency::DepKind;

use crate::dependency_kind::DependencyKind;

pub struct CargoDependencyKind {
    pub kind: DepKind,
}

impl CargoDependencyKind {
    pub const fn dependency_kind(&self) -> DependencyKind {
        match self.kind {
            DepKind::Build => DependencyKind::Build,
            DepKind::Development => DependencyKind::Development,
            DepKind::Normal => DependencyKind::Normal,
        }
    }
}
