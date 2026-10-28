use std::collections::BTreeMap;
use std::collections::BTreeSet;

use crate::package_edge::PackageEdge;

pub struct ResolvedBuild {
    pub edges: BTreeSet<PackageEdge>,
    pub features: BTreeMap<String, BTreeSet<String>>,
}
