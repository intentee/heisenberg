use std::fmt;
use std::fmt::Display;
use std::fmt::Formatter;

use crate::constants::CONSTANTS_MODULE_NAME;

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum ViolationKind {
    AdditionalPublicItem,
    ConfigurationSelectionMacro,
    FailedMacroExpansion,
    GlobImport,
    InvalidMacroDefinition,
    ImplWithoutLocalType,
    LintSuppression,
    ModuleFileInsideBlock,
    MisplacedBlanketImpl { trait_file: String },
    MisplacedImpl { type_file: String },
    NegatedConfiguration,
    NonConstantInConstantsModule,
    NonDocComment,
    NonTestItemInTestTarget,
    PublicAssociatedConstant,
    PublicConstantOutsideConstantsModule,
    Reexport,
    UnimplementedBuiltinMacro,
    UnmappedItem,
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
            Self::ConfigurationSelectionMacro => "ConfigurationSelectionMacro",
            Self::FailedMacroExpansion => "FailedMacroExpansion",
            Self::GlobImport => "GlobImport",
            Self::InvalidMacroDefinition => "InvalidMacroDefinition",
            Self::ImplWithoutLocalType => "ImplWithoutLocalType",
            Self::LintSuppression => "LintSuppression",
            Self::ModuleFileInsideBlock => "ModuleFileInsideBlock",
            Self::MisplacedBlanketImpl { .. } => "MisplacedBlanketImpl",
            Self::MisplacedImpl { .. } => "MisplacedImpl",
            Self::NegatedConfiguration => "NegatedConfiguration",
            Self::NonConstantInConstantsModule => "NonConstantInConstantsModule",
            Self::NonDocComment => "NonDocComment",
            Self::NonTestItemInTestTarget => "NonTestItemInTestTarget",
            Self::PublicAssociatedConstant => "PublicAssociatedConstant",
            Self::PublicConstantOutsideConstantsModule => "PublicConstantOutsideConstantsModule",
            Self::Reexport => "Reexport",
            Self::UnimplementedBuiltinMacro => "UnimplementedBuiltinMacro",
            Self::UnmappedItem => "UnmappedItem",
            Self::UnparsableMacroExpansion => "UnparsableMacroExpansion",
            Self::UnresolvedExternCrate => "UnresolvedExternCrate",
            Self::UnresolvedMacroCall => "UnresolvedMacroCall",
            Self::UnresolvedModule => "UnresolvedModule",
            Self::UnresolvedPath => "UnresolvedPath",
        }
    }

    #[must_use]
    pub const fn purpose(&self) -> &'static str {
        match self {
            Self::AdditionalPublicItem => {
                "Each module exposes a single public item, so every file has one responsibility. Move each additional public item into its own module."
            }
            Self::ConfigurationSelectionMacro => {
                "cfg_select! discards the arms it does not select before they can be analysed. Write each arm as an item under its own #[cfg] attribute."
            }
            Self::FailedMacroExpansion => {
                "A macro call that fails to expand produces no code to analyse. Fix the call so it expands."
            }
            Self::GlobImport => {
                "Glob imports hide where names come from. Import each name explicitly."
            }
            Self::InvalidMacroDefinition => {
                "An invalid macro definition produces no code to analyse. Fix the definition."
            }
            Self::ImplWithoutLocalType => {
                "Behavior belongs with the type that owns it, so impls do not extend foreign types. Implement for a type of this crate, or write a blanket impl beside its trait."
            }
            Self::LintSuppression => {
                "Suppressing lints is a slippery slope. Remove the attribute and fix the code the lint reports."
            }
            Self::ModuleFileInsideBlock => {
                "A module file declared inside a block is outside the analysed module tree. Declare the module at module level."
            }
            Self::MisplacedBlanketImpl { .. } => {
                "A blanket impl reaches every type its bounds admit, so it belongs beside its trait. Move the impl into the file that defines the trait."
            }
            Self::MisplacedImpl { .. } => {
                "All behavior of a type is read in one place, the file that defines it. Move the impl block into that file."
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
                "Integration test files hold only tests. Move helpers and fixtures into a library crate."
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
                "Every name must resolve for the analysis to be complete. Declare the crate as a dependency or remove the declaration."
            }
            Self::UnresolvedMacroCall => {
                "Every name must resolve for the analysis to be complete. Define or import the macro."
            }
            Self::UnresolvedModule => {
                "Every name must resolve for the analysis to be complete. Create the module file or remove the declaration."
            }
            Self::UnresolvedPath => {
                "Every name must resolve for the analysis to be complete. Fix the path so it resolves."
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
            Self::ConfigurationSelectionMacro => formatter.write_str(
                "Configuration selection macro keeps the arms it does not select out of the analysis",
            ),
            Self::FailedMacroExpansion => formatter.write_str("Macro expansion failed"),
            Self::GlobImport => formatter.write_str("Glob import"),
            Self::InvalidMacroDefinition => formatter.write_str("Macro definition is invalid"),
            Self::ImplWithoutLocalType => formatter
                .write_str("Impl block does not implement a struct, enum or union of this crate"),
            Self::LintSuppression => formatter.write_str("Attribute suppresses lints"),
            Self::ModuleFileInsideBlock => {
                formatter.write_str("Module file declared inside a block is not analysed")
            }
            Self::MisplacedBlanketImpl { trait_file } => {
                write!(formatter, "Blanket impl belongs in {trait_file}")
            }
            Self::MisplacedImpl { type_file } => {
                write!(formatter, "Impl block belongs in {type_file}")
            }
            Self::NegatedConfiguration => formatter.write_str(
                "Configuration predicate negates a feature, test or debug_assertions",
            ),
            Self::NonConstantInConstantsModule => {
                formatter.write_str("Constants module holds a public item that is not a constant")
            }
            Self::NonDocComment => formatter.write_str("Comment is not a doc comment"),
            Self::NonTestItemInTestTarget => formatter.write_str(
                "Integration test target holds an item that is neither a test, a constant nor an import",
            ),
            Self::PublicAssociatedConstant => write!(
                formatter,
                "Public associated constant belongs in crate::{CONSTANTS_MODULE_NAME} as a free constant"
            ),
            Self::PublicConstantOutsideConstantsModule => write!(
                formatter,
                "Public constant belongs in crate::{CONSTANTS_MODULE_NAME}"
            ),
            Self::Reexport => formatter.write_str("Re-export"),
            Self::UnimplementedBuiltinMacro => {
                formatter.write_str("Built-in macro is not implemented")
            }
            Self::UnmappedItem => formatter.write_str("Item cannot be mapped to its definition"),
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
