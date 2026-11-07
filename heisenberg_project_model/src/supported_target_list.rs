use std::fmt;
use std::fmt::Display;
use std::fmt::Formatter;

use crate::constants::SUPPORTED_TARGETS;
use crate::supported_target::SupportedTarget;

pub struct SupportedTargetList;

impl Display for SupportedTargetList {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
        formatter.write_str(&SUPPORTED_TARGETS.map(SupportedTarget::triple).join(", "))
    }
}
