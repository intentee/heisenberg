use std::fmt;
use std::fmt::Display;
use std::fmt::Formatter;

use crate::configuration_atom::ConfigurationAtom;
use crate::constants::CONSTANTS_MODULE_NAME;
use crate::error_violation::ErrorViolation;
use crate::harness_violation::HarnessViolation;
use crate::impl_placement::ImplPlacement;
use crate::item_placement::ItemPlacement;
use crate::layout_violation::LayoutViolation;
use crate::method_placement::MethodPlacement;
use crate::non_production_users::NonProductionUsers;
use crate::test_violation::TestViolation;
use crate::used_item::UsedItem;

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum ViolationKind {
    AdditionalPublicItem,
    AttributeAppliedInNoConfiguration,
    BenchmarkHarness(HarnessViolation),
    ConfigurationSelectionMacro,
    Error(ErrorViolation),
    FailedMacroExpansion,
    GlobImport,
    InvalidMacroDefinition,
    ImplWithoutLocalType,
    IncludedSourceFile {
        file: String,
    },
    ItemUsedOnlyBy {
        item: UsedItem,
        users: NonProductionUsers,
    },
    ItemOutOfOrder {
        placement: ItemPlacement,
    },
    Layout(LayoutViolation),
    LintSuppression,
    MethodOutOfOrder {
        placement: MethodPlacement,
    },
    ModuleFileInsideBlock,
    ModuleNotNamedAfterItsItem {
        expected: String,
        item: String,
        module: String,
    },
    MisplacedImpl(ImplPlacement),
    NegatedConfiguration,
    NonBenchmarkItemInBenchmarkTarget,
    NonConstantInConstantsModule,
    NonDocComment,
    NonTestItemInTestTarget,
    ProceduralFunction {
        type_name: String,
    },
    PublicAssociatedConstant,
    PublicConstantOutsideConstantsModule,
    ReferencedTest,
    Reexport,
    SingleLetterName,
    Test(TestViolation),
    TestOnlyCodeInProduction,
    ToolAttribute {
        tool: String,
    },
    TupleInSignature,
    UndestructuredParameter {
        type_name: String,
    },
    UnimplementedBuiltinMacro,
    UnmappedItem,
    UnparsableMacroExpansion,
    UnprefixedTypeParameter,
    UnresolvedExternCrate,
    UnresolvedMacroCall,
    UnresolvedMethodCall,
    UnresolvedModule,
    UnresolvedPath,
    UnsafeCode,
    VacuousNegatedConfiguration {
        atom: ConfigurationAtom,
    },
}

impl Display for ViolationKind {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::AdditionalPublicItem => {
                formatter.write_str("Module already has a public item")
            }
            Self::BenchmarkHarness(violation) => violation.fmt(formatter),
            Self::ConfigurationSelectionMacro => formatter.write_str(
                "Configuration selection macro keeps the arms it does not select out of the analysis",
            ),
            Self::Error(violation) => violation.fmt(formatter),
            Self::FailedMacroExpansion => formatter.write_str("Macro expansion failed"),
            Self::ItemUsedOnlyBy { item, users } => write!(formatter, "{item} {users}"),
            Self::GlobImport => formatter.write_str("Glob import"),
            Self::InvalidMacroDefinition => formatter.write_str("Macro definition is invalid"),
            Self::ImplWithoutLocalType => formatter
                .write_str("Impl block belongs to no single type or trait of this crate"),
            Self::IncludedSourceFile { file } => {
                write!(formatter, "Included file {file} is not a build script output")
            }
            Self::ItemOutOfOrder { placement } => placement.fmt(formatter),
            Self::Layout(violation) => violation.fmt(formatter),
            Self::LintSuppression => formatter.write_str("Attribute suppresses lints"),
            Self::MethodOutOfOrder { placement } => placement.fmt(formatter),
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
            Self::MisplacedImpl(placement) => placement.fmt(formatter),
            Self::NegatedConfiguration => formatter.write_str(
                "Configuration predicate negates a feature, test or debug_assertions",
            ),
            Self::NonBenchmarkItemInBenchmarkTarget => formatter.write_str(
                "Benchmark target holds an item that is neither its entry point, a constant, an import nor a private helper used only in its own file",
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
            Self::ReferencedTest => formatter.write_str("Test is referenced by other code"),
            Self::Reexport => formatter.write_str("Re-export"),
            Self::SingleLetterName => formatter.write_str("Name is a single character"),
            Self::TestOnlyCodeInProduction => formatter
                .write_str("Code depends on cfg(test) outside a test module"),
            Self::Test(violation) => violation.fmt(formatter),
            Self::TupleInSignature => formatter.write_str("Tuple type in a signature"),
            Self::UndestructuredParameter { type_name } => write!(
                formatter,
                "Parameter is used only through the fields of {type_name}"
            ),
            Self::UnimplementedBuiltinMacro => {
                formatter.write_str("Built-in macro is not implemented")
            }
            Self::UnmappedItem => formatter.write_str("Item cannot be mapped to its definition"),
            Self::UnparsableMacroExpansion => {
                formatter.write_str("Macro expansion cannot be parsed")
            }
            Self::UnprefixedTypeParameter => {
                formatter.write_str("Type parameter name is not T followed by a capitalized word")
            }
            Self::UnresolvedExternCrate => formatter.write_str("Extern crate compiles, but rust-analyzer cannot resolve it"),
            Self::UnresolvedMacroCall => formatter.write_str("Macro call compiles, but rust-analyzer cannot resolve its macro"),
            Self::UnresolvedMethodCall => formatter.write_str("Method call compiles, but rust-analyzer cannot resolve its method"),
            Self::UnresolvedModule => formatter.write_str("Module declaration compiles, but rust-analyzer cannot resolve its file"),
            Self::UnresolvedPath => formatter.write_str("Path compiles, but rust-analyzer cannot resolve it"),
            Self::UnsafeCode => {
                formatter.write_str("Unsafe code in a crate that integrates no foreign code")
            }
            Self::AttributeAppliedInNoConfiguration => {
                formatter.write_str("Conditional attribute applies in no declared configuration")
            }
            Self::ToolAttribute { tool } => write!(formatter, "Attribute of the {tool} tool"),
            Self::VacuousNegatedConfiguration { atom } => write!(
                formatter,
                "Predicate negates {atom}, which no declared configuration sets"
            ),
        }
    }
}
