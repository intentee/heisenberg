use crate::constants::LEFT_OUT_CONFIGURATION;
use crate::host_analysis_line::host_analysis_line;

#[derive(Clone, Copy)]
pub enum NextestPresenceEvaluation {
    Evaluated,
    NotEvaluated,
}

impl NextestPresenceEvaluation {
    #[must_use]
    pub fn host_footer(self, crate_targets: usize, modules: usize) -> String {
        format!(
            "{}Not analysed: {LEFT_OUT_CONFIGURATION}\n{}",
            host_analysis_line(crate_targets, modules),
            self.partial_run_footer_end()
        )
    }

    #[must_use]
    pub fn partial_run_footer_end(self) -> String {
        format!(
            "Not evaluated in a partial run: CodeActiveInNoConfiguration, ConfinedPackageUserNotReachingIt, {}UnreachableTestFile, the empty-library checks of TestsOnlyPackageWithoutTestLibraries and BenchmarksOnlyPackageWithoutLibraries\n\
            Precondition: rustc 1.99.1 compiles the workspace in every analysed configuration\n",
            match self {
                Self::Evaluated => "",
                Self::NotEvaluated => "NextestConfigurationMissing, UnneededNextestConfiguration, ",
            }
        )
    }
}
