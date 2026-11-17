use std::collections::BTreeMap;
use std::collections::BTreeSet;

use heisenberg_module_graph::code_category::CodeCategory;
use heisenberg_module_graph::crate_module_graph::CrateModuleGraph;
use heisenberg_module_graph::crate_target_kind::CrateTargetKind;
use heisenberg_module_graph::dependency_reference::DependencyReference;
use heisenberg_project_model::dependency_consumer::DependencyConsumer;
use heisenberg_project_model::dependency_use::DependencyUse;

use crate::configuration_analysis::ConfigurationAnalysis;

pub fn dependency_uses(
    analyses: &[ConfigurationAnalysis],
) -> BTreeMap<String, BTreeSet<DependencyUse>> {
    let mut uses: BTreeMap<String, BTreeSet<DependencyUse>> = BTreeMap::new();

    for CrateModuleGraph {
        crate_target,
        findings,
        ..
    } in analyses
        .iter()
        .flat_map(|ConfigurationAnalysis { graphs, .. }| graphs)
    {
        uses.entry(crate_target.package_name.clone())
            .or_default()
            .extend(findings.dependency_references.iter().map(
                |DependencyReference {
                     category,
                     extern_crate_name,
                 }| DependencyUse {
                    consumer: match (crate_target.kind, category) {
                        (CrateTargetKind::BuildScript, _) => DependencyConsumer::BuildScript,
                        (_, CodeCategory::Test) => DependencyConsumer::Test,
                        (CrateTargetKind::Example, _) | (_, CodeCategory::Benchmark) => {
                            DependencyConsumer::Example
                        }
                        (_, CodeCategory::Production) => DependencyConsumer::Production,
                    },
                    extern_crate_name: extern_crate_name.clone(),
                },
            ));
    }

    uses
}
