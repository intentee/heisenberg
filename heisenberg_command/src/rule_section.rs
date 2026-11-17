#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum RuleSection {
    Analysis,
    Configurations,
    Tools,
    Packages,
    Modules,
    Code,
    Errors,
    Tests,
    Benchmarks,
    Coverage,
}

impl RuleSection {
    #[must_use]
    pub const fn number(self) -> u8 {
        match self {
            Self::Analysis => 1,
            Self::Configurations => 2,
            Self::Tools => 3,
            Self::Packages => 4,
            Self::Modules => 5,
            Self::Code => 6,
            Self::Errors => 7,
            Self::Tests => 8,
            Self::Benchmarks => 9,
            Self::Coverage => 10,
        }
    }

    #[must_use]
    pub const fn title(self) -> &'static str {
        match self {
            Self::Analysis => "Analysable code",
            Self::Configurations => "Declared configurations",
            Self::Tools => "Lints and formatting",
            Self::Packages => "Packages and dependencies",
            Self::Modules => "Modules and items",
            Self::Code => "Names, data and functions",
            Self::Errors => "Error types",
            Self::Tests => "Tests",
            Self::Benchmarks => "Benchmarks",
            Self::Coverage => "Coverage",
        }
    }
}
