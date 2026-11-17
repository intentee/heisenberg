use std::sync::Arc;

use ra_ap_hir::Crate;
use ra_ap_hir::Function;
use ra_ap_hir::Impl;
use thiserror::Error;

use heisenberg_member_crates::member_crates_error::MemberCratesError;
use heisenberg_module_graph::crate_target::CrateTarget;
use heisenberg_source_locator::source_locator_error::SourceLocatorError;

use crate::required_standard_item::RequiredStandardItem;

#[derive(Debug, Error)]
pub enum ModuleDependenciesError {
    #[error("rust-analyzer gives no name for the crate {0:?}, which declares a Result error type")]
    ErrorTypeCrateWithoutName(Crate),
    #[error("rust-analyzer gives no type for the output of the criterion measurement loop {0:?}")]
    LoopOutputWithoutType(Function),
    #[error(transparent)]
    MemberCrates(#[from] MemberCratesError),
    #[error(
        "`{written}` is a Result that is neither a type alias nor written with its error type, so its error type cannot be named"
    )]
    ResultErrorNotWritten { written: String },
    #[error(
        "rust-analyzer gives no source for the trait method {0:?}, whose signature names the associated types that are Result errors"
    )]
    TraitMethodWithoutSource(Function),
    #[error(transparent)]
    TraitResultErrorsUnavailable(Arc<Self>),
    #[error(
        "the name {literal} of an environment variable cannot be read as a string literal: {reason}"
    )]
    UnreadableEnvironmentVariableName { literal: String, reason: String },
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
