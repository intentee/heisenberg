use std::fmt;
use std::fmt::Display;
use std::fmt::Formatter;

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum HarnessViolation {
    MustUseOutput { type_name: String },
    TargetWithoutBenchmark,
    UndeterminedOutput { type_name: String },
    UnvettedMethod { method: String },
}

impl Display for HarnessViolation {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::MustUseOutput { type_name } => write!(
                formatter,
                "Measured routine returns {type_name}, a must-use value that criterion drops"
            ),
            Self::TargetWithoutBenchmark => {
                formatter.write_str("Benchmark target registers no criterion benchmark")
            }
            Self::UndeterminedOutput { type_name } => write!(
                formatter,
                "Measured routine returns {type_name}, which leaves undetermined whether criterion drops a must-use value"
            ),
            Self::UnvettedMethod { method } => {
                write!(formatter, "criterion method {method} is not vetted")
            }
        }
    }
}
