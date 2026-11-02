use std::fmt;
use std::fmt::Display;
use std::fmt::Formatter;

use crate::clippy_lint_path::clippy_lint_path;
use crate::constants::CLIPPY_CONFIGURATION_FILE;
use crate::constants::CLIPPY_HIDDEN_CONFIGURATION_FILE;
use crate::lint_table::LintTable;

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
            | Self::PackageDependencyCycle { manifest: file, .. }
            | Self::PackageDependsOnItself { manifest: file, .. }
            | Self::TestsOnlyPackageWithoutTestLibraries { manifest: file, .. } => file,
            Self::ClippyLintNotDenied { table, .. } | Self::LintAllowedInManifest { table, .. } => {
                table.manifest()
            }
        }
    }

    #[must_use]
    pub const fn issue_type(&self) -> &'static str {
        match self {
            Self::ClippyConfigurationAmbiguous { .. } => "ClippyConfigurationAmbiguous",
            Self::ClippyConfigurationMissing { .. } => "ClippyConfigurationMissing",
            Self::ClippyLintNotDenied { .. } => "ClippyLintNotDenied",
            Self::ClippyTestAllowanceMissing { .. } => "ClippyTestAllowanceMissing",
            Self::LintAllowedInManifest { .. } => "LintAllowedInManifest",
            Self::PackageDependencyCycle { .. } => "PackageDependencyCycle",
            Self::PackageDependsOnItself { .. } => "PackageDependsOnItself",
            Self::TestsOnlyPackageWithoutTestLibraries { .. } => {
                "TestsOnlyPackageWithoutTestLibraries"
            }
        }
    }

    #[must_use]
    pub const fn purpose(&self) -> &'static str {
        match self {
            Self::ClippyConfigurationAmbiguous { .. } => {
                "Clippy reads only one of clippy.toml and .clippy.toml, so keeping both hides which applies. Keep one of them."
            }
            Self::ClippyConfigurationMissing { .. } => {
                "Tests may fail fast while production code may not, and clippy needs a configuration to tell them apart. Add a clippy.toml with the test allowances."
            }
            Self::ClippyLintNotDenied { .. } => {
                "Production code reports failures as typed errors instead of panicking. Deny the lint in the [lints.clippy] or [workspace.lints.clippy] table the package uses."
            }
            Self::ClippyTestAllowanceMissing { .. } => {
                "Tests may fail fast with expect, unwrap and panic. Set the allowance to true in the clippy configuration."
            }
            Self::LintAllowedInManifest { .. } => {
                "Allowing a lint is a suppression, and suppressions are a slippery slope. Remove the allow and fix the code the lint reports."
            }
            Self::PackageDependencyCycle { .. } => {
                "Dev-dependency cycles build a package twice and blur which package owns what. Break the cycle."
            }
            Self::PackageDependsOnItself { .. } => {
                "A package depending on itself is built twice and blurs what it owns. Remove the self-dependency."
            }
            Self::TestsOnlyPackageWithoutTestLibraries { .. } => {
                "A separate tests package is justified only by test-only libraries. Move the tests into the tests directory of the package they test."
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
