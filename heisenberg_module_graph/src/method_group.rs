use std::fmt;
use std::fmt::Display;
use std::fmt::Formatter;

use crate::method_visibility::MethodVisibility;

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum MethodGroup {
    Constructor(MethodVisibility),
    Method(MethodVisibility),
}

impl Display for MethodGroup {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::Constructor(visibility) => write!(formatter, "{visibility} constructors"),
            Self::Method(visibility) => write!(formatter, "{visibility} methods"),
        }
    }
}
