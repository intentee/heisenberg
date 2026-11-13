#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct TargetManifestFlags {
    pub benched: bool,
    pub harness: bool,
    pub tested: bool,
}
