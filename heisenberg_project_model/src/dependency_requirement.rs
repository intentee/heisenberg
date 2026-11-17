use cargo_metadata::semver::Comparator;
use cargo_metadata::semver::Op;
use cargo_metadata::semver::VersionReq;
use cargo_util_schemas::manifest::TomlDependency;
use cargo_util_schemas::manifest::TomlDetailedDependency;

use crate::unpinned_requirement::UnpinnedRequirement;

fn is_pinned(written: &str) -> bool {
    VersionReq::parse(written).is_ok_and(|VersionReq { comparators }| {
        matches!(
            comparators.as_slice(),
            [Comparator {
                op: Op::Caret,
                major,
                minor: Some(minor),
                patch,
                pre,
            }] if patch.is_some() == ((*major == 0 && *minor == 0) || !pre.is_empty())
        )
    })
}

fn unpinned_version(written: &str) -> Option<UnpinnedRequirement> {
    (!is_pinned(written)).then(|| UnpinnedRequirement::Version {
        requirement: written.to_owned(),
    })
}

pub struct DependencyRequirement<'requirement> {
    pub dependency: &'requirement TomlDependency,
}

impl DependencyRequirement<'_> {
    #[must_use]
    pub fn unpinned(&self) -> Option<UnpinnedRequirement> {
        match self.dependency {
            TomlDependency::Simple(version) => unpinned_version(version),
            TomlDependency::Detailed(TomlDetailedDependency {
                git: Some(_), rev, ..
            }) => rev
                .is_none()
                .then_some(UnpinnedRequirement::GitWithoutRevision),
            TomlDependency::Detailed(TomlDetailedDependency { version, .. }) => {
                version.as_deref().and_then(unpinned_version)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use cargo_util_schemas::manifest::TomlDependency;
    use toml::from_str;

    use super::DependencyRequirement;
    use crate::unpinned_requirement::UnpinnedRequirement;

    fn unpinned(written: &str) -> Option<UnpinnedRequirement> {
        let dependency = from_str::<TomlDependency>(written)
            .expect("the dependency of a test is a valid cargo dependency");

        DependencyRequirement {
            dependency: &dependency,
        }
        .unpinned()
    }

    fn version(requirement: &str) -> UnpinnedRequirement {
        UnpinnedRequirement::Version {
            requirement: requirement.to_owned(),
        }
    }

    #[test]
    fn major_and_minor_pin_a_released_version() {
        assert_eq!(unpinned("version = \"1.2\""), None);
    }

    #[test]
    fn patch_of_a_released_version_is_not_a_pin() {
        assert_eq!(unpinned("version = \"1.2.3\""), Some(version("1.2.3")));
    }

    #[test]
    fn major_alone_is_not_a_pin() {
        assert_eq!(unpinned("version = \"1\""), Some(version("1")));
    }

    #[test]
    fn patch_pins_a_zero_zero_version() {
        assert_eq!(unpinned("version = \"0.0.358\""), None);
    }

    #[test]
    fn zero_zero_without_a_patch_is_not_a_pin() {
        assert_eq!(unpinned("version = \"0.0\""), Some(version("0.0")));
    }

    #[test]
    fn patch_pins_a_pre_release() {
        assert_eq!(unpinned("version = \"1.2.0-rc.1\""), None);
    }

    #[test]
    fn exact_requirement_is_not_a_pin() {
        assert_eq!(unpinned("version = \"=1.2.3\""), Some(version("=1.2.3")));
    }

    #[test]
    fn range_is_not_a_pin() {
        assert_eq!(
            unpinned("version = \">=1.2, <2\""),
            Some(version(">=1.2, <2"))
        );
    }

    #[test]
    fn wildcard_is_not_a_pin() {
        assert_eq!(unpinned("version = \"*\""), Some(version("*")));
    }

    #[test]
    fn requirement_that_is_no_version_is_not_a_pin() {
        assert_eq!(unpinned("version = \"latest\""), Some(version("latest")));
    }

    #[test]
    fn git_dependency_with_a_rev_is_pinned() {
        assert_eq!(
            unpinned("git = \"https://example.invalid/crate\"\nrev = \"0123abc\""),
            None
        );
    }

    #[test]
    fn git_dependency_on_a_branch_is_not_pinned() {
        assert_eq!(
            unpinned("git = \"https://example.invalid/crate\"\nbranch = \"main\"")
                .map(|requirement| requirement.to_string()),
            Some("follows a git repository without naming a rev".to_owned())
        );
    }

    #[test]
    fn requirement_written_as_a_plain_string_is_checked() {
        assert_eq!(
            DependencyRequirement {
                dependency: &TomlDependency::Simple("1".to_owned()),
            }
            .unpinned(),
            Some(version("1"))
        );
    }

    #[test]
    fn path_dependency_without_a_version_is_exempt() {
        assert_eq!(unpinned("path = \"../crate\""), None);
    }
}
