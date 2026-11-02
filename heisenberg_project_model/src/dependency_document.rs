use std::collections::BTreeSet;

use cargo::workspace::dependency::DepKind;

use crate::dependency_kind_document::DependencyKindDocument;

pub struct DependencyDocument {
    pub extern_crate_name: String,
    pub kinds: BTreeSet<DependencyKindDocument>,
    pub package_id: String,
}

impl DependencyDocument {
    pub fn has_kind(&self, kind: DepKind) -> bool {
        self.kinds.iter().any(|document| document.kind == kind)
    }
}
