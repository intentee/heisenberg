use std::fmt;
use std::fmt::Display;
use std::fmt::Formatter;

use crate::constants::CONSTANTS_MODULE_NAME;
use crate::impl_placement::ImplPlacement;

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum PlacementViolation {
    AdditionalPublicItem,
    ImplWithoutLocalType,
    MisplacedImpl(ImplPlacement),
    ModuleNotNamedAfterItsItem {
        expected: String,
        item: String,
        module: String,
    },
    NonConstantInConstantsModule,
    PublicAssociatedConstant,
    PublicConstantOutsideConstantsModule,
}

impl Display for PlacementViolation {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::AdditionalPublicItem => formatter.write_str("Module already has a public item"),
            Self::ImplWithoutLocalType => {
                formatter.write_str("Impl block belongs to no single type or trait of this crate")
            }
            Self::MisplacedImpl(placement) => placement.fmt(formatter),
            Self::ModuleNotNamedAfterItsItem {
                expected,
                item,
                module,
            } => write!(
                formatter,
                "Module is named {module}, but its public item {item} names it {expected}"
            ),
            Self::NonConstantInConstantsModule => {
                formatter.write_str("Constants module holds a public item that is not a constant")
            }
            Self::PublicAssociatedConstant => write!(
                formatter,
                "Public associated constant belongs in crate::{CONSTANTS_MODULE_NAME} as a free constant"
            ),
            Self::PublicConstantOutsideConstantsModule => write!(
                formatter,
                "Public constant belongs in crate::{CONSTANTS_MODULE_NAME}"
            ),
        }
    }
}
