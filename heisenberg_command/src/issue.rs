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
use heisenberg_project_model::build_script_origin::BuildScriptOrigin;
use heisenberg_project_model::build_script_variables::BuildScriptVariables;
use heisenberg_project_model::configuration::Configuration;
use heisenberg_project_model::confined_reach::ConfinedReach;
use heisenberg_project_model::confinement_breach::ConfinementBreach;
use heisenberg_project_model::package_violation::PackageViolation;

use crate::analysis_context::AnalysisContext;
use crate::analysis_contexts::AnalysisContexts;
use crate::configuration_narrowing::ConfigurationNarrowing;
use crate::cycle_shape::CycleShape;
use crate::issue_location::IssueLocation;
use crate::issue_position::IssuePosition;
use crate::issue_type::IssueType;
use crate::module_inclusion::ModuleInclusion;
use crate::narrowing::Narrowing;
use crate::shared_file_inclusions::SharedFileInclusions;
use crate::violation_occurrences::ViolationOccurrences;
use crate::violation_report::ViolationReport;
use crate::violation_variant::ViolationVariant;

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
    BuildScriptVariables(BuildScriptVariables),
    CodeActiveInNoConfiguration {
        focus: SourceLocation,
    },
    ConfinedPackageReached {
        breach: ConfinementBreach,
    },
    DuplicatedFunctionBody {
        configurations: BTreeSet<Configuration>,
        copies: NonEmpty<SourceLocation>,
    },
    FunctionWithoutCoverageData {
        function: SourceLocation,
    },
    ModuleCycle {
        container: ModulePath,
        reported: BTreeSet<AnalysisContext>,
        witnesses: NonEmpty<CycleWitness>,
    },
    Package(PackageViolation),
    PartiallyExecutedFunction {
        function: SourceLocation,
        unexecuted: NonEmpty<SourceLocation>,
    },
    SharedModuleFile {
        file: String,
        inclusions: SharedFileInclusions,
    },
    UnexecutedFunction {
        function: SourceLocation,
    },
    UnexecutedRegion {
        region: SourceLocation,
    },
    UnresolvedForeignReferences {
        configurations: BTreeSet<Configuration>,
        module: ForeignModule,
        sites: NonEmpty<SourceLocation>,
    },
    Violation(ViolationReport),
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
    pub const fn new_violation(report: ViolationReport) -> Self {
        Self::Violation(report)
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
            Self::BuildScriptVariables(BuildScriptVariables { variables, .. }) => variables
                .iter()
                .map(|variable| format!("  sets {variable}"))
                .collect(),
            Self::ConfinedPackageReached {
                breach: ConfinementBreach { path, .. },
                ..
            } => vec![format!("  {}", path.join(" -> "))],
            Self::DuplicatedFunctionBody { copies, .. } => copies
                .others
                .iter()
                .map(|copy| format!("  duplicate at {copy}"))
                .collect(),
            Self::PartiallyExecutedFunction { unexecuted, .. } => unexecuted
                .iter()
                .map(|region| format!("  not executed at {region}"))
                .collect(),
            Self::SharedModuleFile { inclusions, .. } => inclusions.lines(),
            Self::UnresolvedForeignReferences { sites, .. } => sites
                .others
                .iter()
                .map(|site| format!("  also unresolved at {site}"))
                .collect(),
            Self::Violation(ViolationReport {
                occurrences: ViolationOccurrences { folded, .. },
                ..
            }) => folded
                .iter()
                .map(|finding| format!("  {finding}"))
                .collect(),
            Self::Package(violation) => violation
                .details()
                .iter()
                .map(|line| format!("  {line}"))
                .collect(),
            Self::CodeActiveInNoConfiguration { .. }
            | Self::FunctionWithoutCoverageData { .. }
            | Self::UnexecutedFunction { .. }
            | Self::UnexecutedRegion { .. } => Vec::new(),
        }
    }

    #[must_use]
    pub fn issue_type(&self) -> IssueType {
        match self {
            Self::BuildScriptVariables(_) => IssueType::BuildScriptEnvironmentVariable,
            Self::CodeActiveInNoConfiguration { .. } => IssueType::CodeActiveInNoConfiguration,
            Self::ConfinedPackageReached { .. } => IssueType::ConfinedPackageReachedOutsideItsUsers,
            Self::DuplicatedFunctionBody { .. } => IssueType::DuplicatedFunctionBody,
            Self::FunctionWithoutCoverageData { .. } => IssueType::CodeWithoutCoverageData,
            Self::ModuleCycle { .. } => IssueType::ModuleCycle,
            Self::Package(violation) => IssueType::from(violation),
            Self::PartiallyExecutedFunction { .. }
            | Self::UnexecutedFunction { .. }
            | Self::UnexecutedRegion { .. } => IssueType::UncoveredCode,
            Self::SharedModuleFile { .. } => IssueType::SharedModuleFile,
            Self::UnresolvedForeignReferences { .. } => IssueType::UnresolvedPath,
            Self::Violation(ViolationReport { violation, .. }) => IssueType::from(&violation.kind),
        }
    }

    #[must_use]
    pub fn lines(
        &self,
        AnalysisContexts {
            configurations: analysed_configurations,
            files,
        }: &AnalysisContexts,
    ) -> Vec<String> {
        let location = self.location();
        let analysed = files.get(&location.file).into_iter().flatten();
        let mut variant_lines: Vec<String> = Vec::new();
        let narrowing = match self {
            Self::ModuleCycle { reported, .. } => Narrowing {
                base: &analysed.chain(reported).cloned().collect(),
                reported,
            }
            .to_string(),
            Self::Violation(ViolationReport {
                occurrences: ViolationOccurrences { reported, .. },
                variants,
                ..
            }) => {
                let base: BTreeSet<AnalysisContext> = analysed
                    .chain(reported)
                    .chain(
                        variants
                            .iter()
                            .flat_map(|ViolationVariant { reported, .. }| reported),
                    )
                    .cloned()
                    .collect();

                variant_lines.extend(variants.iter().map(|ViolationVariant { kind, reported }| {
                    format!(
                        "  {kind}{}",
                        Narrowing {
                            base: &base,
                            reported,
                        }
                    )
                }));

                Narrowing {
                    base: &base,
                    reported,
                }
                .to_string()
            }
            Self::BuildScriptVariables(BuildScriptVariables { configurations, .. })
            | Self::ConfinedPackageReached {
                breach: ConfinementBreach { configurations, .. },
            } => ConfigurationNarrowing {
                all: analysed_configurations,
                configurations,
            }
            .to_string(),
            Self::SharedModuleFile { inclusions, .. } => {
                configuration_narrowing(analysed, &inclusions.configurations())
            }
            Self::DuplicatedFunctionBody { configurations, .. }
            | Self::UnresolvedForeignReferences { configurations, .. } => {
                configuration_narrowing(analysed, configurations)
            }
            Self::CodeActiveInNoConfiguration { .. }
            | Self::FunctionWithoutCoverageData { .. }
            | Self::Package(_)
            | Self::PartiallyExecutedFunction { .. }
            | Self::UnexecutedFunction { .. }
            | Self::UnexecutedRegion { .. } => String::new(),
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
                .chain(variant_lines)
                .chain(self.details())
                .map(|line| format!("{CONTINUATION_INDENT}{line}")),
            )
            .collect()
    }

    #[must_use]
    pub fn location(&self) -> IssueLocation {
        match self {
            Self::BuildScriptVariables(BuildScriptVariables { file, .. }) => {
                IssueLocation::new_whole_file(file)
            }
            Self::CodeActiveInNoConfiguration { focus } => IssueLocation::new_position(focus),
            Self::ConfinedPackageReached {
                breach:
                    ConfinementBreach {
                        reach: ConfinedReach { manifest, .. },
                        ..
                    },
            } => IssueLocation::new_whole_file(manifest),
            Self::DuplicatedFunctionBody { copies, .. } => {
                IssueLocation::new_position(&copies.first)
            }
            Self::FunctionWithoutCoverageData { function }
            | Self::PartiallyExecutedFunction { function, .. }
            | Self::UnexecutedFunction { function } => IssueLocation::new_position(function),
            Self::ModuleCycle { witnesses, .. } => {
                IssueLocation::new_position(cycle_site(witnesses))
            }
            Self::Package(violation) => IssueLocation::new_whole_file(violation.file()),
            Self::SharedModuleFile { file, .. } => IssueLocation::new_whole_file(file),
            Self::UnexecutedRegion { region } => IssueLocation::new_position(region),
            Self::UnresolvedForeignReferences { sites, .. } => {
                IssueLocation::new_position(&sites.first)
            }
            Self::Violation(ViolationReport { violation, .. }) => {
                IssueLocation::new_position(&violation.location)
            }
        }
    }

    #[must_use]
    pub fn message(&self) -> String {
        match self {
            Self::BuildScriptVariables(BuildScriptVariables {
                origin: BuildScriptOrigin::Declared,
                package,
                ..
            }) => format!(
                "Declared output of the build script of {package} sets environment variables for the compiler"
            ),
            Self::BuildScriptVariables(BuildScriptVariables {
                origin: BuildScriptOrigin::Run,
                package,
                ..
            }) => format!("Build script of {package} sets environment variables for the compiler"),
            Self::CodeActiveInNoConfiguration { .. } => {
                "Code is active in no declared configuration".to_owned()
            }
            Self::ConfinedPackageReached {
                breach:
                    ConfinementBreach {
                        reach:
                            ConfinedReach {
                                member, package, ..
                            },
                        ..
                    },
            } => format!(
                "Package {member} depends on confined package {package} without being one of its users"
            ),
            Self::DuplicatedFunctionBody { .. } => "Function body is duplicated".to_owned(),
            Self::FunctionWithoutCoverageData { .. } => {
                "The coverage export holds no data for the function".to_owned()
            }
            Self::ModuleCycle { container, .. } => {
                format!("Parts of {container} depend on each other")
            }
            Self::Package(violation) => violation.to_string(),
            Self::PartiallyExecutedFunction { .. } => {
                "No test executes parts of the function".to_owned()
            }
            Self::SharedModuleFile {
                inclusions: SharedFileInclusions { inclusions, .. },
                ..
            } => {
                format!("Module file belongs to {} modules", inclusions.len())
            }
            Self::UnexecutedFunction { .. } => "No test executes the function".to_owned(),
            Self::UnexecutedRegion { .. } => "No test executes the code".to_owned(),
            Self::UnresolvedForeignReferences { module, .. } => {
                format!("Path into {module} compiles, but rust-analyzer cannot resolve it")
            }
            Self::Violation(ViolationReport { violation, .. }) => violation.kind.to_string(),
        }
    }
}
