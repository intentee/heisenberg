use clap::ValueEnum;

use heisenberg_coverage_measurement::measured_code::MeasuredCode;
use heisenberg_coverage_measurement::measured_coverage::MeasuredCoverage;
use heisenberg_coverage_measurement::measured_function::MeasuredFunction;
use heisenberg_coverage_measurement::measured_item::MeasuredItem;
use heisenberg_coverage_measurement::measured_region::MeasuredRegion;
use heisenberg_module_cycles::non_empty::NonEmpty;
use heisenberg_module_graph::source_location::SourceLocation;
use heisenberg_project_model::configuration_list::ConfigurationList;
use heisenberg_project_model::constants::SUPPORTED_RUST_VERSION;
use heisenberg_project_model::run_scope::RunScope;

use crate::analysis_contexts::AnalysisContexts;
use crate::compliance_summary::ComplianceSummary;
use crate::configuration_coverage::ConfigurationCoverage;
use crate::issue::Issue;
use crate::issue_report::IssueReport;
use crate::issue_type::IssueType;
use crate::rendered_report::RenderedReport;

fn function_issue(MeasuredFunction { location, regions }: &MeasuredFunction) -> Option<Issue> {
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
        unexecuted_regions(regions).map(|unexecuted| Issue::PartiallyExecutedFunction {
            function: location.clone(),
            unexecuted,
        })
    }
}

fn item_issue(MeasuredItem { location, regions }: &MeasuredItem) -> Option<Issue> {
    unexecuted_regions(regions).map(|unexecuted| Issue::PartiallyExecutedItem {
        item: location.clone(),
        unexecuted,
    })
}

fn unexecuted_regions(regions: &[MeasuredRegion]) -> Option<NonEmpty<SourceLocation>> {
    let mut unexecuted = regions
        .iter()
        .filter(|MeasuredRegion { executed, .. }| !executed)
        .map(|MeasuredRegion { location, .. }| location.clone());

    unexecuted.next().map(|first| NonEmpty {
        first,
        others: unexecuted.collect(),
    })
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

        footer.extend(
            ComplianceSummary::new_full(
                &IssueType::value_variants()
                    .iter()
                    .copied()
                    .filter(|issue_type| issue_type.measures_coverage())
                    .collect::<Vec<IssueType>>(),
            )
            .summary_lines(),
        );

        IssueReport {
            contexts: &AnalysisContexts::default(),
            footer,
            issues: measurements
                .iter()
                .flat_map(
                    |ConfigurationCoverage {
                         coverage:
                             MeasuredCoverage {
                                 code: MeasuredCode { functions, items },
                                 ..
                             },
                         ..
                     }| {
                        functions
                            .iter()
                            .filter_map(function_issue)
                            .chain(items.iter().filter_map(item_issue))
                    },
                )
                .collect(),
        }
        .render()
    }
}
