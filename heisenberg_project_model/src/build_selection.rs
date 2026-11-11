use std::collections::BTreeMap;
use std::collections::BTreeSet;

use crate::package_edge::PackageEdge;

pub enum BuildSelection<'build> {
    Everything,
    Resolved {
        edges: BTreeSet<PackageEdge>,
        features: &'build BTreeMap<String, BTreeSet<String>>,
    },
}

impl BuildSelection<'_> {
    pub fn features(&self, package_id: &str) -> Vec<String> {
        match self {
            Self::Everything => Vec::new(),
            Self::Resolved { features, .. } => features
                .get(package_id)
                .into_iter()
                .flatten()
                .cloned()
                .collect(),
        }
    }

    pub fn includes_edge(&self, edge: &PackageEdge) -> bool {
        match self {
            Self::Everything => true,
            Self::Resolved { edges, .. } => edges.contains(edge),
        }
    }

    pub fn includes_package(&self, package_id: &str) -> bool {
        match self {
            Self::Everything => true,
            Self::Resolved { features, .. } => features.contains_key(package_id),
        }
    }
}
