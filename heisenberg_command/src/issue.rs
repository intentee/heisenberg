use std::collections::BTreeMap;
use std::collections::BTreeSet;
use std::iter;

use heisenberg_module_cycles::cycle_witness::CycleWitness;
use heisenberg_module_cycles::non_empty::NonEmpty;
use heisenberg_module_cycles::sibling_edge::SiblingEdge;
use heisenberg_module_graph::foreign_module::ForeignModule;
use heisenberg_module_graph::line_position::LinePosition;
use heisenberg_module_graph::module_dependency::ModuleDependency;
use heisenberg_module_graph::module_file_analysis::ModuleFileAnalysis;
use heisenberg_module_graph::module_path::ModulePath;
use heisenberg_module_graph::source_location::SourceLocation;
use heisenberg_module_graph::violation::Violation;
use heisenberg_project_model::configuration::Configuration;
use heisenberg_project_model::package_violation::PackageViolation;

use crate::analysis_context::AnalysisContext;
use crate::configuration_narrowing::ConfigurationNarrowing;
use crate::cycle_shape::CycleShape;
use crate::issue_location::IssueLocation;
use crate::issue_position::IssuePosition;
use crate::issue_type::IssueType;
use crate::module_inclusion::ModuleInclusion;
use crate::narrowing::Narrowing;
use crate::shared_file_inclusions::SharedFileInclusions;
use crate::violation_occurrences::ViolationOccurrences;

const CONTINUATION_INDENT: &str = "    ";

fn configuration_narrowing<'contexts>(
    analysed: impl Iterator<Item = &'contexts AnalysisContext>,
    configurations: &BTreeSet<Configuration>,
) -> String {
    ConfigurationNarrowing {
        all: &analysed
            .map(|AnalysisContext { configuration, .. }| configuration)
            .chain(configurations)
            .cloned()
            .collect(),
        configurations,
    }
    .to_string()
}

fn cycle_site(witnesses: &NonEmpty<CycleWitness>) -> &SourceLocation {
    witnesses
        .iter()
        .flat_map(|CycleWitness { dependencies, .. }| dependencies.iter())
        .map(|ModuleDependency { location, .. }| location)
        .fold(&witnesses.first.dependencies.first.location, Ord::min)
}

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum Issue {
    CodeActiveInNoConfiguration {
        focus: SourceLocation,
    },
    DuplicatedFunctionBody {
        configurations: BTreeSet<Configuration>,
        copies: NonEmpty<SourceLocation>,
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
    UnresolvedForeignReferences {
        configurations: BTreeSet<Configuration>,
        module: ForeignModule,
        sites: NonEmpty<SourceLocation>,
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
    pub fn new_duplicated_function_body(
        NonEmpty { first, others }: &NonEmpty<&SourceLocation>,
        configurations: BTreeSet<Configuration>,
    ) -> Self {
        Self::DuplicatedFunctionBody {
            configurations,
            copies: NonEmpty {
                first: (*first).clone(),
                others: others.iter().map(|copy| (*copy).clone()).collect(),
            },
        }
    }

    #[must_use]
    pub fn new_inactive_code(focus: &SourceLocation) -> Self {
        Self::CodeActiveInNoConfiguration {
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
        inclusions: BTreeMap<ModuleInclusion, BTreeMap<Configuration, ModuleFileAnalysis>>,
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
    pub fn new_unresolved_foreign_references(
        module: &ForeignModule,
        NonEmpty { first, others }: &NonEmpty<&SourceLocation>,
        configurations: BTreeSet<Configuration>,
    ) -> Self {
        Self::UnresolvedForeignReferences {
            configurations,
            module: module.clone(),
            sites: NonEmpty {
                first: (*first).clone(),
                others: others.iter().map(|site| (*site).clone()).collect(),
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
            Self::DuplicatedFunctionBody { copies, .. } => copies
                .others
                .iter()
                .map(|copy| format!("  duplicate at {copy}"))
                .collect(),
            Self::SharedModuleFile { inclusions, .. } => inclusions.lines(),
            Self::UnresolvedForeignReferences { sites, .. } => sites
                .others
                .iter()
                .map(|site| format!("  also unresolved at {site}"))
                .collect(),
            Self::Violation {
                occurrences: ViolationOccurrences { folded, .. },
                ..
            } => folded
                .iter()
                .map(|finding| format!("  {finding}"))
                .collect(),
            Self::Package(violation) => violation
                .details()
                .iter()
                .map(|line| format!("  {line}"))
                .collect(),
            Self::CodeActiveInNoConfiguration { .. } => Vec::new(),
        }
    }

    #[must_use]
    pub fn issue_type(&self) -> IssueType {
        match self {
            Self::CodeActiveInNoConfiguration { .. } => IssueType::CodeActiveInNoConfiguration,
            Self::DuplicatedFunctionBody { .. } => IssueType::DuplicatedFunctionBody,
            Self::ModuleCycle { .. } => IssueType::ModuleCycle,
            Self::Package(violation) => IssueType::from(violation),
            Self::SharedModuleFile { .. } => IssueType::SharedModuleFile,
            Self::UnresolvedForeignReferences { .. } => IssueType::UnresolvedPath,
            Self::Violation { violation, .. } => IssueType::from(&violation.kind),
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
                configuration_narrowing(analysed, &inclusions.configurations())
            }
            Self::DuplicatedFunctionBody { configurations, .. }
            | Self::UnresolvedForeignReferences { configurations, .. } => {
                configuration_narrowing(analysed, configurations)
            }
            Self::CodeActiveInNoConfiguration { .. } | Self::Package(_) => String::new(),
        };

        iter::once(format!("- {location}: {}{narrowing}", self.message()))
            .chain(
                match &location.position {
                    IssuePosition::Line(LinePosition { source_line, .. }) => {
                        Some(source_line.clone())
                    }
                    IssuePosition::WholeFile => None,
                }
                .into_iter()
                .chain(self.details())
                .map(|line| format!("{CONTINUATION_INDENT}{line}")),
            )
            .collect()
    }

    #[must_use]
    pub fn location(&self) -> IssueLocation {
        match self {
            Self::CodeActiveInNoConfiguration { focus } => IssueLocation::new_position(focus),
            Self::DuplicatedFunctionBody { copies, .. } => {
                IssueLocation::new_position(&copies.first)
            }
            Self::ModuleCycle { witnesses, .. } => {
                IssueLocation::new_position(cycle_site(witnesses))
            }
            Self::Package(violation) => IssueLocation::new_whole_file(violation.file()),
            Self::SharedModuleFile { file, .. } => IssueLocation::new_whole_file(file),
            Self::UnresolvedForeignReferences { sites, .. } => {
                IssueLocation::new_position(&sites.first)
            }
            Self::Violation { violation, .. } => IssueLocation::new_position(&violation.location),
        }
    }

    #[must_use]
    pub fn message(&self) -> String {
        match self {
            Self::CodeActiveInNoConfiguration { .. } => {
                "Code is active in no declared configuration".to_owned()
            }
            Self::DuplicatedFunctionBody { .. } => "Function body is duplicated".to_owned(),
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
            Self::UnresolvedForeignReferences { module, .. } => {
                format!("Path into {module} compiles, but rust-analyzer cannot resolve it")
            }
            Self::Violation { violation, .. } => violation.kind.to_string(),
        }
    }
}
