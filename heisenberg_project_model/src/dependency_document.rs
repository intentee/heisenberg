use std::collections::BTreeSet;

use serde_json::Value;

use crate::dependency_kind::DependencyKind;

pub struct DependencyDocument {
    pub json: Value,
    pub kinds: BTreeSet<DependencyKind>,
    pub package_id: String,
}
