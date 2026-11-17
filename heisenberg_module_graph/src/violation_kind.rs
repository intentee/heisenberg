use std::fmt;
use std::fmt::Display;
use std::fmt::Formatter;

use crate::configuration_atom::ConfigurationAtom;
use crate::error_violation::ErrorViolation;
use crate::harness_violation::HarnessViolation;
use crate::item_placement::ItemPlacement;
use crate::layout_violation::LayoutViolation;
use crate::method_placement::MethodPlacement;
use crate::non_production_users::NonProductionUsers;
use crate::placement_violation::PlacementViolation;
use crate::rule_applicability::RuleApplicability;
use crate::test_violation::TestViolation;
use crate::used_item::UsedItem;

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum ViolationKind {
    AttributeAppliedInNoConfiguration,
    BenchmarkHarness(HarnessViolation),
    ConfigurationSelectionMacro,
    Error(ErrorViolation),
    FailedMacroExpansion,
    GlobImport,
    InvalidMacroDefinition,
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
    NegatedConfiguration,
    NonBenchmarkItemInBenchmarkTarget,
    NonDocComment,
    NonTestItemInTestTarget,
    Placement(PlacementViolation),
    ProceduralFunction {
        type_name: String,
    },
    ReferencedTest,
    Reexport,
    SingleLetterName,
    Test(TestViolation),
    TestOnlyCodeInProduction,
    ToolAttribute {
        tool: String,
    },
    TupleInSignature,
    UndeclaredEnvironmentVariable {
        variable: String,
    },
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

impl ViolationKind {
    #[must_use]
    pub const fn applicability(&self) -> RuleApplicability {
        match self {
            Self::Error(_) | Self::Placement(_) => RuleApplicability::ProductionCode,
            _ => RuleApplicability::AllCode,
        }
    }
}

impl Display for ViolationKind {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::BenchmarkHarness(violation) => violation.fmt(formatter),
            Self::ConfigurationSelectionMacro => formatter.write_str(
                "Configuration selection macro keeps the arms it does not select out of the analysis",
            ),
            Self::Error(violation) => violation.fmt(formatter),
            Self::FailedMacroExpansion => formatter.write_str("Macro expansion failed"),
            Self::ItemUsedOnlyBy { item, users } => write!(formatter, "{item} {users}"),
            Self::GlobImport => formatter.write_str("Glob import"),
            Self::InvalidMacroDefinition => formatter.write_str("Macro definition is invalid"),
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
            Self::NegatedConfiguration => formatter.write_str(
                "Configuration predicate negates a feature, test or debug_assertions",
            ),
            Self::NonBenchmarkItemInBenchmarkTarget => formatter.write_str(
                "Benchmark target holds an item that is neither its entry point, a constant, an import nor a private helper used only in its own file",
            ),
            Self::NonDocComment => formatter.write_str("Comment is not a doc comment"),
            Self::Placement(violation) => violation.fmt(formatter),
            Self::NonTestItemInTestTarget => formatter.write_str(
                "Integration test target holds an item that is neither a test, a constant, an import nor a private helper used only in its own file",
            ),
            Self::ProceduralFunction { type_name } => {
                write!(formatter, "Function takes {type_name} without belonging to it")
            }
            Self::ReferencedTest => formatter.write_str("Test is referenced by other code"),
            Self::Reexport => formatter.write_str("Re-export"),
            Self::SingleLetterName => formatter.write_str("Name is a single character"),
            Self::TestOnlyCodeInProduction => formatter
                .write_str("Code depends on cfg(test) outside a test module"),
            Self::Test(violation) => violation.fmt(formatter),
            Self::TupleInSignature => formatter.write_str("Tuple type in a signature"),
            Self::UndeclaredEnvironmentVariable { variable } => write!(
                formatter,
                "Environment variable {variable} is declared by neither cargo, the cargo configuration nor the build script"
            ),
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
