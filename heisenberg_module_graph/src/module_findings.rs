use std::collections::BTreeSet;

use crate::configured_out_negations::ConfiguredOutNegations;
use crate::every_configuration_findings::EveryConfigurationFindings;
use crate::module_dependency::ModuleDependency;
use crate::shared_module_file::SharedModuleFile;
use crate::unresolved_foreign_reference::UnresolvedForeignReference;
use crate::violation_set::ViolationSet;
use crate::walked_code::WalkedCode;

#[derive(Clone, Debug, Default, Eq, Ord, PartialEq, PartialOrd)]
pub struct ModuleFindings {
    pub configured_out_negations: ConfiguredOutNegations,
    pub dependencies: BTreeSet<ModuleDependency>,
    pub shared_module_files: BTreeSet<SharedModuleFile>,
    pub unresolved_foreign_references: BTreeSet<UnresolvedForeignReference>,
    pub every_configuration: EveryConfigurationFindings,
    pub violations: ViolationSet,
    pub walked_code: WalkedCode,
}

impl ModuleFindings {
    pub fn merge(
        &mut self,
        Self {
            configured_out_negations,
            dependencies,
            shared_module_files,
            unresolved_foreign_references,
            every_configuration,
            violations,
            walked_code,
        }: Self,
    ) {
        self.configured_out_negations
            .merge(configured_out_negations);
        self.dependencies.extend(dependencies);
        self.shared_module_files.extend(shared_module_files);
        self.unresolved_foreign_references
            .extend(unresolved_foreign_references);
        self.every_configuration.merge(every_configuration);
        self.violations.merge(violations);
        self.walked_code.merge(walked_code);
    }
}
