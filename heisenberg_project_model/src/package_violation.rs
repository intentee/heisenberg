use std::fmt;
use std::fmt::Display;
use std::fmt::Formatter;

use crate::clippy_lint_path::clippy_lint_path;
use crate::constants::CLIPPY_CONFIGURATION_FILE;
use crate::constants::CLIPPY_HIDDEN_CONFIGURATION_FILE;
use crate::lint_table::LintTable;
use crate::potential_owner::PotentialOwner;

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum PackageViolation {
    ClippyConfigurationAmbiguous {
        file: String,
    },
    ClippyConfigurationMissing {
        manifest: String,
        package: String,
    },
    ClippyLintNotDenied {
        lint: String,
        table: LintTable,
    },
    ClippyTestAllowanceMissing {
        file: String,
        setting: String,
    },
    LintAllowedInManifest {
        lint: String,
        table: LintTable,
        tool: String,
    },
    PackageDependencyCycle {
        manifest: String,
        packages: Vec<String>,
    },
    PackageDependsOnItself {
        manifest: String,
        package: String,
    },
    PublishableTestCrate {
        manifest: String,
        package: String,
    },
    TestCrateDeclaredTwice {
        manifest: String,
        package: String,
    },
    TestCrateNotAMember {
        manifest: String,
        package: String,
    },
    TestCrateUsedByProduction {
        dependent: String,
        manifest: String,
        package: String,
    },
    TestsOnlyPackageWithoutTestLibraries {
        manifest: String,
        package: String,
        potential_owners: Vec<PotentialOwner>,
    },
}

impl PackageViolation {
    #[must_use]
    pub fn details(&self) -> Vec<String> {
        match self {
            Self::TestsOnlyPackageWithoutTestLibraries {
                potential_owners, ..
            } => potential_owners
                .iter()
                .map(PotentialOwner::to_string)
                .collect(),
            Self::ClippyConfigurationAmbiguous { .. }
            | Self::ClippyConfigurationMissing { .. }
            | Self::ClippyLintNotDenied { .. }
            | Self::ClippyTestAllowanceMissing { .. }
            | Self::LintAllowedInManifest { .. }
            | Self::PackageDependencyCycle { .. }
            | Self::PackageDependsOnItself { .. }
            | Self::PublishableTestCrate { .. }
            | Self::TestCrateDeclaredTwice { .. }
            | Self::TestCrateNotAMember { .. }
            | Self::TestCrateUsedByProduction { .. } => Vec::new(),
        }
    }

    #[must_use]
    pub fn file(&self) -> &str {
        match self {
            Self::ClippyConfigurationAmbiguous { file }
            | Self::ClippyTestAllowanceMissing { file, .. }
            | Self::ClippyConfigurationMissing { manifest: file, .. }
            | Self::PackageDependencyCycle { manifest: file, .. }
            | Self::PackageDependsOnItself { manifest: file, .. }
            | Self::PublishableTestCrate { manifest: file, .. }
            | Self::TestCrateDeclaredTwice { manifest: file, .. }
            | Self::TestCrateNotAMember { manifest: file, .. }
            | Self::TestCrateUsedByProduction { manifest: file, .. }
            | Self::TestsOnlyPackageWithoutTestLibraries { manifest: file, .. } => file,
            Self::ClippyLintNotDenied { table, .. } | Self::LintAllowedInManifest { table, .. } => {
                table.manifest()
            }
        }
    }
}

impl Display for PackageViolation {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::ClippyConfigurationAmbiguous { .. } => write!(
                formatter,
                "Directory holds both {CLIPPY_CONFIGURATION_FILE} and {CLIPPY_HIDDEN_CONFIGURATION_FILE}"
            ),
            Self::ClippyConfigurationMissing { package, .. } => write!(
                formatter,
                "Package {package} has no {CLIPPY_CONFIGURATION_FILE} or {CLIPPY_HIDDEN_CONFIGURATION_FILE} within the workspace"
            ),
            Self::ClippyLintNotDenied { lint, table } => {
                write!(
                    formatter,
                    "{table} does not deny {}",
                    clippy_lint_path(lint)
                )
            }
            Self::ClippyTestAllowanceMissing { setting, .. } => {
                write!(
                    formatter,
                    "Clippy configuration does not set {setting} = true"
                )
            }
            Self::LintAllowedInManifest { lint, table, tool } => {
                write!(formatter, "{table} allows {lint} in its {tool} lints")
            }
            Self::PackageDependencyCycle { packages, .. } => write!(
                formatter,
                "Packages {} depend on each other through dev-dependencies",
                packages.join(", ")
            ),
            Self::PackageDependsOnItself { package, .. } => write!(
                formatter,
                "Package {package} depends on itself through dev-dependencies"
            ),
            Self::PublishableTestCrate { package, .. } => write!(
                formatter,
                "Package {package} is declared a test crate but can be published"
            ),
            Self::TestCrateDeclaredTwice { package, .. } => write!(
                formatter,
                "Package {package} is declared a test crate more than once"
            ),
            Self::TestCrateNotAMember { package, .. } => write!(
                formatter,
                "Package {package} is declared a test crate but is not a workspace member"
            ),
            Self::TestCrateUsedByProduction {
                dependent, package, ..
            } => write!(
                formatter,
                "Package {dependent} depends on test crate {package} through a normal or build dependency"
            ),
            Self::TestsOnlyPackageWithoutTestLibraries { package, .. } => write!(
                formatter,
                "Package {package} holds only tests and no test library of its own, so the package its tests verify must own them"
            ),
        }
    }
}
