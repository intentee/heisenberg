use std::fmt;
use std::fmt::Display;
use std::fmt::Formatter;

use crate::constants::ERROR_TRAIT_PATH;
use crate::constants::INFALLIBLE_PATH;
use crate::constants::STANDARD_SLEEP_MS_PATH;
use crate::constants::STANDARD_SLEEP_PATH;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum RequiredStandardItem {
    Destruct,
    Drop,
    ErrorTrait,
    FromStr,
    Infallible,
    IntoIterator,
    Iterator,
    Result,
    Sleep,
    SleepMs,
    ToString,
}

impl Display for RequiredStandardItem {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::Destruct => write!(formatter, "core::marker::Destruct"),
            Self::Drop => write!(formatter, "core::mem::drop"),
            Self::ErrorTrait => write!(formatter, "core::{}", ERROR_TRAIT_PATH.join("::")),
            Self::FromStr => write!(formatter, "core::str::FromStr"),
            Self::Infallible => write!(formatter, "core::{}", INFALLIBLE_PATH.join("::")),
            Self::IntoIterator => write!(formatter, "core::iter::IntoIterator"),
            Self::Iterator => write!(formatter, "core::iter::Iterator"),
            Self::Result => write!(formatter, "core::result::Result"),
            Self::Sleep => write!(formatter, "std::{}", STANDARD_SLEEP_PATH.join("::")),
            Self::SleepMs => write!(formatter, "std::{}", STANDARD_SLEEP_MS_PATH.join("::")),
            Self::ToString => write!(formatter, "alloc::string::ToString"),
        }
    }
}
