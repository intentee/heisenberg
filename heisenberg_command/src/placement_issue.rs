use heisenberg_module_graph::impl_placement::ImplPlacement;
use heisenberg_module_graph::placement_violation::PlacementViolation;

use crate::constants::ADDITIONAL_PUBLIC_ITEM;
use crate::constants::IMPL_WITHOUT_LOCAL_TYPE;
use crate::constants::ITEM_MORE_VISIBLE_THAN_ITS_USES;
use crate::constants::MISPLACED_BLANKET_IMPL;
use crate::constants::MISPLACED_FOREIGN_TYPE_IMPL;
use crate::constants::MISPLACED_IMPL;
use crate::constants::MISPLACED_TRAIT_OBJECT_IMPL;
use crate::constants::MODULE_NOT_NAMED_AFTER_ITS_ITEM;
use crate::constants::NON_CONSTANT_IN_CONSTANTS_MODULE;
use crate::constants::PUBLIC_ASSOCIATED_CONSTANT;
use crate::constants::PUBLIC_CONSTANT_OUTSIDE_CONSTANTS_MODULE;
use crate::issue_text::IssueText;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PlacementIssue {
    AdditionalPublicItem,
    ImplWithoutLocalType,
    ItemMoreVisibleThanItsUses,
    MisplacedBlanketImpl,
    MisplacedForeignTypeImpl,
    MisplacedImpl,
    MisplacedTraitObjectImpl,
    ModuleNotNamedAfterItsItem,
    NonConstantInConstantsModule,
    PublicAssociatedConstant,
    PublicConstantOutsideConstantsModule,
}

impl PlacementIssue {
    #[must_use]
    pub const fn text(self) -> IssueText {
        match self {
            Self::AdditionalPublicItem => ADDITIONAL_PUBLIC_ITEM,
            Self::ImplWithoutLocalType => IMPL_WITHOUT_LOCAL_TYPE,
            Self::ItemMoreVisibleThanItsUses => ITEM_MORE_VISIBLE_THAN_ITS_USES,
            Self::MisplacedBlanketImpl => MISPLACED_BLANKET_IMPL,
            Self::MisplacedForeignTypeImpl => MISPLACED_FOREIGN_TYPE_IMPL,
            Self::MisplacedImpl => MISPLACED_IMPL,
            Self::MisplacedTraitObjectImpl => MISPLACED_TRAIT_OBJECT_IMPL,
            Self::ModuleNotNamedAfterItsItem => MODULE_NOT_NAMED_AFTER_ITS_ITEM,
            Self::NonConstantInConstantsModule => NON_CONSTANT_IN_CONSTANTS_MODULE,
            Self::PublicAssociatedConstant => PUBLIC_ASSOCIATED_CONSTANT,
            Self::PublicConstantOutsideConstantsModule => PUBLIC_CONSTANT_OUTSIDE_CONSTANTS_MODULE,
        }
    }

    #[must_use]
    pub const fn variants() -> [Self; 11] {
        [
            Self::AdditionalPublicItem,
            Self::ImplWithoutLocalType,
            Self::ItemMoreVisibleThanItsUses,
            Self::MisplacedBlanketImpl,
            Self::MisplacedForeignTypeImpl,
            Self::MisplacedImpl,
            Self::MisplacedTraitObjectImpl,
            Self::ModuleNotNamedAfterItsItem,
            Self::NonConstantInConstantsModule,
            Self::PublicAssociatedConstant,
            Self::PublicConstantOutsideConstantsModule,
        ]
    }
}

impl From<&PlacementViolation> for PlacementIssue {
    fn from(violation: &PlacementViolation) -> Self {
        match violation {
            PlacementViolation::AdditionalPublicItem => Self::AdditionalPublicItem,
            PlacementViolation::ImplWithoutLocalType => Self::ImplWithoutLocalType,
            PlacementViolation::MisplacedImpl(ImplPlacement::Blanket { .. }) => {
                Self::MisplacedBlanketImpl
            }
            PlacementViolation::MisplacedImpl(ImplPlacement::ForeignType { .. }) => {
                Self::MisplacedForeignTypeImpl
            }
            PlacementViolation::MisplacedImpl(ImplPlacement::TraitObject { .. }) => {
                Self::MisplacedTraitObjectImpl
            }
            PlacementViolation::MisplacedImpl(ImplPlacement::Type { .. }) => Self::MisplacedImpl,
            PlacementViolation::MoreVisibleThanItsUses { .. } => Self::ItemMoreVisibleThanItsUses,
            PlacementViolation::ModuleNotNamedAfterItsItem { .. } => {
                Self::ModuleNotNamedAfterItsItem
            }
            PlacementViolation::NonConstantInConstantsModule => Self::NonConstantInConstantsModule,
            PlacementViolation::PublicAssociatedConstant => Self::PublicAssociatedConstant,
            PlacementViolation::PublicConstantOutsideConstantsModule => {
                Self::PublicConstantOutsideConstantsModule
            }
        }
    }
}
