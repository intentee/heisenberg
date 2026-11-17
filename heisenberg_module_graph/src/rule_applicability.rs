use std::fmt;
use std::fmt::Display;
use std::fmt::Formatter;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum RuleApplicability {
    AllCode,
    ProductionCode,
}

impl Display for RuleApplicability {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
        formatter.write_str(match self {
            Self::AllCode => "all code",
            Self::ProductionCode => "production code, not to test code",
        })
    }
}
