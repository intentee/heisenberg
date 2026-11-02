use cargo::workspace::dependency::DepKind;
use cargo_platform::Platform;

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct DependencyKindDocument {
    pub kind: DepKind,
    pub platform: Option<Platform>,
}
