use crate::source_location::SourceLocation;

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct FunctionBody {
    pub fingerprint: Vec<String>,
    pub location: SourceLocation,
}
