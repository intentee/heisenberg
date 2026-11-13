use ra_ap_hir::Function;
use ra_ap_hir::Impl;
use thiserror::Error;

use heisenberg_member_crates::member_crates_error::MemberCratesError;
use heisenberg_source_locator::source_locator_error::SourceLocatorError;

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
        "the standard library lacks the items that the analysis relies on: {}",
        items.join(", ")
    )]
    StandardItemsMissing { items: Vec<&'static str> },
}
