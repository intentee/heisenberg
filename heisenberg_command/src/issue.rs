use std::collections::BTreeMap;
use std::collections::BTreeSet;
use std::iter;

use heisenberg_module_cycles::cycle_site::cycle_site;
use heisenberg_module_cycles::cycle_witness::CycleWitness;
use heisenberg_module_cycles::non_empty::NonEmpty;
use heisenberg_module_cycles::sibling_edge::SiblingEdge;
use heisenberg_module_graph::inactive_region::InactiveRegion;
use heisenberg_module_graph::line_position::LinePosition;
use heisenberg_module_graph::module_dependency::ModuleDependency;
use heisenberg_module_graph::module_file_analysis::ModuleFileAnalysis;
use heisenberg_module_graph::module_path::ModulePath;
use heisenberg_module_graph::source_location::SourceLocation;
use heisenberg_module_graph::violation::Violation;
use heisenberg_project_model::package_violation::PackageViolation;
use heisenberg_project_model::supported_target::SupportedTarget;

use crate::analysis_context::AnalysisContext;
use crate::cycle_shape::CycleShape;
use crate::issue_location::IssueLocation;
use crate::issue_position::IssuePosition;
use crate::module_inclusion::ModuleInclusion;
use crate::narrowing::Narrowing;
use crate::shared_file_inclusions::SharedFileInclusions;
use crate::target_narrowing::TargetNarrowing;
use crate::violation_occurrences::ViolationOccurrences;

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum Issue {
    CodeActiveOnNoTarget {
        focus: SourceLocation,
    },
    ModuleCycle {
        container: ModulePath,
        reported: BTreeSet<AnalysisContext>,
        witnesses: NonEmpty<CycleWitness>,
    },
    Package(PackageViolation),
    SharedModuleFile {
        file: String,
        inclusions: SharedFileInclusions,
    },
    Violation {
        occurrences: ViolationOccurrences,
        violation: Violation,
    },
}

impl Issue {
    #[must_use]
    pub fn new_cycle(
        CycleShape {
            container,
            witnesses,
        }: CycleShape<'_>,
        reported: BTreeSet<AnalysisContext>,
    ) -> Self {
        Self::ModuleCycle {
            container: container.clone(),
            reported,
            witnesses: witnesses.clone(),
        }
    }

    #[must_use]
    pub fn new_inactive_code(InactiveRegion { focus, .. }: &InactiveRegion) -> Self {
        Self::CodeActiveOnNoTarget {
            focus: focus.clone(),
        }
    }

    #[must_use]
    pub fn new_package(violation: &PackageViolation) -> Self {
        Self::Package(violation.clone())
    }

    #[must_use]
    pub fn new_shared_module_file(
        file: &str,
        inclusions: BTreeMap<ModuleInclusion, BTreeMap<SupportedTarget, ModuleFileAnalysis>>,
        submodule_files: BTreeSet<String>,
    ) -> Self {
        Self::SharedModuleFile {
            file: file.to_owned(),
            inclusions: SharedFileInclusions {
                inclusions,
                submodule_files,
            },
        }
    }

    #[must_use]
    pub fn new_violation(violation: &Violation, occurrences: ViolationOccurrences) -> Self {
        Self::Violation {
            occurrences,
            violation: violation.clone(),
        }
    }

    #[must_use]
    pub fn details(&self) -> Vec<String> {
        match self {
            Self::ModuleCycle { witnesses, .. } => witnesses
                .iter()
                .flat_map(
                    |CycleWitness {
                         dependencies,
                         edge: SiblingEdge { from, to },
                     }| {
                        iter::once(format!("  {from} -> {to}:")).chain(dependencies.iter().map(
                            |ModuleDependency { from, location, to }| {
                                format!("    {from} uses {to} at {location}")
                            },
                        ))
                    },
                )
                .collect(),
            Self::SharedModuleFile { inclusions, .. } => inclusions.lines(),
            Self::Violation {
                occurrences: ViolationOccurrences { folded, .. },
                ..
            } => folded
                .iter()
                .map(|finding| format!("  {finding}"))
                .collect(),
            Self::CodeActiveOnNoTarget { .. } | Self::Package(_) => Vec::new(),
        }
    }

    #[must_use]
    pub const fn issue_type(&self) -> &'static str {
        match self {
            Self::CodeActiveOnNoTarget { .. } => "CodeActiveOnNoTarget",
            Self::ModuleCycle { .. } => "ModuleCycle",
            Self::Package(violation) => violation.issue_type(),
            Self::SharedModuleFile { .. } => "SharedModuleFile",
            Self::Violation { violation, .. } => violation.kind.issue_type(),
        }
    }

    #[must_use]
    pub fn lines(
        &self,
        contexts_by_file: &BTreeMap<String, BTreeSet<AnalysisContext>>,
    ) -> Vec<String> {
        let location = self.location();
        let analysed = contexts_by_file.get(&location.file).into_iter().flatten();
        let narrowing = match self {
            Self::ModuleCycle { reported, .. }
            | Self::Violation {
                occurrences: ViolationOccurrences { reported, .. },
                ..
            } => Narrowing {
                base: &analysed.chain(reported).cloned().collect(),
                reported,
            }
            .to_string(),
            Self::SharedModuleFile { inclusions, .. } => {
                let targets = inclusions.targets();

                TargetNarrowing {
                    all: &analysed
                        .map(|AnalysisContext { target, .. }| *target)
                        .chain(targets.iter().copied())
                        .collect(),
                    targets: &targets,
                }
                .to_string()
            }
            Self::CodeActiveOnNoTarget { .. } | Self::Package(_) => String::new(),
        };

        iter::once(format!(
            "ERROR: {} - {location} - {}{narrowing}",
            self.issue_type(),
            self.message()
        ))
        .chain(match &location.position {
            IssuePosition::Line(LinePosition { source_line, .. }) => Some(source_line.clone()),
            IssuePosition::WholeFile => None,
        })
        .chain(self.details())
        .collect()
    }

    #[must_use]
    pub fn location(&self) -> IssueLocation {
        match self {
            Self::CodeActiveOnNoTarget { focus } => IssueLocation::new_position(focus),
            Self::ModuleCycle { witnesses, .. } => {
                IssueLocation::new_position(cycle_site(witnesses))
            }
            Self::Package(violation) => IssueLocation::new_whole_file(violation.file()),
            Self::SharedModuleFile { file, .. } => IssueLocation::new_whole_file(file),
            Self::Violation { violation, .. } => IssueLocation::new_position(&violation.location),
        }
    }

    #[must_use]
    pub fn message(&self) -> String {
        match self {
            Self::CodeActiveOnNoTarget { .. } => "Code is active on no supported target".to_owned(),
            Self::ModuleCycle { container, .. } => {
                format!("Parts of {container} depend on each other")
            }
            Self::Package(violation) => violation.to_string(),
            Self::SharedModuleFile {
                inclusions: SharedFileInclusions { inclusions, .. },
                ..
            } => {
                format!("Module file belongs to {} modules", inclusions.len())
            }
            Self::Violation { violation, .. } => violation.kind.to_string(),
        }
    }
}
