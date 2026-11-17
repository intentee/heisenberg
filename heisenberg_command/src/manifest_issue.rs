use heisenberg_project_model::manifest_violation::ManifestViolation;

use crate::constants::DOC_TESTS_ENABLED;
use crate::constants::RUST_VERSION_BELOW_SUPPORTED;
use crate::constants::TEST_DEPENDENCY_IN_PRODUCTION;
use crate::constants::TEST_LIBRARY_IN_PRODUCTION_PACKAGE;
use crate::constants::UNPINNED_DEPENDENCY_REQUIREMENT;
use crate::constants::UNUSED_DEPENDENCY;
use crate::issue_text::IssueText;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ManifestIssue {
    DocTestsEnabled,
    RustVersionBelowSupported,
    TestDependencyInProduction,
    TestLibraryInProductionPackage,
    UnpinnedDependencyRequirement,
    UnusedDependency,
}

impl ManifestIssue {
    #[must_use]
    pub const fn text(self) -> IssueText {
        match self {
            Self::DocTestsEnabled => DOC_TESTS_ENABLED,
            Self::RustVersionBelowSupported => RUST_VERSION_BELOW_SUPPORTED,
            Self::TestDependencyInProduction => TEST_DEPENDENCY_IN_PRODUCTION,
            Self::TestLibraryInProductionPackage => TEST_LIBRARY_IN_PRODUCTION_PACKAGE,
            Self::UnpinnedDependencyRequirement => UNPINNED_DEPENDENCY_REQUIREMENT,
            Self::UnusedDependency => UNUSED_DEPENDENCY,
        }
    }

    #[must_use]
    pub const fn variants() -> [Self; 6] {
        [
            Self::DocTestsEnabled,
            Self::RustVersionBelowSupported,
            Self::TestDependencyInProduction,
            Self::TestLibraryInProductionPackage,
            Self::UnpinnedDependencyRequirement,
            Self::UnusedDependency,
        ]
    }
}

impl From<&ManifestViolation> for ManifestIssue {
    fn from(violation: &ManifestViolation) -> Self {
        match violation {
            ManifestViolation::DocTestsEnabled { .. } => Self::DocTestsEnabled,
            ManifestViolation::RustVersionBelowSupported { .. } => Self::RustVersionBelowSupported,
            ManifestViolation::DependencyUnused { .. } => Self::UnusedDependency,
            ManifestViolation::TestDependencyInProduction { .. } => {
                Self::TestDependencyInProduction
            }
            ManifestViolation::TestLibraryInProductionPackage { .. } => {
                Self::TestLibraryInProductionPackage
            }
            ManifestViolation::UnpinnedDependency { .. } => Self::UnpinnedDependencyRequirement,
        }
    }
}
