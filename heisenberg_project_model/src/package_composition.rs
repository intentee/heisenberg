use crate::libraryless_contents::LibrarylessContents;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PackageComposition {
    HoldsProductionCode,
    WithoutLibrary(LibrarylessContents),
}
