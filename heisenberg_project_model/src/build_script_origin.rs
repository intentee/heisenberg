#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum BuildScriptOrigin {
    Declared { manifest: String },
    Run { script: String },
}
