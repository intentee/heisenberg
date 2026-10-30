use std::collections::BTreeSet;
use std::iter;

use heisenberg_module_cycles::cycle_witness::CycleWitness;
use heisenberg_module_graph::module_dependency::ModuleDependency;
use heisenberg_module_graph::module_path::ModulePath;
use heisenberg_module_graph::violation_kind::ViolationKind;
use heisenberg_project_model::package_violation::PackageViolation;

use crate::analysis_context::AnalysisContext;
use crate::shared_file_inclusions::SharedFileInclusions;

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum IssueContent {
    CodeActiveOnNoTarget,
    ModuleCycle {
        container: ModulePath,
        reported: BTreeSet<AnalysisContext>,
        witnesses: Vec<CycleWitness>,
    },
    Package(PackageViolation),
    SharedModuleFile(SharedFileInclusions),
    Violation {
        kind: ViolationKind,
        reported: BTreeSet<AnalysisContext>,
    },
}

impl IssueContent {
    #[must_use]
    pub fn details(&self) -> Vec<String> {
        match self {
            Self::ModuleCycle { witnesses, .. } => witnesses
                .iter()
                .flat_map(
                    |CycleWitness {
                         dependencies,
                         from,
                         to,
                     }| {
                        iter::once(format!("  {from} -> {to}:")).chain(dependencies.iter().map(
                            |ModuleDependency { from, location, to }| {
                                format!("    {from} uses {to} at {location}")
                            },
                        ))
                    },
                )
                .collect(),
            Self::SharedModuleFile(inclusions) => inclusions.lines(),
            Self::CodeActiveOnNoTarget | Self::Package(_) | Self::Violation { .. } => Vec::new(),
        }
    }

    #[must_use]
    pub const fn issue_type(&self) -> &'static str {
        match self {
            Self::CodeActiveOnNoTarget => "CodeActiveOnNoTarget",
            Self::ModuleCycle { .. } => "ModuleCycle",
            Self::Package(violation) => violation.issue_type(),
            Self::SharedModuleFile(_) => "SharedModuleFile",
            Self::Violation { kind, .. } => kind.issue_type(),
        }
    }

    #[must_use]
    pub fn message(&self) -> String {
        match self {
            Self::CodeActiveOnNoTarget => "Code is active on no supported target".to_owned(),
            Self::ModuleCycle { container, .. } => {
                format!("Parts of {container} depend on each other")
            }
            Self::Package(violation) => violation.to_string(),
            Self::SharedModuleFile(SharedFileInclusions { inclusions }) => {
                format!("Module file belongs to {} modules", inclusions.len())
            }
            Self::Violation { kind, .. } => kind.to_string(),
        }
    }
}
