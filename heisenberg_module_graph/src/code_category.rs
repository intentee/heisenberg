#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum CodeCategory {
    Benchmark,
    Production,
    Test,
}
