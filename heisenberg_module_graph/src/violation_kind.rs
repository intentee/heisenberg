use std::fmt;
use std::fmt::Display;
use std::fmt::Formatter;

use crate::constants::CONSTANTS_MODULE_NAME;

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum ViolationKind {
    AdditionalPublicItem,
    FailedMacroExpansion,
    GlobImport,
    InvalidMacroDefinition,
    ImplWithoutLocalType,
    MisplacedImpl { type_file: String },
    NegatedConfiguration,
    NonConstantInConstantsModule,
    NonTestItemInTestTarget,
    PublicConstantOutsideConstantsModule,
    Reexport,
    UnimplementedBuiltinMacro,
    UnparsableMacroExpansion,
    UnresolvedExternCrate,
    UnresolvedMacroCall,
    UnresolvedModule,
    UnresolvedPath,
}

impl ViolationKind {
    #[must_use]
    pub const fn issue_type(&self) -> &'static str {
        match self {
            Self::AdditionalPublicItem => "AdditionalPublicItem",
            Self::FailedMacroExpansion => "FailedMacroExpansion",
            Self::GlobImport => "GlobImport",
            Self::InvalidMacroDefinition => "InvalidMacroDefinition",
            Self::ImplWithoutLocalType => "ImplWithoutLocalType",
            Self::MisplacedImpl { .. } => "MisplacedImpl",
            Self::NegatedConfiguration => "NegatedConfiguration",
            Self::NonConstantInConstantsModule => "NonConstantInConstantsModule",
            Self::NonTestItemInTestTarget => "NonTestItemInTestTarget",
            Self::PublicConstantOutsideConstantsModule => "PublicConstantOutsideConstantsModule",
            Self::Reexport => "Reexport",
            Self::UnimplementedBuiltinMacro => "UnimplementedBuiltinMacro",
            Self::UnparsableMacroExpansion => "UnparsableMacroExpansion",
            Self::UnresolvedExternCrate => "UnresolvedExternCrate",
            Self::UnresolvedMacroCall => "UnresolvedMacroCall",
            Self::UnresolvedModule => "UnresolvedModule",
            Self::UnresolvedPath => "UnresolvedPath",
        }
    }
}

impl Display for ViolationKind {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::AdditionalPublicItem => {
                formatter.write_str("Module already has a public item")
            }
            Self::FailedMacroExpansion => formatter.write_str("Macro expansion failed"),
            Self::GlobImport => formatter.write_str("Glob import"),
            Self::InvalidMacroDefinition => formatter.write_str("Macro definition is invalid"),
            Self::ImplWithoutLocalType => formatter
                .write_str("Impl block does not implement a struct, enum or union of this crate"),
            Self::MisplacedImpl { type_file } => {
                write!(formatter, "Impl block belongs in {type_file}")
            }
            Self::NegatedConfiguration => formatter.write_str(
                "Configuration predicate negates a feature, test or debug_assertions",
            ),
            Self::NonConstantInConstantsModule => {
                formatter.write_str("Constants module holds a public item that is not a constant")
            }
            Self::NonTestItemInTestTarget => formatter.write_str(
                "Integration test target holds an item that is neither a test, a constant nor an import",
            ),
            Self::PublicConstantOutsideConstantsModule => write!(
                formatter,
                "Public constant belongs in crate::{CONSTANTS_MODULE_NAME}"
            ),
            Self::Reexport => formatter.write_str("Re-export"),
            Self::UnimplementedBuiltinMacro => {
                formatter.write_str("Built-in macro is not implemented")
            }
            Self::UnparsableMacroExpansion => {
                formatter.write_str("Macro expansion cannot be parsed")
            }
            Self::UnresolvedExternCrate => formatter.write_str("Extern crate is unresolved"),
            Self::UnresolvedMacroCall => formatter.write_str("Macro call is unresolved"),
            Self::UnresolvedModule => formatter.write_str("Module file is unresolved"),
            Self::UnresolvedPath => formatter.write_str("Path is unresolved"),
        }
    }
}
