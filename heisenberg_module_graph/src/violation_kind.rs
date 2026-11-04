use std::fmt;
use std::fmt::Display;
use std::fmt::Formatter;

use crate::constants::CONSTANTS_MODULE_NAME;

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum ViolationKind {
    AdditionalPublicItem,
    ComputedEnvironmentVariableName,
    ConfigurationSelectionMacro,
    FailedMacroExpansion,
    FunctionUsedOnlyByTests,
    GlobImport,
    InvalidMacroDefinition,
    ImplWithoutLocalType,
    LintSuppression,
    ModuleFileInsideBlock,
    ModuleNotNamedAfterItsItem {
        expected: String,
        item: String,
        module: String,
    },
    MisplacedBlanketImpl {
        trait_file: String,
    },
    MisplacedForeignTypeImpl {
        trait_file: String,
        trait_name: String,
    },
    MisplacedImpl {
        type_file: String,
    },
    MisplacedTraitObjectImpl {
        trait_file: String,
    },
    NegatedConfiguration,
    NonConstantInConstantsModule,
    NonDocComment,
    NonTestItemInTestTarget,
    ProceduralFunction {
        type_name: String,
    },
    PublicAssociatedConstant,
    PublicConstantOutsideConstantsModule,
    Reexport,
    TestOnlyCodeInProduction,
    UnimplementedBuiltinMacro,
    UnmappedItem,
    UnparsableMacroExpansion,
    UnresolvedExternCrate,
    UnresolvedMacroCall,
    UnresolvedMethodCall,
    UnresolvedModule,
    UnresolvedPath,
    UndeclaredEnvironmentVariable {
        variable: String,
    },
}

impl Display for ViolationKind {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::AdditionalPublicItem => {
                formatter.write_str("Module already has a public item")
            }
            Self::ComputedEnvironmentVariableName => formatter.write_str(
                "Macro call argument does not expand to a string literal",
            ),
            Self::ConfigurationSelectionMacro => formatter.write_str(
                "Configuration selection macro keeps the arms it does not select out of the analysis",
            ),
            Self::FailedMacroExpansion => formatter.write_str("Macro expansion failed"),
            Self::FunctionUsedOnlyByTests => formatter.write_str("Function is used only by tests"),
            Self::GlobImport => formatter.write_str("Glob import"),
            Self::InvalidMacroDefinition => formatter.write_str("Macro definition is invalid"),
            Self::ImplWithoutLocalType => formatter
                .write_str("Impl block belongs to no single type or trait of this crate"),
            Self::LintSuppression => formatter.write_str("Attribute suppresses lints"),
            Self::ModuleFileInsideBlock => {
                formatter.write_str("Module file declared inside a block is not analysed")
            }
            Self::ModuleNotNamedAfterItsItem {
                expected,
                item,
                module,
            } => write!(
                formatter,
                "Module is named {module}, but its public item {item} names it {expected}"
            ),
            Self::MisplacedBlanketImpl { trait_file } => {
                write!(formatter, "Blanket impl belongs in {trait_file}")
            }
            Self::MisplacedForeignTypeImpl {
                trait_file,
                trait_name,
            } => write!(
                formatter,
                "Impl block of {trait_name} for a foreign type belongs in {trait_file}"
            ),
            Self::MisplacedImpl { type_file } => {
                write!(formatter, "Impl block belongs in {type_file}")
            }
            Self::MisplacedTraitObjectImpl { trait_file } => {
                write!(formatter, "Impl block on a trait object belongs in {trait_file}")
            }
            Self::NegatedConfiguration => formatter.write_str(
                "Configuration predicate negates a feature, test or debug_assertions",
            ),
            Self::NonConstantInConstantsModule => {
                formatter.write_str("Constants module holds a public item that is not a constant")
            }
            Self::NonDocComment => formatter.write_str("Comment is not a doc comment"),
            Self::NonTestItemInTestTarget => formatter.write_str(
                "Integration test target holds an item that is neither a test, a constant, an import nor a private helper used only in its own file",
            ),
            Self::ProceduralFunction { type_name } => {
                write!(formatter, "Function takes {type_name} without belonging to it")
            }
            Self::PublicAssociatedConstant => write!(
                formatter,
                "Public associated constant belongs in crate::{CONSTANTS_MODULE_NAME} as a free constant"
            ),
            Self::PublicConstantOutsideConstantsModule => write!(
                formatter,
                "Public constant belongs in crate::{CONSTANTS_MODULE_NAME}"
            ),
            Self::Reexport => formatter.write_str("Re-export"),
            Self::TestOnlyCodeInProduction => formatter
                .write_str("Code exists only under cfg(test) outside a test module"),
            Self::UnimplementedBuiltinMacro => {
                formatter.write_str("Built-in macro is not implemented")
            }
            Self::UnmappedItem => formatter.write_str("Item cannot be mapped to its definition"),
            Self::UnparsableMacroExpansion => {
                formatter.write_str("Macro expansion cannot be parsed")
            }
            Self::UnresolvedExternCrate => formatter.write_str("Extern crate is unresolved"),
            Self::UnresolvedMacroCall => formatter.write_str("Macro call is unresolved"),
            Self::UnresolvedMethodCall => formatter.write_str("Method call is unresolved"),
            Self::UnresolvedModule => formatter.write_str("Module file is unresolved"),
            Self::UnresolvedPath => formatter.write_str("Path is unresolved"),
            Self::UndeclaredEnvironmentVariable { variable } => write!(
                formatter,
                "Macro call reads {variable}, which neither cargo, a build script nor the [env] configuration sets"
            ),
        }
    }
}
