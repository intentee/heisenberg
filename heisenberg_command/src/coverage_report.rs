use heisenberg_coverage_measurement::measured_code::MeasuredCode;
use heisenberg_coverage_measurement::measured_coverage::MeasuredCoverage;
use heisenberg_coverage_measurement::measured_function::MeasuredFunction;
use heisenberg_coverage_measurement::measured_region::MeasuredRegion;
use heisenberg_module_cycles::non_empty::NonEmpty;
use heisenberg_module_graph::source_location::SourceLocation;
use heisenberg_project_model::configuration_list::ConfigurationList;
use heisenberg_project_model::run_scope::RunScope;

use crate::analysis_contexts::AnalysisContexts;
use crate::configuration_coverage::ConfigurationCoverage;
use crate::constants::SUPPORTED_RUST_VERSION;
use crate::issue::Issue;
use crate::issue_report::IssueReport;
use crate::rendered_report::RenderedReport;

fn function_issue(MeasuredFunction { location, regions }: &MeasuredFunction) -> Option<Issue> {
    let mut unexecuted = regions
        .iter()
        .filter(|MeasuredRegion { executed, .. }| !executed)
        .map(|MeasuredRegion { location, .. }| location.clone());

    if regions.is_empty() {
        Some(Issue::FunctionWithoutCoverageData {
            function: location.clone(),
        })
    } else if regions
        .iter()
        .all(|MeasuredRegion { executed, .. }| !executed)
    {
        Some(Issue::UnexecutedFunction {
            function: location.clone(),
        })
    } else {
        unexecuted
            .next()
            .map(|first| Issue::PartiallyExecutedFunction {
                function: location.clone(),
                unexecuted: NonEmpty {
                    first,
                    others: unexecuted.collect::<Vec<SourceLocation>>(),
                },
            })
    }
}

pub struct CoverageReport {
    pub measurements: Vec<ConfigurationCoverage>,
    pub scope: RunScope,
}

impl CoverageReport {
    #[must_use]
    pub fn render(&self) -> RenderedReport {
        let Self {
            measurements,
            scope,
        } = self;
        let mut footer: Vec<String> = measurements
            .iter()
            .map(
                |ConfigurationCoverage {
                     configuration,
                     coverage:
                         MeasuredCoverage {
                             code: MeasuredCode { functions, .. },
                             crate_targets,
                             regions,
                         },
                 }| {
                    format!(
                        "Measured on {}: {crate_targets} crate targets, {} functions, {regions} regions",
                        configuration.name,
                        functions.len(),
                    )
                },
            )
            .collect();

        footer.extend(scope.is_partial().then(|| {
            format!(
                "Not measured: {}",
                ConfigurationList {
                    configurations: &scope.left_out_configurations,
                }
            )
        }));

        footer.push(format!(
            "Precondition: rustc {SUPPORTED_RUST_VERSION} compiles the workspace in the measured configuration, whose tests the coverage export records"
        ));

        IssueReport {
            contexts: &AnalysisContexts::default(),
            footer,
            issues: measurements
                .iter()
                .flat_map(
                    |ConfigurationCoverage {
                         coverage:
                             MeasuredCoverage {
                                 code:
                                     MeasuredCode {
                                         functions,
                                         standalone_regions,
                                     },
                                 ..
                             },
                         ..
                     }| {
                        functions.iter().filter_map(function_issue).chain(
                            standalone_regions
                                .iter()
                                .filter(|MeasuredRegion { executed, .. }| !executed)
                                .map(|MeasuredRegion { location, .. }| Issue::UnexecutedRegion {
                                    region: location.clone(),
                                }),
                        )
                    },
                )
                .collect(),
        }
        .render()
    }
}
