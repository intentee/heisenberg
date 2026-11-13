use std::fmt;
use std::fmt::Display;
use std::fmt::Formatter;

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum BenchmarkTargetViolation {
    ExcludedFromBenchmarks { manifest: String, target: String },
    RunByTests { manifest: String, target: String },
    WithLibtestHarness { manifest: String, target: String },
}

impl BenchmarkTargetViolation {
    #[must_use]
    pub fn manifest(&self) -> &str {
        match self {
            Self::ExcludedFromBenchmarks { manifest, .. }
            | Self::RunByTests { manifest, .. }
            | Self::WithLibtestHarness { manifest, .. } => manifest,
        }
    }
}

impl Display for BenchmarkTargetViolation {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::ExcludedFromBenchmarks { target, .. } => write!(
                formatter,
                "Benchmark target {target} sets bench = false, so cargo bench never runs it"
            ),
            Self::RunByTests { target, .. } => write!(
                formatter,
                "Benchmark target {target} sets test = true, so cargo test runs it"
            ),
            Self::WithLibtestHarness { target, .. } => write!(
                formatter,
                "Benchmark target {target} uses libtest's harness, which runs no benchmark on stable Rust"
            ),
        }
    }
}
