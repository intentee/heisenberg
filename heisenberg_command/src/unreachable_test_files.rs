use std::collections::BTreeSet;

use heisenberg_module_graph::crate_module_graph::CrateModuleGraph;
use heisenberg_project_model::package_violation::PackageViolation;

use crate::configuration_analysis::ConfigurationAnalysis;

pub fn unreachable_test_files(analyses: &[ConfigurationAnalysis]) -> Vec<PackageViolation> {
    let module_files: BTreeSet<&String> = analyses
        .iter()
        .flat_map(|ConfigurationAnalysis { graphs, .. }| graphs)
        .flat_map(|CrateModuleGraph { module_files, .. }| module_files)
        .collect();

    analyses
        .iter()
        .flat_map(
            |ConfigurationAnalysis {
                 test_directory_files,
                 ..
             }| test_directory_files,
        )
        .collect::<BTreeSet<&String>>()
        .into_iter()
        .filter(|file| !module_files.contains(file))
        .map(|file| PackageViolation::UnreachableTestFile { file: file.clone() })
        .collect()
}
