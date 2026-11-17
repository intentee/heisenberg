use std::fmt;
use std::fmt::Display;
use std::fmt::Formatter;

use crate::constants::SUPPORTED_RUST_VERSION;
use crate::dependency_role::DependencyRole;
use crate::lint_table::LintTable;
use crate::unpinned_requirement::UnpinnedRequirement;

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum ManifestViolation {
    DependencyUnused {
        dependency: String,
        manifest: String,
        role: DependencyRole,
    },
    DocTestsEnabled {
        manifest: String,
        target: String,
    },
    RustVersionBelowSupported {
        rust_version: String,
        table: LintTable,
    },
    TestDependencyInProduction {
        dependency: String,
        manifest: String,
    },
    TestLibraryInProductionPackage {
        dependency: String,
        manifest: String,
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
            Self::DependencyUnused { manifest, .. }
            | Self::DocTestsEnabled { manifest, .. }
            | Self::TestDependencyInProduction { manifest, .. }
            | Self::TestLibraryInProductionPackage { manifest, .. }
            | Self::UnpinnedDependency { manifest, .. } => manifest,
        }
    }
}

impl Display for ManifestViolation {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::DependencyUnused {
                dependency, role, ..
            } => write!(
                formatter,
                "{role} {dependency} is referred to by no code that the role serves"
            ),
            Self::TestDependencyInProduction { dependency, .. } => write!(
                formatter,
                "Dependency {dependency} is referred to only by test, benchmark or example code"
            ),
            Self::TestLibraryInProductionPackage { dependency, .. } => write!(
                formatter,
                "Dev-dependency {dependency} brings a library from outside the workspace into the tests of a production package"
            ),
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
