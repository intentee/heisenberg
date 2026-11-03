use std::fmt;
use std::fmt::Display;
use std::fmt::Formatter;

use crate::constants::CONSTANTS_MODULE_NAME;

macro_rules! resolution_purpose {
    ($instruction:literal) => {
        concat!(
            "Every name must resolve for the analysis to be complete. ",
            $instruction
        )
    };
}

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

impl ViolationKind {
    #[must_use]
    pub const fn issue_type(&self) -> &'static str {
        match self {
            Self::AdditionalPublicItem => "AdditionalPublicItem",
            Self::ComputedEnvironmentVariableName => "ComputedEnvironmentVariableName",
            Self::ConfigurationSelectionMacro => "ConfigurationSelectionMacro",
            Self::FailedMacroExpansion => "FailedMacroExpansion",
            Self::FunctionUsedOnlyByTests => "FunctionUsedOnlyByTests",
            Self::GlobImport => "GlobImport",
            Self::InvalidMacroDefinition => "InvalidMacroDefinition",
            Self::ImplWithoutLocalType => "ImplWithoutLocalType",
            Self::LintSuppression => "LintSuppression",
            Self::ModuleFileInsideBlock => "ModuleFileInsideBlock",
            Self::ModuleNotNamedAfterItsItem { .. } => "ModuleNotNamedAfterItsItem",
            Self::MisplacedBlanketImpl { .. } => "MisplacedBlanketImpl",
            Self::MisplacedForeignTypeImpl { .. } => "MisplacedForeignTypeImpl",
            Self::MisplacedImpl { .. } => "MisplacedImpl",
            Self::MisplacedTraitObjectImpl { .. } => "MisplacedTraitObjectImpl",
            Self::NegatedConfiguration => "NegatedConfiguration",
            Self::NonConstantInConstantsModule => "NonConstantInConstantsModule",
            Self::NonDocComment => "NonDocComment",
            Self::NonTestItemInTestTarget => "NonTestItemInTestTarget",
            Self::ProceduralFunction { .. } => "ProceduralFunction",
            Self::PublicAssociatedConstant => "PublicAssociatedConstant",
            Self::PublicConstantOutsideConstantsModule => "PublicConstantOutsideConstantsModule",
            Self::Reexport => "Reexport",
            Self::TestOnlyCodeInProduction => "TestOnlyCodeInProduction",
            Self::UnimplementedBuiltinMacro => "UnimplementedBuiltinMacro",
            Self::UnmappedItem => "UnmappedItem",
            Self::UnparsableMacroExpansion => "UnparsableMacroExpansion",
            Self::UnresolvedExternCrate => "UnresolvedExternCrate",
            Self::UnresolvedMacroCall => "UnresolvedMacroCall",
            Self::UnresolvedMethodCall => "UnresolvedMethodCall",
            Self::UnresolvedModule => "UnresolvedModule",
            Self::UnresolvedPath => "UnresolvedPath",
            Self::UndeclaredEnvironmentVariable { .. } => "UndeclaredEnvironmentVariable",
        }
    }

    #[must_use]
    pub const fn purpose(&self) -> &'static str {
        match self {
            Self::AdditionalPublicItem => {
                "Each module exposes a single public item, so every file has one responsibility. Move each additional public item into its own module."
            }
            Self::ComputedEnvironmentVariableName => {
                "Heisenberg accounts for the environment variable a macro call reads only when a string literal names it. Name the variable with a string literal."
            }
            Self::ConfigurationSelectionMacro => {
                "cfg_select! discards the arms it does not select before they can be analysed. Write each arm as an item under its own #[cfg] attribute."
            }
            Self::FailedMacroExpansion => {
                "A macro call that fails to expand produces no code to analyse. Fix the call so it expands."
            }
            Self::FunctionUsedOnlyByTests => {
                "Production code holds only what production uses. Move a function that only tests use into a test crate: a leaf package with integration tests and at most a library for them."
            }
            Self::GlobImport => {
                "Glob imports hide where names come from. Import each name explicitly."
            }
            Self::InvalidMacroDefinition => {
                "An invalid macro definition produces no code to analyse. Fix the definition."
            }
            Self::ImplWithoutLocalType => {
                "An impl of another crate's trait belongs beside the one type of this crate it serves. Implement the trait for a struct, enum or union of this crate, or for a foreign type with exactly one of them among the trait's type arguments."
            }
            Self::LintSuppression => {
                "Suppressing lints is a slippery slope. Remove the attribute and fix the code the lint reports."
            }
            Self::ModuleFileInsideBlock => {
                "A module file declared inside a block is outside the analysed module tree. Declare the module at module level."
            }
            Self::ModuleNotNamedAfterItsItem { .. } => {
                "A module is named after the one item it exposes, so the file that defines an item is found from the item's name alone. Rename the module to the snake case of its public item."
            }
            Self::MisplacedBlanketImpl { .. } => {
                "A blanket impl reaches every type its bounds admit, so it belongs beside its trait. Move the impl into the file that defines the trait."
            }
            Self::MisplacedForeignTypeImpl { .. } => {
                "An impl of a trait of this crate for a type it does not define belongs beside the trait, so every impl has exactly one place. Move the impl into the file that defines the trait."
            }
            Self::MisplacedImpl { .. } => {
                "All behavior of a type is read in one place, the file that defines it. Move the impl block into that file."
            }
            Self::MisplacedTraitObjectImpl { .. } => {
                "A trait object's behavior belongs beside its trait. Move the impl into the file that defines the trait."
            }
            Self::NegatedConfiguration => {
                "Code that exists only without a feature, outside tests or outside debug builds escapes testing and makes features non-additive. Express the condition positively."
            }
            Self::NonConstantInConstantsModule => {
                "The constants module is the single home of public constants. Move every other public item out of it."
            }
            Self::NonDocComment => {
                "Names and structure document the code; only doc comments, which documentation tooling consumes, are allowed. Rename or restructure until the comment is unnecessary, or make it a doc comment if it documents an API."
            }
            Self::NonTestItemInTestTarget => {
                "A helper used beyond its own test file is a test library, and test libraries never live in a tests directory. Move it into a library crate; when that crate depends on the package under test, move the tests that use it there too, so package dependencies never form a cycle."
            }
            Self::ProceduralFunction { .. } => {
                "Behavior belongs with the data it works on. Make the function a method of the type it takes, a constructor of its own type, or a private helper used in a single file."
            }
            Self::PublicAssociatedConstant => {
                "Public constants have a single home, crate::constants. Move the constant there as a free constant."
            }
            Self::PublicConstantOutsideConstantsModule => {
                "Public constants have a single home, crate::constants. Move the constant there."
            }
            Self::Reexport => {
                "Every item is reachable through exactly one path. Import the item from where it is defined instead of re-exporting it."
            }
            Self::TestOnlyCodeInProduction => {
                "Production code holds only what production runs. Keep unit tests in the inline #[cfg(test)] mod tests of the file they test, and move every other piece of test-only code into a test crate."
            }
            Self::UnimplementedBuiltinMacro => {
                "rust-analyzer cannot expand this built-in macro, so its output cannot be analysed. Replace the call with code that can be analysed."
            }
            Self::UnmappedItem => {
                "An item that cannot be mapped to its definition leaves the analysis incomplete. Write it so it maps to a definition."
            }
            Self::UnparsableMacroExpansion => {
                "A macro expansion that does not parse cannot be analysed. Fix the macro so its output is valid Rust."
            }
            Self::UnresolvedExternCrate => {
                resolution_purpose!("Declare the crate as a dependency or remove the declaration.")
            }
            Self::UnresolvedMacroCall => resolution_purpose!("Define or import the macro."),
            Self::UnresolvedMethodCall => {
                resolution_purpose!("Fix the call so its method resolves.")
            }
            Self::UnresolvedModule => {
                resolution_purpose!("Create the module file or remove the declaration.")
            }
            Self::UnresolvedPath => resolution_purpose!("Fix the path so it resolves."),
            Self::UndeclaredEnvironmentVariable { .. } => {
                "A compile-time environment variable that only the shell sets is an input Heisenberg cannot see, so the analysis would use a wrong value. Read only variables that cargo or a build script sets, declare the variable in the [env] table of the cargo configuration, or read it at run time."
            }
        }
    }
}

impl Display for ViolationKind {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::AdditionalPublicItem => {
                formatter.write_str("Module already has a public item")
            }
            Self::ComputedEnvironmentVariableName => formatter.write_str(
                "Macro call does not name its environment variable with a readable string literal",
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
            Self::ModuleNotNamedAfterItsItem { expected, module } => write!(
                formatter,
                "Module is named {module}, but its public item names it {expected}"
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
                .write_str("Code exists only under cfg(test) outside the unit-test module"),
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
