use std::collections::BTreeMap;
use std::collections::BTreeSet;
use std::sync::Arc;

use cargo_metadata::Metadata;
use guppy::DependencyKind;
use guppy::Error;
use guppy::graph::DependencyDirection;
use guppy::graph::PackageGraph;
use guppy::graph::PackageLink;
use guppy::graph::cargo::CargoOptions;
use guppy::graph::cargo::CargoSet;
use guppy::graph::feature::FeatureFilter;
use guppy::graph::feature::FeatureId;
use guppy::graph::feature::FeatureSet;
use guppy::graph::feature::StandardFeatures;
use guppy::graph::feature::named_feature_filter;
use guppy::platform::EnabledTernary;
use guppy::platform::Platform;
use guppy::platform::PlatformSpec;
use serde_json::error;
use serde_json::from_value;

use crate::build_link::BuildLink;
use crate::build_selection::BuildSelection;
use crate::build_side::BuildSide;
use crate::build_unit::BuildUnit;
use crate::feature_selection::FeatureSelection;
use crate::package_edge::PackageEdge;
use crate::resolved_build::ResolvedBuild;
use crate::resolved_packages::ResolvedPackages;

fn build_link(
    link: &PackageLink<'_>,
    development_only: bool,
    from: BuildSide,
    to: BuildSide,
) -> BuildLink {
    BuildLink {
        development_only,
        from: BuildUnit {
            package: link.from().id().repr().to_owned(),
            side: from,
        },
        to: BuildUnit {
            package: link.to().id().repr().to_owned(),
            side: to,
        },
    }
}

fn is_enabled(
    features: &FeatureSet<'_>,
    link: &PackageLink<'_>,
    kind: DependencyKind,
    platform: &PlatformSpec,
) -> Result<bool, Error> {
    let status = link.req_for_kind(kind).status();

    if status.optional_status().is_never() {
        Ok(status.required_on(platform) != EnabledTernary::Disabled)
    } else {
        features
            .contains(FeatureId::optional_dependency(
                link.from().id(),
                link.dep_name(),
            ))
            .map(|activated| {
                let enabled = if activated {
                    status.enabled_on(platform)
                } else {
                    status.required_on(platform)
                };

                enabled != EnabledTernary::Disabled
            })
    }
}

pub struct FeatureGraph {
    pub graph: PackageGraph,
    pub resolved: ResolvedPackages,
}

impl FeatureGraph {
    pub fn new(resolved: ResolvedPackages) -> Result<Self, Error> {
        PackageGraph::from_json(resolved.metadata(&BuildSelection::Everything).to_string())
            .map(|graph| Self { graph, resolved })
    }

    pub fn build<'graph>(
        &'graph self,
        features: impl FeatureFilter<'graph>,
        platform: &Arc<Platform>,
    ) -> Result<ResolvedBuild, Error> {
        let initials = self.graph.resolve_workspace().to_feature_set(features);
        let platform = PlatformSpec::Platforms(vec![platform.clone()]);
        let mut options = CargoOptions::new();

        options
            .set_resolver(self.resolved.resolver)
            .set_include_dev(true)
            .set_target_platform(platform.clone())
            .set_host_platform(platform.clone());

        CargoSet::new(
            initials,
            self.graph.feature_graph().resolve_none(),
            &options,
        )
        .and_then(|cargo_set| {
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

            let target = cargo_set.target_features();
            let host = cargo_set.host_features();

            cargo_set
                .target_links()
                .map(|link| {
                    is_enabled(target, &link, DependencyKind::Normal, &platform).map(|normal| {
                        build_link(&link, !normal, BuildSide::Target, BuildSide::Target)
                    })
                })
                .chain(cargo_set.host_links().map(|link| {
                    is_enabled(host, &link, DependencyKind::Normal, &platform).and_then(|normal| {
                        is_enabled(host, &link, DependencyKind::Build, &platform).map(|build| {
                            build_link(
                                &link,
                                !(normal || link.from().has_build_script() && build),
                                BuildSide::Host,
                                BuildSide::Host,
                            )
                        })
                    })
                }))
                .chain(cargo_set.proc_macro_links().map(|link| {
                    is_enabled(target, &link, DependencyKind::Normal, &platform).map(|normal| {
                        build_link(&link, !normal, BuildSide::Target, BuildSide::Host)
                    })
                }))
                .chain(
                    cargo_set.build_dep_links().map(|link| {
                        Ok(build_link(&link, false, BuildSide::Target, BuildSide::Host))
                    }),
                )
                .collect::<Result<BTreeSet<BuildLink>, Error>>()
                .map(|links| ResolvedBuild { features, links })
        })
    }

    pub fn build_selected(
        &self,
        FeatureSelection {
            default_features,
            features,
        }: &FeatureSelection,
        platform: &Arc<Platform>,
    ) -> Result<ResolvedBuild, Error> {
        self.build(
            named_feature_filter(
                if *default_features {
                    StandardFeatures::Default
                } else {
                    StandardFeatures::None
                },
                features.iter().map(String::as_str),
            ),
            platform,
        )
    }

    pub fn metadata(
        &self,
        ResolvedBuild { features, links }: &ResolvedBuild,
    ) -> Result<Metadata, error::Error> {
        from_value(
            self.resolved.metadata(&BuildSelection::Resolved {
                edges: links
                    .iter()
                    .map(|BuildLink { from, to, .. }| PackageEdge {
                        from: from.package.clone(),
                        to: to.package.clone(),
                    })
                    .collect(),
                features,
            }),
        )
    }
}
