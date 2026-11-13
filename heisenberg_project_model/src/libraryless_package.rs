use std::fmt;
use std::fmt::Display;
use std::fmt::Formatter;

use crate::libraryless_contents::LibrarylessContents;
use crate::potential_owner::PotentialOwner;

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct LibrarylessPackage {
    pub contents: LibrarylessContents,
    pub manifest: String,
    pub package: String,
    pub potential_owners: Vec<PotentialOwner>,
}

impl Display for LibrarylessPackage {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
        let Self {
            contents, package, ..
        } = self;

        match contents {
            LibrarylessContents::Benchmarks => write!(
                formatter,
                "Package {package} holds only benchmarks and no library of its own, so the package its benchmarks measure must own them"
            ),
            LibrarylessContents::Tests => write!(
                formatter,
                "Package {package} holds only tests and no test library of its own, so the package its tests verify must own them"
            ),
        }
    }
}
