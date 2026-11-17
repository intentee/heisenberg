#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum DependencyConsumer {
    BuildScript,
    Example,
    Production,
    Test,
}
