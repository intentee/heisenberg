use crate::package_edge::PackageEdge;
use crate::resolved_build::ResolvedBuild;

pub enum BuildSelection {
    Everything,
    Resolved(ResolvedBuild),
}

impl BuildSelection {
    pub fn includes_package(&self, package_id: &str) -> bool {
        match self {
            Self::Everything => true,
            Self::Resolved(build) => build.features.contains_key(package_id),
        }
    }

    pub fn includes_edge(&self, edge: &PackageEdge) -> bool {
        match self {
            Self::Everything => true,
            Self::Resolved(build) => build.edges.contains(edge),
        }
    }

    pub fn features(&self, package_id: &str) -> Vec<String> {
        match self {
            Self::Everything => Vec::new(),
            Self::Resolved(build) => build
                .features
                .get(package_id)
                .into_iter()
                .flatten()
                .cloned()
                .collect(),
        }
    }
}
