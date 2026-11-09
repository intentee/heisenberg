use std::fmt;
use std::fmt::Display;
use std::fmt::Formatter;
use std::num::NonZeroU64;
use std::time::Duration;

use humantime::format_duration;

use crate::constants::CLIPPY_CONFIGURATION_FILE;
use crate::constants::CLIPPY_HIDDEN_CONFIGURATION_FILE;
use crate::constants::MAXIMUM_SLOW_TEST_PERIOD;
use crate::constants::MAXIMUM_TEST_DURATION;
use crate::constants::NEXTEST_CONFIGURATION_DIRECTORY;
use crate::constants::NEXTEST_CONFIGURATION_FILE;
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
    LintAllowedInManifest {
        lint: String,
        table: LintTable,
        tool: String,
    },
    NextestConfigurationMissing {
        manifest: String,
    },
    NextestOverride {
        file: String,
        profile: String,
    },
    NextestSlowPeriodTooLong {
        file: String,
        period: Duration,
        profile: String,
    },
    NextestSlowTimeoutUnset {
        file: String,
    },
    NextestTestNeverTerminated {
        file: String,
        profile: String,
    },
    NextestTestTerminatedTooLate {
        file: String,
        period: Duration,
        periods: NonZeroU64,
        profile: String,
    },
    NextestTimeoutPasses {
        file: String,
        profile: String,
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
    RequiredClippySettingMissing {
        file: String,
        setting: String,
        value: bool,
    },
    RequiredLintNotDenied {
        lint: String,
        table: LintTable,
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
    UnvettedClippySetting {
        file: String,
        setting: String,
    },
    UnvettedNextestSetting {
        file: String,
        setting: String,
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
            | Self::LintAllowedInManifest { .. }
            | Self::NextestConfigurationMissing { .. }
            | Self::NextestOverride { .. }
            | Self::NextestSlowPeriodTooLong { .. }
            | Self::NextestSlowTimeoutUnset { .. }
            | Self::NextestTestNeverTerminated { .. }
            | Self::NextestTestTerminatedTooLate { .. }
            | Self::NextestTimeoutPasses { .. }
            | Self::PackageDependencyCycle { .. }
            | Self::PackageDependsOnItself { .. }
            | Self::PublishableTestCrate { .. }
            | Self::RequiredClippySettingMissing { .. }
            | Self::RequiredLintNotDenied { .. }
            | Self::TestCrateDeclaredTwice { .. }
            | Self::TestCrateNotAMember { .. }
            | Self::TestCrateUsedByProduction { .. }
            | Self::UnvettedClippySetting { .. }
            | Self::UnvettedNextestSetting { .. } => Vec::new(),
        }
    }

    #[must_use]
    pub fn file(&self) -> &str {
        match self {
            Self::ClippyConfigurationAmbiguous { file }
            | Self::RequiredClippySettingMissing { file, .. }
            | Self::UnvettedClippySetting { file, .. }
            | Self::NextestOverride { file, .. }
            | Self::NextestSlowPeriodTooLong { file, .. }
            | Self::NextestSlowTimeoutUnset { file }
            | Self::NextestTestNeverTerminated { file, .. }
            | Self::NextestTestTerminatedTooLate { file, .. }
            | Self::NextestTimeoutPasses { file, .. }
            | Self::UnvettedNextestSetting { file, .. }
            | Self::ClippyConfigurationMissing { manifest: file, .. }
            | Self::NextestConfigurationMissing { manifest: file }
            | Self::PackageDependencyCycle { manifest: file, .. }
            | Self::PackageDependsOnItself { manifest: file, .. }
            | Self::PublishableTestCrate { manifest: file, .. }
            | Self::TestCrateDeclaredTwice { manifest: file, .. }
            | Self::TestCrateNotAMember { manifest: file, .. }
            | Self::TestCrateUsedByProduction { manifest: file, .. }
            | Self::TestsOnlyPackageWithoutTestLibraries { manifest: file, .. } => file,
            Self::LintAllowedInManifest { table, .. }
            | Self::RequiredLintNotDenied { table, .. } => table.manifest(),
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
            Self::LintAllowedInManifest { lint, table, tool } => {
                write!(formatter, "{table} allows {lint} in its {tool} lints")
            }
            Self::NextestConfigurationMissing { .. } => write!(
                formatter,
                "Workspace has no {NEXTEST_CONFIGURATION_DIRECTORY}/{NEXTEST_CONFIGURATION_FILE}"
            ),
            Self::NextestOverride { profile, .. } => write!(
                formatter,
                "Profile {profile} overrides the settings of some tests"
            ),
            Self::NextestSlowPeriodTooLong {
                period, profile, ..
            } => write!(
                formatter,
                "Profile {profile} flags a test as slow after {}, beyond {}",
                format_duration(*period),
                format_duration(MAXIMUM_SLOW_TEST_PERIOD)
            ),
            Self::NextestSlowTimeoutUnset { .. } => write!(
                formatter,
                "Profile default sets no slow-timeout, so nextest never terminates a test"
            ),
            Self::NextestTestNeverTerminated { profile, .. } => {
                write!(formatter, "Profile {profile} never terminates a slow test")
            }
            Self::NextestTestTerminatedTooLate {
                period,
                periods,
                profile,
                ..
            } => write!(
                formatter,
                "Profile {profile} terminates a test after {periods} periods of {}, beyond {}",
                format_duration(*period),
                format_duration(MAXIMUM_TEST_DURATION)
            ),
            Self::NextestTimeoutPasses { profile, .. } => write!(
                formatter,
                "Profile {profile} counts a timed out test as passing"
            ),
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
            Self::RequiredClippySettingMissing { setting, value, .. } => {
                write!(
                    formatter,
                    "Clippy configuration does not set {setting} = {value}"
                )
            }
            Self::RequiredLintNotDenied { lint, table } => {
                write!(formatter, "{table} does not deny {lint}")
            }
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
            Self::UnvettedClippySetting { setting, .. } => write!(
                formatter,
                "Clippy configuration sets {setting}, which Heisenberg has not vetted"
            ),
            Self::UnvettedNextestSetting { setting, .. } => write!(
                formatter,
                "Nextest configuration sets {setting}, which Heisenberg has not vetted"
            ),
        }
    }
}
