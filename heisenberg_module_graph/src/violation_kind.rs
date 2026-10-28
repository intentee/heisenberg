use std::fmt;
use std::fmt::Display;
use std::fmt::Formatter;

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum ViolationKind {
    FailedMacroExpansion,
    GlobImport,
    InvalidMacroDefinition,
    NegatedConfiguration,
    SharedModuleFile,
    UnimplementedBuiltinMacro,
    UnparsableMacroExpansion,
    UnresolvedExternCrate,
    UnresolvedMacroCall,
    UnresolvedModule,
    UnresolvedPath,
}

impl Display for ViolationKind {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
        formatter.write_str(match self {
            Self::FailedMacroExpansion => "macro expansion failed",
            Self::GlobImport => "glob import",
            Self::InvalidMacroDefinition => "macro definition is invalid",
            Self::NegatedConfiguration => {
                "configuration predicate negates a feature, test or debug_assertions"
            }
            Self::SharedModuleFile => "module file belongs to more than one module",
            Self::UnimplementedBuiltinMacro => "built-in macro is not implemented",
            Self::UnparsableMacroExpansion => "macro expansion cannot be parsed",
            Self::UnresolvedExternCrate => "extern crate is unresolved",
            Self::UnresolvedMacroCall => "macro call is unresolved",
            Self::UnresolvedModule => "module file is unresolved",
            Self::UnresolvedPath => "path is unresolved",
        })
    }
}
