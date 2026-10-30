use std::fmt;
use std::fmt::Display;
use std::fmt::Formatter;

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum PackageViolation {
    DependencyCycle {
        packages: Vec<String>,
    },
    TestsWithoutTestLibraries {
        destinations: Vec<String>,
        package: String,
    },
}

impl Display for PackageViolation {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::DependencyCycle { packages } => write!(
                formatter,
                "packages {} depend on each other through dev-dependencies",
                packages.join(", ")
            ),
            Self::TestsWithoutTestLibraries {
                destinations,
                package,
            } => write!(
                formatter,
                "package {package} holds only tests that need no test library; move them into the tests directory of {}",
                destinations.join(", ")
            ),
        }
    }
}
