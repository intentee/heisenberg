use std::collections::BTreeMap;
use std::collections::BTreeSet;
use std::sync::Arc;

use cargo_metadata::Metadata;
use guppy::Error;
use guppy::graph::DependencyDirection;
use guppy::graph::PackageGraph;
use guppy::graph::cargo::CargoOptions;
use guppy::graph::cargo::CargoSet;
use guppy::graph::feature::StandardFeatures;
use guppy::platform::Platform;
use guppy::platform::PlatformSpec;
use serde_json::error;
use serde_json::from_value;

use crate::build_selection::BuildSelection;
use crate::package_edge::PackageEdge;
use crate::resolved_build::ResolvedBuild;
use crate::resolved_packages::ResolvedPackages;

pub struct FeatureGraph {
    pub graph: PackageGraph,
    pub resolved: ResolvedPackages,
}

impl FeatureGraph {
    pub fn new(resolved: ResolvedPackages) -> Result<Self, Error> {
        PackageGraph::from_json(resolved.metadata(&BuildSelection::Everything).to_string())
            .map(|graph| Self { graph, resolved })
    }

    pub fn build(
        &self,
        features: StandardFeatures,
        platform: &Arc<Platform>,
    ) -> Result<ResolvedBuild, Error> {
        let initials = self.graph.resolve_workspace().to_feature_set(features);
        let platform = PlatformSpec::Platforms(vec![platform.clone()]);
        let mut options = CargoOptions::new();

        options
            .set_resolver(self.resolved.resolver)
            .set_include_dev(true)
            .set_target_platform(platform.clone())
            .set_host_platform(platform);

        CargoSet::new(
            initials,
            self.graph.feature_graph().resolve_none(),
            &options,
        )
        .map(|cargo_set| {
            let mut features: BTreeMap<String, BTreeSet<String>> = BTreeMap::new();

            for feature_set in [cargo_set.target_features(), cargo_set.host_features()] {
                for feature_list in feature_set.packages_with_features(DependencyDirection::Forward)
                {
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
        })
    }

    pub fn metadata(&self, build: ResolvedBuild) -> Result<Metadata, error::Error> {
        from_value(self.resolved.metadata(&BuildSelection::Resolved(build)))
    }
}
