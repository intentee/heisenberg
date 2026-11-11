use ra_ap_hir::Impl;
use thiserror::Error;

use heisenberg_member_crates::member_crates_error::MemberCratesError;
use heisenberg_source_locator::source_locator_error::SourceLocatorError;

#[derive(Debug, Error)]
pub enum ModuleDependenciesError {
    #[error(transparent)]
    MemberCrates(#[from] MemberCratesError),
    #[error("rust-analyzer gives no source for the impl {0:?} that a `Self` resolves to")]
    SelfTypeWithoutSource(Impl),
    #[error(transparent)]
    SourceLocator(#[from] SourceLocatorError),
}
