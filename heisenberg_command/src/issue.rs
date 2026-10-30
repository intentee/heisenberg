use std::collections::BTreeMap;
use std::collections::BTreeSet;
use std::iter;

use heisenberg_module_cycles::cycle_witness::CycleWitness;
use heisenberg_module_graph::inactive_region::InactiveRegion;
use heisenberg_module_graph::line_position::LinePosition;
use heisenberg_module_graph::module_dependency::ModuleDependency;
use heisenberg_module_graph::module_file_analysis::ModuleFileAnalysis;
use heisenberg_module_graph::violation::Violation;
use heisenberg_project_model::package_violation::PackageViolation;
use heisenberg_project_model::supported_target::SupportedTarget;

use crate::analysis_context::AnalysisContext;
use crate::cycle_shape::CycleShape;
use crate::issue_content::IssueContent;
use crate::issue_location::IssueLocation;
use crate::issue_position::IssuePosition;
use crate::module_inclusion::ModuleInclusion;
use crate::narrowing::Narrowing;
use crate::shared_file_inclusions::SharedFileInclusions;
use crate::target_narrowing::TargetNarrowing;

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct Issue {
    pub location: IssueLocation,
    pub content: IssueContent,
}

impl Issue {
    #[expect(
        clippy::expect_used,
        reason = "a cycle exists only through module dependencies, and each has a site"
    )]
    #[must_use]
    pub fn new_cycle(
        CycleShape {
            container,
            witnesses,
        }: CycleShape<'_>,
        reported: BTreeSet<AnalysisContext>,
    ) -> Self {
        Self {
            content: IssueContent::ModuleCycle {
                container: container.clone(),
                reported,
                witnesses: witnesses.to_vec(),
            },
            location: IssueLocation::new_position(
                witnesses
                    .iter()
                    .flat_map(|CycleWitness { dependencies, .. }| dependencies)
                    .map(|ModuleDependency { location, .. }| location)
                    .min()
                    .expect("every cycle has a dependency site"),
            ),
        }
    }

    #[must_use]
    pub fn new_inactive_code(InactiveRegion { start, .. }: &InactiveRegion) -> Self {
        Self {
            content: IssueContent::CodeActiveOnNoTarget,
            location: IssueLocation::new_position(start),
        }
    }

    #[must_use]
    pub fn new_package(violation: &PackageViolation) -> Self {
        Self {
            content: IssueContent::Package(violation.clone()),
            location: IssueLocation::new_whole_file(violation.manifest()),
        }
    }

    #[must_use]
    pub fn new_shared_module_file(
        file: &str,
        inclusions: BTreeMap<ModuleInclusion, BTreeMap<SupportedTarget, ModuleFileAnalysis>>,
    ) -> Self {
        Self {
            content: IssueContent::SharedModuleFile(SharedFileInclusions { inclusions }),
            location: IssueLocation::new_whole_file(file),
        }
    }

    #[must_use]
    pub fn new_violation(
        Violation { kind, location }: &Violation,
        reported: BTreeSet<AnalysisContext>,
    ) -> Self {
        Self {
            content: IssueContent::Violation {
                kind: kind.clone(),
                reported,
            },
            location: IssueLocation::new_position(location),
        }
    }

    #[must_use]
    pub fn lines(
        &self,
        contexts_by_file: &BTreeMap<String, BTreeSet<AnalysisContext>>,
    ) -> Vec<String> {
        let Self { content, location } = self;
        let narrowing = match content {
            IssueContent::ModuleCycle { reported, .. }
            | IssueContent::Violation { reported, .. } => Narrowing {
                base: &contexts_by_file[&location.file],
                reported,
            }
            .to_string(),
            IssueContent::SharedModuleFile(inclusions) => TargetNarrowing {
                all: &contexts_by_file[&location.file]
                    .iter()
                    .map(|AnalysisContext { target, .. }| *target)
                    .collect(),
                targets: &inclusions.targets(),
            }
            .to_string(),
            IssueContent::CodeActiveOnNoTarget | IssueContent::Package(_) => String::new(),
        };

        iter::once(format!(
            "ERROR: {} - {location} - {}{narrowing}",
            content.issue_type(),
            content.message()
        ))
        .chain(match &location.position {
            IssuePosition::Line(LinePosition { source_line, .. }) => Some(source_line.clone()),
            IssuePosition::WholeFile => None,
        })
        .chain(content.details())
        .collect()
    }
}
