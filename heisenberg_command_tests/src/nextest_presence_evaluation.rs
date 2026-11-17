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
            "Not evaluated in a partial run: AttributeAppliedInNoConfiguration, CodeActiveInNoConfiguration, ConfinedPackageUserNotReachingIt, ItemMoreVisibleThanItsUses, {}TestDependencyInProduction, TestLibraryInProductionPackage, {}UnreachableSourceFile, UnusedDependency, VacuousNegatedConfiguration\n\
            Evaluated in part in a partial run: BenchmarksOnlyPackageWithoutLibraries, TestsOnlyPackageWithoutTestLibraries\n\
            Precondition: rustc 1.99.1 compiles the workspace in every analysed configuration\n\
            Rules: {} evaluated, 2 evaluated in part, {} not evaluated\n\
            Heisenberg 0.1.0\n",
            match self {
                Self::Evaluated => "",
                Self::NotEvaluated => "NextestConfigurationMissing, ",
            },
            match self {
                Self::Evaluated => "",
                Self::NotEvaluated => "UnneededNextestConfiguration, ",
            },
            match self {
                Self::Evaluated => 104,
                Self::NotEvaluated => 102,
            },
            match self {
                Self::Evaluated => 9,
                Self::NotEvaluated => 11,
            }
        )
    }
}
