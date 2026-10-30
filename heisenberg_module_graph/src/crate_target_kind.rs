#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum CrateTargetKind {
    Benchmark,
    Binary,
    BuildScript,
    Example,
    IntegrationTest,
    Library,
}
