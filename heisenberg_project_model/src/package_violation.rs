use std::fmt;
use std::fmt::Display;
use std::fmt::Formatter;

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum PackageViolation {
    PackageDependencyCycle {
        manifest: String,
        packages: Vec<String>,
    },
    TestsOnlyPackageWithoutTestLibraries {
        destinations: Vec<String>,
        manifest: String,
        package: String,
    },
}

impl PackageViolation {
    #[must_use]
    pub const fn issue_type(&self) -> &'static str {
        match self {
            Self::PackageDependencyCycle { .. } => "PackageDependencyCycle",
            Self::TestsOnlyPackageWithoutTestLibraries { .. } => {
                "TestsOnlyPackageWithoutTestLibraries"
            }
        }
    }

    #[must_use]
    pub fn manifest(&self) -> &str {
        match self {
            Self::PackageDependencyCycle { manifest, .. }
            | Self::TestsOnlyPackageWithoutTestLibraries { manifest, .. } => manifest,
        }
    }
}

impl Display for PackageViolation {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::PackageDependencyCycle { packages, .. } => write!(
                formatter,
                "Packages {} depend on each other through dev-dependencies",
                packages.join(", ")
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
