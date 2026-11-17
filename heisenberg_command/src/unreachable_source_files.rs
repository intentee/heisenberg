use std::collections::BTreeSet;

use heisenberg_module_graph::crate_module_graph::CrateModuleGraph;
use heisenberg_project_model::package_violation::PackageViolation;

use crate::configuration_analysis::ConfigurationAnalysis;

pub fn unreachable_source_files(analyses: &[ConfigurationAnalysis]) -> Vec<PackageViolation> {
    let accounted_files: BTreeSet<&String> = analyses
        .iter()
        .flat_map(
            |ConfigurationAnalysis {
                 build_script_output_files,
                 graphs,
                 ..
             }| {
                graphs
                    .iter()
                    .flat_map(|CrateModuleGraph { module_files, .. }| module_files)
                    .chain(build_script_output_files)
            },
        )
        .collect();

    analyses
        .iter()
        .flat_map(
            |ConfigurationAnalysis {
                 member_source_files,
                 ..
             }| member_source_files,
        )
        .collect::<BTreeSet<&String>>()
        .into_iter()
        .filter(|file| !accounted_files.contains(file))
        .map(|file| PackageViolation::UnreachableSourceFile { file: file.clone() })
        .collect()
}
