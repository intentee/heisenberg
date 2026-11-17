use heisenberg_project_model::manifest_violation::ManifestViolation;

use crate::constants::RUST_VERSION_BELOW_SUPPORTED;
use crate::constants::UNPINNED_DEPENDENCY_REQUIREMENT;
use crate::issue_text::IssueText;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ManifestIssue {
    RustVersionBelowSupported,
    UnpinnedDependencyRequirement,
}

impl ManifestIssue {
    #[must_use]
    pub const fn text(self) -> IssueText {
        match self {
            Self::RustVersionBelowSupported => RUST_VERSION_BELOW_SUPPORTED,
            Self::UnpinnedDependencyRequirement => UNPINNED_DEPENDENCY_REQUIREMENT,
        }
    }

    #[must_use]
    pub const fn variants() -> [Self; 2] {
        [
            Self::RustVersionBelowSupported,
            Self::UnpinnedDependencyRequirement,
        ]
    }
}

impl From<&ManifestViolation> for ManifestIssue {
    fn from(violation: &ManifestViolation) -> Self {
        match violation {
            ManifestViolation::RustVersionBelowSupported { .. } => Self::RustVersionBelowSupported,
            ManifestViolation::UnpinnedDependency { .. } => Self::UnpinnedDependencyRequirement,
        }
    }
}
