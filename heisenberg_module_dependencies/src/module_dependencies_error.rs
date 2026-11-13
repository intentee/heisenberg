use ra_ap_hir::Function;
use ra_ap_hir::Impl;
use thiserror::Error;

use heisenberg_member_crates::member_crates_error::MemberCratesError;
use heisenberg_module_graph::crate_target::CrateTarget;
use heisenberg_source_locator::source_locator_error::SourceLocatorError;

use crate::required_standard_item::RequiredStandardItem;

#[derive(Debug, Error)]
pub enum ModuleDependenciesError {
    #[error("rust-analyzer gives no type for the output of the criterion measurement loop {0:?}")]
    LoopOutputWithoutType(Function),
    #[error(transparent)]
    MemberCrates(#[from] MemberCratesError),
    #[error("rust-analyzer gives no source for the impl {0:?} that a `Self` resolves to")]
    SelfTypeWithoutSource(Impl),
    #[error(transparent)]
    SourceLocator(#[from] SourceLocatorError),
    #[error(
        "the standard library of the crate target `{crate_target}` lacks the items that the analysis relies on: {}",
        items.iter().map(ToString::to_string).collect::<Vec<String>>().join(", ")
    )]
    StandardItemsMissing {
        crate_target: CrateTarget,
        items: Vec<RequiredStandardItem>,
    },
}
