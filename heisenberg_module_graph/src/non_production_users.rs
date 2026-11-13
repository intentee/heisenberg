use std::fmt;
use std::fmt::Display;
use std::fmt::Formatter;

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum NonProductionUsers {
    Benchmarks,
    Tests,
}

impl Display for NonProductionUsers {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::Benchmarks => formatter
                .write_str("Function is used only by benchmarks, or by benchmarks and tests"),
            Self::Tests => formatter.write_str("Function is used only by tests"),
        }
    }
}
