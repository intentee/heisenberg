use std::fmt;
use std::fmt::Display;
use std::fmt::Formatter;

use crate::clippy_lint_path::clippy_lint_path;
use crate::constants::CLIPPY_CONFIGURATION_FILE;
use crate::constants::CLIPPY_HIDDEN_CONFIGURATION_FILE;
use crate::constants::TESTS_PACKAGE_SUFFIX;
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
        manifest: String,
        package: String,
        potential_owners: Vec<String>,
    },
    TestsPackageWithoutTestsSuffix {
        manifest: String,
        package: String,
    },
}

impl PackageViolation {
    #[must_use]
    pub fn details(&self) -> Vec<String> {
        match self {
            Self::TestsOnlyPackageWithoutTestLibraries {
                potential_owners, ..
            } => {
                if potential_owners.is_empty() {
                    vec![
                        "no package it uses can take its tests without a dependency cycle"
                            .to_owned(),
                    ]
                } else {
                    potential_owners
                        .iter()
                        .map(|owner| format!("could move without a dependency cycle to {owner}"))
                        .collect()
                }
            }
            Self::ClippyConfigurationAmbiguous { .. }
            | Self::ClippyConfigurationMissing { .. }
            | Self::ClippyLintNotDenied { .. }
            | Self::ClippyTestAllowanceMissing { .. }
            | Self::LintAllowedInManifest { .. }
            | Self::PackageDependencyCycle { .. }
            | Self::PackageDependsOnItself { .. }
            | Self::TestsPackageWithoutTestsSuffix { .. } => Vec::new(),
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
            | Self::TestsOnlyPackageWithoutTestLibraries { manifest: file, .. }
            | Self::TestsPackageWithoutTestsSuffix { manifest: file, .. } => file,
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
            Self::TestsPackageWithoutTestsSuffix { .. } => "TestsPackageWithoutTestsSuffix",
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
                "Production code reports failures as typed errors instead of panicking. Deny the lint in the [lints.clippy] or [workspace.lints.clippy] table the package uses; only a package named <something>_tests that no workspace member depends on may leave it undenied, in its own lints table."
            }
            Self::ClippyTestAllowanceMissing { .. } => {
                "Tests may fail fast with expect, unwrap and panic. Set the allowance to true in the clippy configuration."
            }
            Self::LintAllowedInManifest { .. } => {
                "Allowing a lint is a suppression, and suppressions are a slippery slope. Remove the allow; if the lint then reports code, fix that code."
            }
            Self::PackageDependencyCycle { .. } => {
                "Package dependencies never form a cycle, dev-dependencies included. When a test library depends on the package whose tests use it, move those tests and the helpers that need the package into a separate tests package that holds them in its library."
            }
            Self::PackageDependsOnItself { .. } => {
                "A package depending on itself is built twice and blurs what it owns. Remove the self-dependency."
            }
            Self::TestsOnlyPackageWithoutTestLibraries { .. } => {
                "Tests belong to the package whose behaviour they verify, in its tests directory. A separate tests package is justified only by test libraries of its own: shared helpers, which never live in a tests directory, and helpers that need the package under test, which that package cannot dev-depend on without a dependency cycle. Heisenberg cannot tell from the code which package the tests verify, so the packages listed could take the tests without a cycle, but the right owner is the package they verify. If a test library they use depends on that package, the test library is entangled with the code it helps to test: move the helpers that need the package into this package's library, or make the test library independent of it."
            }
            Self::TestsPackageWithoutTestsSuffix { .. } => {
                "A package that holds tests says so in its name, so people and tools tell test crates from production crates. Rename the package to end in _tests."
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
            Self::TestsOnlyPackageWithoutTestLibraries { package, .. } => write!(
                formatter,
                "Package {package} holds only tests and no test library of its own, so the package its tests verify must own them"
            ),
            Self::TestsPackageWithoutTestsSuffix { package, .. } => write!(
                formatter,
                "Package {package} holds only tests but its name does not end in {TESTS_PACKAGE_SUFFIX}"
            ),
        }
    }
}
