use std::collections::BTreeMap;
use std::collections::BTreeSet;

use heisenberg_module_graph::crate_target::CrateTarget;
use heisenberg_module_graph::crate_target_kind::CrateTargetKind;

use crate::constants::CARGO_BINARY_NAME_VARIABLE;
use crate::constants::CARGO_BINARY_PATH_VARIABLE_PREFIX;
use crate::constants::CARGO_PRIMARY_PACKAGE_VARIABLE;
use crate::constants::CARGO_TARGET_TEMPORARY_DIRECTORY_VARIABLE;

pub struct CargoTargetVariables {
    binary_paths: BTreeMap<String, BTreeSet<String>>,
}

impl CargoTargetVariables {
    pub fn new<'target>(targets: impl Iterator<Item = &'target CrateTarget>) -> Self {
        let mut binary_paths: BTreeMap<String, BTreeSet<String>> = BTreeMap::new();

        for CrateTarget {
            name, package_name, ..
        } in targets.filter(|CrateTarget { kind, .. }| *kind == CrateTargetKind::Binary)
        {
            binary_paths
                .entry(package_name.clone())
                .or_default()
                .insert(format!("{CARGO_BINARY_PATH_VARIABLE_PREFIX}{name}"));
        }

        Self { binary_paths }
    }

    pub fn declares(
        &self,
        CrateTarget {
            kind, package_name, ..
        }: &CrateTarget,
        variable: &str,
    ) -> bool {
        variable == CARGO_PRIMARY_PACKAGE_VARIABLE
            || match kind {
                CrateTargetKind::Binary | CrateTargetKind::Example => {
                    variable == CARGO_BINARY_NAME_VARIABLE
                }
                CrateTargetKind::Benchmark | CrateTargetKind::IntegrationTest => {
                    variable == CARGO_TARGET_TEMPORARY_DIRECTORY_VARIABLE
                        || self
                            .binary_paths
                            .get(package_name)
                            .is_some_and(|paths| paths.contains(variable))
                }
                CrateTargetKind::BuildScript | CrateTargetKind::Library => false,
            }
    }
}
