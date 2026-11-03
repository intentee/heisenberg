use std::collections::BTreeSet;

use crate::dependency_kind_document::DependencyKindDocument;

pub struct DependencyDocument {
    pub extern_crate_name: String,
    pub kinds: BTreeSet<DependencyKindDocument>,
    pub package_id: String,
}
