use std::fmt;
use std::fmt::Display;
use std::fmt::Formatter;

use crate::clippy_lint_path::clippy_lint_path;
use crate::constants::CLIPPY_CONFIGURATION_FILE;
use crate::constants::CLIPPY_HIDDEN_CONFIGURATION_FILE;

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
        manifest: String,
        package: String,
    },
    ClippyTestAllowanceMissing {
        file: String,
        setting: String,
    },
    PackageDependencyCycle {
        manifest: String,
        packages: Vec<String>,
    },
    PackageDependsOnItself {
        manifest: String,
        package: String,
    },
    TestsOnlyPackageWithoutTestLibraries {
        destinations: Vec<String>,
        manifest: String,
        package: String,
    },
}

impl PackageViolation {
    #[must_use]
    pub fn file(&self) -> &str {
        match self {
            Self::ClippyConfigurationAmbiguous { file }
            | Self::ClippyTestAllowanceMissing { file, .. }
            | Self::ClippyConfigurationMissing { manifest: file, .. }
            | Self::ClippyLintNotDenied { manifest: file, .. }
            | Self::PackageDependencyCycle { manifest: file, .. }
            | Self::PackageDependsOnItself { manifest: file, .. }
            | Self::TestsOnlyPackageWithoutTestLibraries { manifest: file, .. } => file,
        }
    }

    #[must_use]
    pub const fn issue_type(&self) -> &'static str {
        match self {
            Self::ClippyConfigurationAmbiguous { .. } => "ClippyConfigurationAmbiguous",
            Self::ClippyConfigurationMissing { .. } => "ClippyConfigurationMissing",
            Self::ClippyLintNotDenied { .. } => "ClippyLintNotDenied",
            Self::ClippyTestAllowanceMissing { .. } => "ClippyTestAllowanceMissing",
            Self::PackageDependencyCycle { .. } => "PackageDependencyCycle",
            Self::PackageDependsOnItself { .. } => "PackageDependsOnItself",
            Self::TestsOnlyPackageWithoutTestLibraries { .. } => {
                "TestsOnlyPackageWithoutTestLibraries"
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
            Self::ClippyLintNotDenied { lint, package, .. } => write!(
                formatter,
                "Package {package} does not deny {}",
                clippy_lint_path(lint)
            ),
            Self::ClippyTestAllowanceMissing { setting, .. } => {
                write!(
                    formatter,
                    "Clippy configuration does not set {setting} = true"
                )
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
            Self::TestsOnlyPackageWithoutTestLibraries {
                destinations,
                package,
                ..
            } => write!(
                formatter,
                "Package {package} holds only tests that need no test library; move them into the tests directory of {}",
                destinations.join(", ")
            ),
        }
    }
}
