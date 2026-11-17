use std::fmt;
use std::fmt::Display;
use std::fmt::Formatter;

use crate::constants::ERROR_TRAIT_PATH;
use crate::constants::OWNED_BOX_PATH;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum RequiredStandardItem {
    Display,
    ErrorTrait,
    Option,
    OwnedBox,
    Result,
    ToString,
}

impl Display for RequiredStandardItem {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::Display => write!(formatter, "core::fmt::Display"),
            Self::ErrorTrait => write!(formatter, "core::{}", ERROR_TRAIT_PATH.join("::")),
            Self::Option => write!(formatter, "core::option::Option"),
            Self::OwnedBox => write!(formatter, "alloc::{}", OWNED_BOX_PATH.join("::")),
            Self::Result => write!(formatter, "core::result::Result"),
            Self::ToString => write!(formatter, "alloc::string::ToString"),
        }
    }
}
