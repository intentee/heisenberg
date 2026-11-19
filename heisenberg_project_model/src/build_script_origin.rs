#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum BuildScriptOrigin {
    Declared,
    Run,
}
