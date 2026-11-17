use std::fmt;
use std::fmt::Display;
use std::fmt::Formatter;
use std::num::NonZeroU64;
use std::time::Duration;

use humantime::format_duration;

use crate::benchmark_target_violation::BenchmarkTargetViolation;
use crate::clippy_violation::ClippyViolation;
use crate::confinement_violation::ConfinementViolation;
use crate::constants::MAXIMUM_SLOW_TEST_PERIOD;
use crate::constants::MAXIMUM_TEST_DURATION;
use crate::integration_test_violation::IntegrationTestViolation;
use crate::libraryless_package::LibrarylessPackage;
use crate::lint_table::LintTable;
use crate::manifest_violation::ManifestViolation;
use crate::nextest_presence_violation::NextestPresenceViolation;
use crate::potential_owner::PotentialOwner;
use crate::rustfmt_violation::RustfmtViolation;

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum PackageViolation {
    BenchmarkTarget(BenchmarkTargetViolation),
    Clippy(ClippyViolation),
    Confinement(ConfinementViolation),
    IntegrationTests(IntegrationTestViolation),
    LintAllowedInManifest {
        lint: String,
        table: LintTable,
        tool: String,
    },
    Manifest(ManifestViolation),
    NextestOverride {
        file: String,
        profile: String,
    },
    NextestPresence(NextestPresenceViolation),
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
    Rustfmt(RustfmtViolation),
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
    UnreachableSourceFile {
        file: String,
    },
    UnvettedNextestSetting {
        file: String,
        setting: String,
    },
    WithoutLibrary(LibrarylessPackage),
}

impl PackageViolation {
    #[must_use]
    pub fn details(&self) -> Vec<String> {
        match self {
            Self::WithoutLibrary(LibrarylessPackage {
                potential_owners, ..
            }) => potential_owners
                .iter()
                .map(PotentialOwner::to_string)
                .collect(),
            Self::IntegrationTests(IntegrationTestViolation::Split { targets, .. }) => {
                targets.clone()
            }
            Self::BenchmarkTarget(_)
            | Self::Clippy(_)
            | Self::Confinement(_)
            | Self::IntegrationTests(IntegrationTestViolation::Discovered { .. })
            | Self::LintAllowedInManifest { .. }
            | Self::Manifest(_)
            | Self::Rustfmt(_)
            | Self::NextestPresence(_)
            | Self::NextestOverride { .. }
            | Self::NextestSlowPeriodTooLong { .. }
            | Self::NextestSlowTimeoutUnset { .. }
            | Self::NextestTestNeverTerminated { .. }
            | Self::NextestTestTerminatedTooLate { .. }
            | Self::NextestTimeoutPasses { .. }
            | Self::PackageDependencyCycle { .. }
            | Self::PackageDependsOnItself { .. }
            | Self::PublishableTestCrate { .. }
            | Self::RequiredLintNotDenied { .. }
            | Self::TestCrateDeclaredTwice { .. }
            | Self::TestCrateNotAMember { .. }
            | Self::TestCrateUsedByProduction { .. }
            | Self::UnreachableSourceFile { .. }
            | Self::UnvettedNextestSetting { .. } => Vec::new(),
        }
    }

    #[must_use]
    pub fn file(&self) -> &str {
        match self {
            Self::NextestOverride { file, .. }
            | Self::NextestSlowPeriodTooLong { file, .. }
            | Self::NextestSlowTimeoutUnset { file }
            | Self::NextestTestNeverTerminated { file, .. }
            | Self::NextestTestTerminatedTooLate { file, .. }
            | Self::NextestTimeoutPasses { file, .. }
            | Self::UnvettedNextestSetting { file, .. }
            | Self::UnreachableSourceFile { file }
            | Self::PackageDependencyCycle { manifest: file, .. }
            | Self::PackageDependsOnItself { manifest: file, .. }
            | Self::PublishableTestCrate { manifest: file, .. }
            | Self::TestCrateDeclaredTwice { manifest: file, .. }
            | Self::TestCrateNotAMember { manifest: file, .. }
            | Self::TestCrateUsedByProduction { manifest: file, .. } => file,
            Self::BenchmarkTarget(violation) => violation.manifest(),
            Self::Clippy(violation) => violation.file(),
            Self::Confinement(violation) => violation.manifest(),
            Self::IntegrationTests(violation) => violation.manifest(),
            Self::Manifest(violation) => violation.manifest(),
            Self::Rustfmt(violation) => violation.file(),
            Self::NextestPresence(violation) => violation.file(),
            Self::WithoutLibrary(LibrarylessPackage { manifest, .. }) => manifest,
            Self::LintAllowedInManifest { table, .. }
            | Self::RequiredLintNotDenied { table, .. } => table.manifest(),
        }
    }
}

impl Display for PackageViolation {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::BenchmarkTarget(violation) => violation.fmt(formatter),
            Self::Clippy(violation) => violation.fmt(formatter),
            Self::IntegrationTests(violation) => violation.fmt(formatter),
            Self::Manifest(violation) => violation.fmt(formatter),
            Self::Rustfmt(violation) => violation.fmt(formatter),
            Self::Confinement(violation) => violation.fmt(formatter),
            Self::LintAllowedInManifest { lint, table, tool } => {
                write!(formatter, "{table} allows {lint} in its {tool} lints")
            }
            Self::NextestPresence(violation) => violation.fmt(formatter),
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
            Self::UnreachableSourceFile { .. } => {
                formatter.write_str("Source file is a module file of no crate target")
            }
            Self::UnvettedNextestSetting { setting, .. } => write!(
                formatter,
                "Nextest configuration sets {setting}, which Heisenberg has not vetted"
            ),
            Self::WithoutLibrary(package) => package.fmt(formatter),
        }
    }
}
