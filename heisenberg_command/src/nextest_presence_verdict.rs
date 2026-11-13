use heisenberg_module_graph::crate_module_graph::CrateModuleGraph;
use heisenberg_project_model::nextest_presence::NextestPresence;
use heisenberg_project_model::nextest_presence_violation::NextestPresenceViolation;
use heisenberg_project_model::package_violation::PackageViolation;
use heisenberg_project_model::run_scope::RunScope;

use crate::configuration_analysis::ConfigurationAnalysis;

pub enum NextestPresenceVerdict {
    Compliant,
    NotEvaluated,
    Violation(PackageViolation),
}

impl NextestPresenceVerdict {
    #[must_use]
    pub fn new(
        presence: &NextestPresence,
        analyses: &[ConfigurationAnalysis],
        scope: &RunScope,
    ) -> Self {
        let runs_libtest_tests = analyses.iter().any(|ConfigurationAnalysis { graphs, .. }| {
            graphs.iter().any(
                |CrateModuleGraph {
                     runs_libtest_tests, ..
                 }| *runs_libtest_tests,
            )
        });

        match presence {
            NextestPresence::Missing { manifest } if runs_libtest_tests => Self::Violation(
                PackageViolation::NextestPresence(NextestPresenceViolation::Missing {
                    manifest: manifest.clone(),
                }),
            ),
            NextestPresence::Missing { .. } | NextestPresence::Present { .. }
                if !runs_libtest_tests && scope.is_partial() =>
            {
                Self::NotEvaluated
            }
            NextestPresence::Present { file } if !runs_libtest_tests => {
                Self::Violation(PackageViolation::NextestPresence(
                    NextestPresenceViolation::Unneeded { file: file.clone() },
                ))
            }
            NextestPresence::Missing { .. } | NextestPresence::Present { .. } => Self::Compliant,
        }
    }
}
