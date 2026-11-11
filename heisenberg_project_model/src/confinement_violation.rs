use std::fmt;
use std::fmt::Display;
use std::fmt::Formatter;

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum ConfinementViolation {
    UnknownPackage {
        manifest: String,
        package: String,
    },
    UserListedTwice {
        manifest: String,
        package: String,
        user: String,
    },
    UserNotAMember {
        manifest: String,
        package: String,
        user: String,
    },
    UserNotReachingIt {
        manifest: String,
        package: String,
        user: String,
    },
}

impl ConfinementViolation {
    #[must_use]
    pub fn manifest(&self) -> &str {
        match self {
            Self::UnknownPackage { manifest, .. }
            | Self::UserListedTwice { manifest, .. }
            | Self::UserNotAMember { manifest, .. }
            | Self::UserNotReachingIt { manifest, .. } => manifest,
        }
    }
}

impl Display for ConfinementViolation {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnknownPackage { package, .. } => write!(
                formatter,
                "Confined package {package} names no package that the workspace resolves"
            ),
            Self::UserListedTwice { package, user, .. } => write!(
                formatter,
                "Package {user} is listed more than once as a user of confined package {package}"
            ),
            Self::UserNotAMember { package, user, .. } => write!(
                formatter,
                "Package {user} is listed as a user of confined package {package} but is not a workspace member"
            ),
            Self::UserNotReachingIt { package, user, .. } => write!(
                formatter,
                "Package {user} is listed as a user of confined package {package} but depends on it in no declared configuration"
            ),
        }
    }
}
