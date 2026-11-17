use std::fmt;
use std::fmt::Display;
use std::fmt::Formatter;

use crate::constants::SUPPORTED_RUST_VERSION;
use crate::lint_table::LintTable;
use crate::unpinned_requirement::UnpinnedRequirement;

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum ManifestViolation {
    DocTestsEnabled {
        manifest: String,
        target: String,
    },
    RustVersionBelowSupported {
        rust_version: String,
        table: LintTable,
    },
    UnpinnedDependency {
        dependency: String,
        manifest: String,
        requirement: UnpinnedRequirement,
    },
}

impl ManifestViolation {
    #[must_use]
    pub fn manifest(&self) -> &str {
        match self {
            Self::RustVersionBelowSupported { table, .. } => table.manifest(),
            Self::DocTestsEnabled { manifest, .. } | Self::UnpinnedDependency { manifest, .. } => {
                manifest
            }
        }
    }
}

impl Display for ManifestViolation {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::DocTestsEnabled { target, .. } => write!(
                formatter,
                "Target {target} has doc tests enabled, which cargo test runs and no analysis reads"
            ),
            Self::RustVersionBelowSupported {
                rust_version,
                table,
            } => write!(
                formatter,
                "{table} declares rust-version {rust_version}, which is older than the supported Rust {SUPPORTED_RUST_VERSION}"
            ),
            Self::UnpinnedDependency {
                dependency,
                requirement,
                ..
            } => write!(formatter, "Dependency {dependency} {requirement}"),
        }
    }
}
