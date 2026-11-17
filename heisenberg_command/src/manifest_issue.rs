use heisenberg_project_model::manifest_violation::ManifestViolation;

use crate::constants::DOC_TESTS_ENABLED;
use crate::constants::RUST_VERSION_BELOW_SUPPORTED;
use crate::constants::UNPINNED_DEPENDENCY_REQUIREMENT;
use crate::issue_text::IssueText;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ManifestIssue {
    DocTestsEnabled,
    RustVersionBelowSupported,
    UnpinnedDependencyRequirement,
}

impl ManifestIssue {
    #[must_use]
    pub const fn text(self) -> IssueText {
        match self {
            Self::DocTestsEnabled => DOC_TESTS_ENABLED,
            Self::RustVersionBelowSupported => RUST_VERSION_BELOW_SUPPORTED,
            Self::UnpinnedDependencyRequirement => UNPINNED_DEPENDENCY_REQUIREMENT,
        }
    }

    #[must_use]
    pub const fn variants() -> [Self; 3] {
        [
            Self::DocTestsEnabled,
            Self::RustVersionBelowSupported,
            Self::UnpinnedDependencyRequirement,
        ]
    }
}

impl From<&ManifestViolation> for ManifestIssue {
    fn from(violation: &ManifestViolation) -> Self {
        match violation {
            ManifestViolation::DocTestsEnabled { .. } => Self::DocTestsEnabled,
            ManifestViolation::RustVersionBelowSupported { .. } => Self::RustVersionBelowSupported,
            ManifestViolation::UnpinnedDependency { .. } => Self::UnpinnedDependencyRequirement,
        }
    }
}
