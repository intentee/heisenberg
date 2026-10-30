use std::fmt;
use std::fmt::Display;
use std::fmt::Formatter;

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum ViolationKind {
    AdditionalPublicItem,
    FailedMacroExpansion,
    GlobImport,
    InvalidMacroDefinition,
    ImplWithoutLocalType,
    MisplacedImpl { type_file: String },
    ModuleFileRepeatedInCrate,
    NegatedConfiguration,
    NonTestItemInTestTarget,
    Reexport,
    UnimplementedBuiltinMacro,
    UnparsableMacroExpansion,
    UnresolvedExternCrate,
    UnresolvedMacroCall,
    UnresolvedModule,
    UnresolvedPath,
}

impl Display for ViolationKind {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::AdditionalPublicItem => {
                formatter.write_str("module already has a public item")
            }
            Self::FailedMacroExpansion => formatter.write_str("macro expansion failed"),
            Self::GlobImport => formatter.write_str("glob import"),
            Self::InvalidMacroDefinition => formatter.write_str("macro definition is invalid"),
            Self::ImplWithoutLocalType => formatter
                .write_str("impl block does not implement a struct, enum or union of this crate"),
            Self::MisplacedImpl { type_file } => {
                write!(formatter, "impl block belongs in {type_file}")
            }
            Self::ModuleFileRepeatedInCrate => formatter.write_str(
                "module file is another module of the same crate, so it cannot be analysed as this module",
            ),
            Self::NegatedConfiguration => formatter.write_str(
                "configuration predicate negates a feature, test or debug_assertions",
            ),
            Self::NonTestItemInTestTarget => formatter
                .write_str("integration test target holds an item that is neither a test nor an import"),
            Self::Reexport => formatter.write_str("re-export"),
            Self::UnimplementedBuiltinMacro => {
                formatter.write_str("built-in macro is not implemented")
            }
            Self::UnparsableMacroExpansion => {
                formatter.write_str("macro expansion cannot be parsed")
            }
            Self::UnresolvedExternCrate => formatter.write_str("extern crate is unresolved"),
            Self::UnresolvedMacroCall => formatter.write_str("macro call is unresolved"),
            Self::UnresolvedModule => formatter.write_str("module file is unresolved"),
            Self::UnresolvedPath => formatter.write_str("path is unresolved"),
        }
    }
}
