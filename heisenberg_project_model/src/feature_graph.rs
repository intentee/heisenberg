use std::collections::BTreeMap;
use std::collections::BTreeSet;
use std::sync::Arc;

use guppy::graph::DependencyDirection;
use guppy::graph::PackageGraph;
use guppy::graph::cargo::CargoOptions;
use guppy::graph::cargo::CargoSet;
use guppy::graph::feature::StandardFeatures;
use guppy::platform::Platform;
use guppy::platform::PlatformSpec;

use crate::build_selection::BuildSelection;
use crate::package_edge::PackageEdge;
use crate::resolved_build::ResolvedBuild;
use crate::resolved_packages::ResolvedPackages;

pub struct FeatureGraph {
    pub graph: PackageGraph,
    pub resolved: ResolvedPackages,
}

impl FeatureGraph {
    #[expect(
        clippy::expect_used,
        reason = "guppy reads the metadata document that Heisenberg builds from cargo itself"
    )]
    #[must_use]
    pub fn new(resolved: ResolvedPackages) -> Self {
        Self {
            graph: PackageGraph::from_json(
                resolved.metadata(&BuildSelection::Everything).to_string(),
            )
            .expect("guppy reads the metadata document that cargo produced"),
            resolved,
        }
    }

    #[expect(
        clippy::expect_used,
        reason = "every package id passed to guppy comes from the same package graph"
    )]
    pub fn build(&self, features: StandardFeatures, platform: &Arc<Platform>) -> ResolvedBuild {
        let initials = self.graph.resolve_workspace().to_feature_set(features);
        let platform = PlatformSpec::Platforms(vec![platform.clone()]);
        let mut options = CargoOptions::new();

        options
            .set_resolver(self.resolved.resolver)
            .set_include_dev(true)
            .set_target_platform(platform.clone())
            .set_host_platform(platform);

        let cargo_set = CargoSet::new(
            initials,
            self.graph.feature_graph().resolve_none(),
            &options,
        )
        .expect("every package id comes from the same package graph");
        let mut features: BTreeMap<String, BTreeSet<String>> = BTreeMap::new();

        for feature_set in [cargo_set.target_features(), cargo_set.host_features()] {
            for feature_list in feature_set.packages_with_features(DependencyDirection::Forward) {
                features
                    .entry(feature_list.package().id().repr().to_owned())
                    .or_default()
                    .extend(feature_list.named_features().map(str::to_owned));
            }
        }

        ResolvedBuild {
            edges: cargo_set
                .target_links()
                .chain(cargo_set.host_links())
                .chain(cargo_set.proc_macro_links())
                .chain(cargo_set.build_dep_links())
                .map(|link| PackageEdge {
                    from: link.from().id().repr().to_owned(),
                    to: link.to().id().repr().to_owned(),
                })
                .collect(),
            features,
        }
    }
}
