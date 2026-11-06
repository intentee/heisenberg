use std::iter;

use clap::ValueEnum;
use clap::builder::PossibleValue;

use heisenberg_module_graph::violation_kind::ViolationKind;
use heisenberg_project_model::clippy_lint_path::clippy_lint_path;
use heisenberg_project_model::constants::CLIPPY_CONFIGURATION_FILE;
use heisenberg_project_model::constants::CLIPPY_DENIED_LINTS;
use heisenberg_project_model::constants::CLIPPY_HIDDEN_CONFIGURATION_FILE;
use heisenberg_project_model::constants::CLIPPY_TEST_ALLOWANCES;
use heisenberg_project_model::package_violation::PackageViolation;

use crate::constants::ADDITIONAL_PUBLIC_ITEM;
use crate::constants::CLIPPY_CONFIGURATION_AMBIGUOUS;
use crate::constants::CLIPPY_CONFIGURATION_MISSING;
use crate::constants::CLIPPY_LINT_NOT_DENIED;
use crate::constants::CLIPPY_TEST_ALLOWANCE_MISSING;
use crate::constants::CODE_ACTIVE_ON_NO_TARGET;
use crate::constants::CONFIGURATION_SELECTION_MACRO;
use crate::constants::DUPLICATED_FUNCTION_BODY;
use crate::constants::FAILED_MACRO_EXPANSION;
use crate::constants::FUNCTION_USED_ONLY_BY_TESTS;
use crate::constants::GLOB_IMPORT;
use crate::constants::IMPL_WITHOUT_LOCAL_TYPE;
use crate::constants::INVALID_MACRO_DEFINITION;
use crate::constants::ITEM_OUT_OF_ORDER;
use crate::constants::LINT_ALLOWED_IN_MANIFEST;
use crate::constants::LINT_SUPPRESSION;
use crate::constants::METHOD_OUT_OF_ORDER;
use crate::constants::MISPLACED_BLANKET_IMPL;
use crate::constants::MISPLACED_FOREIGN_TYPE_IMPL;
use crate::constants::MISPLACED_IMPL;
use crate::constants::MISPLACED_TRAIT_OBJECT_IMPL;
use crate::constants::MODULE_CYCLE;
use crate::constants::MODULE_FILE_INSIDE_BLOCK;
use crate::constants::MODULE_NOT_NAMED_AFTER_ITS_ITEM;
use crate::constants::NEGATED_CONFIGURATION;
use crate::constants::NON_CONSTANT_IN_CONSTANTS_MODULE;
use crate::constants::NON_DOC_COMMENT;
use crate::constants::NON_TEST_ITEM_IN_TEST_TARGET;
use crate::constants::PACKAGE_DEPENDENCY_CYCLE;
use crate::constants::PACKAGE_DEPENDS_ON_ITSELF;
use crate::constants::PROCEDURAL_FUNCTION;
use crate::constants::PUBLIC_ASSOCIATED_CONSTANT;
use crate::constants::PUBLIC_CONSTANT_OUTSIDE_CONSTANTS_MODULE;
use crate::constants::PUBLISHABLE_TEST_CRATE;
use crate::constants::REEXPORT;
use crate::constants::SHARED_MODULE_FILE;
use crate::constants::SINGLE_LETTER_NAME;
use crate::constants::TEST_CRATE_DECLARED_TWICE;
use crate::constants::TEST_CRATE_NOT_A_MEMBER;
use crate::constants::TEST_CRATE_USED_BY_PRODUCTION;
use crate::constants::TEST_ONLY_CODE_IN_PRODUCTION;
use crate::constants::TESTS_ONLY_PACKAGE_WITHOUT_TEST_LIBRARIES;
use crate::constants::TUPLE_IN_SIGNATURE;
use crate::constants::UNDESTRUCTURED_PARAMETER;
use crate::constants::UNIMPLEMENTED_BUILTIN_MACRO;
use crate::constants::UNMAPPED_ITEM;
use crate::constants::UNPARSABLE_MACRO_EXPANSION;
use crate::constants::UNPREFIXED_TYPE_PARAMETER;
use crate::constants::UNRESOLVED_EXTERN_CRATE;
use crate::constants::UNRESOLVED_MACRO_CALL;
use crate::constants::UNRESOLVED_METHOD_CALL;
use crate::constants::UNRESOLVED_MODULE;
use crate::constants::UNRESOLVED_PATH;
use crate::issue_text::IssueText;

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum IssueType {
    AdditionalPublicItem,
    ClippyConfigurationAmbiguous,
    ClippyConfigurationMissing,
    ClippyLintNotDenied,
    ClippyTestAllowanceMissing,
    CodeActiveOnNoTarget,
    ConfigurationSelectionMacro,
    DuplicatedFunctionBody,
    FailedMacroExpansion,
    FunctionUsedOnlyByTests,
    GlobImport,
    ImplWithoutLocalType,
    InvalidMacroDefinition,
    ItemOutOfOrder,
    LintAllowedInManifest,
    LintSuppression,
    MethodOutOfOrder,
    MisplacedBlanketImpl,
    MisplacedForeignTypeImpl,
    MisplacedImpl,
    MisplacedTraitObjectImpl,
    ModuleCycle,
    ModuleFileInsideBlock,
    ModuleNotNamedAfterItsItem,
    NegatedConfiguration,
    NonConstantInConstantsModule,
    NonDocComment,
    NonTestItemInTestTarget,
    PackageDependencyCycle,
    PackageDependsOnItself,
    ProceduralFunction,
    PublicAssociatedConstant,
    PublicConstantOutsideConstantsModule,
    PublishableTestCrate,
    Reexport,
    SharedModuleFile,
    SingleLetterName,
    TestCrateDeclaredTwice,
    TestCrateNotAMember,
    TestCrateUsedByProduction,
    TestOnlyCodeInProduction,
    TestsOnlyPackageWithoutTestLibraries,
    TupleInSignature,
    UndestructuredParameter,
    UnimplementedBuiltinMacro,
    UnmappedItem,
    UnparsableMacroExpansion,
    UnprefixedTypeParameter,
    UnresolvedExternCrate,
    UnresolvedMacroCall,
    UnresolvedMethodCall,
    UnresolvedModule,
    UnresolvedPath,
}

impl IssueType {
    #[must_use]
    pub fn explanation(self) -> String {
        let IssueText {
            definition,
            name,
            purpose,
            summary,
        } = self.text();

        iter::once(format!("{name}: {summary}"))
            .chain([definition.to_owned()])
            .chain(self.values())
            .chain([purpose.to_owned()])
            .collect::<Vec<String>>()
            .join(
                "

",
            )
            + "
"
    }

    #[must_use]
    pub const fn text(self) -> IssueText {
        match self {
            Self::AdditionalPublicItem => ADDITIONAL_PUBLIC_ITEM,
            Self::ClippyConfigurationAmbiguous => CLIPPY_CONFIGURATION_AMBIGUOUS,
            Self::ClippyConfigurationMissing => CLIPPY_CONFIGURATION_MISSING,
            Self::ClippyLintNotDenied => CLIPPY_LINT_NOT_DENIED,
            Self::ClippyTestAllowanceMissing => CLIPPY_TEST_ALLOWANCE_MISSING,
            Self::CodeActiveOnNoTarget => CODE_ACTIVE_ON_NO_TARGET,
            Self::ConfigurationSelectionMacro => CONFIGURATION_SELECTION_MACRO,
            Self::DuplicatedFunctionBody => DUPLICATED_FUNCTION_BODY,
            Self::FailedMacroExpansion => FAILED_MACRO_EXPANSION,
            Self::FunctionUsedOnlyByTests => FUNCTION_USED_ONLY_BY_TESTS,
            Self::GlobImport => GLOB_IMPORT,
            Self::ImplWithoutLocalType => IMPL_WITHOUT_LOCAL_TYPE,
            Self::InvalidMacroDefinition => INVALID_MACRO_DEFINITION,
            Self::ItemOutOfOrder => ITEM_OUT_OF_ORDER,
            Self::LintAllowedInManifest => LINT_ALLOWED_IN_MANIFEST,
            Self::LintSuppression => LINT_SUPPRESSION,
            Self::MethodOutOfOrder => METHOD_OUT_OF_ORDER,
            Self::MisplacedBlanketImpl => MISPLACED_BLANKET_IMPL,
            Self::MisplacedForeignTypeImpl => MISPLACED_FOREIGN_TYPE_IMPL,
            Self::MisplacedImpl => MISPLACED_IMPL,
            Self::MisplacedTraitObjectImpl => MISPLACED_TRAIT_OBJECT_IMPL,
            Self::ModuleCycle => MODULE_CYCLE,
            Self::ModuleFileInsideBlock => MODULE_FILE_INSIDE_BLOCK,
            Self::ModuleNotNamedAfterItsItem => MODULE_NOT_NAMED_AFTER_ITS_ITEM,
            Self::NegatedConfiguration => NEGATED_CONFIGURATION,
            Self::NonConstantInConstantsModule => NON_CONSTANT_IN_CONSTANTS_MODULE,
            Self::NonDocComment => NON_DOC_COMMENT,
            Self::NonTestItemInTestTarget => NON_TEST_ITEM_IN_TEST_TARGET,
            Self::PackageDependencyCycle => PACKAGE_DEPENDENCY_CYCLE,
            Self::PackageDependsOnItself => PACKAGE_DEPENDS_ON_ITSELF,
            Self::ProceduralFunction => PROCEDURAL_FUNCTION,
            Self::PublicAssociatedConstant => PUBLIC_ASSOCIATED_CONSTANT,
            Self::PublicConstantOutsideConstantsModule => PUBLIC_CONSTANT_OUTSIDE_CONSTANTS_MODULE,
            Self::PublishableTestCrate => PUBLISHABLE_TEST_CRATE,
            Self::Reexport => REEXPORT,
            Self::SharedModuleFile => SHARED_MODULE_FILE,
            Self::SingleLetterName => SINGLE_LETTER_NAME,
            Self::TestCrateDeclaredTwice => TEST_CRATE_DECLARED_TWICE,
            Self::TestCrateNotAMember => TEST_CRATE_NOT_A_MEMBER,
            Self::TestCrateUsedByProduction => TEST_CRATE_USED_BY_PRODUCTION,
            Self::TestOnlyCodeInProduction => TEST_ONLY_CODE_IN_PRODUCTION,
            Self::TestsOnlyPackageWithoutTestLibraries => TESTS_ONLY_PACKAGE_WITHOUT_TEST_LIBRARIES,
            Self::TupleInSignature => TUPLE_IN_SIGNATURE,
            Self::UndestructuredParameter => UNDESTRUCTURED_PARAMETER,
            Self::UnimplementedBuiltinMacro => UNIMPLEMENTED_BUILTIN_MACRO,
            Self::UnmappedItem => UNMAPPED_ITEM,
            Self::UnparsableMacroExpansion => UNPARSABLE_MACRO_EXPANSION,
            Self::UnprefixedTypeParameter => UNPREFIXED_TYPE_PARAMETER,
            Self::UnresolvedExternCrate => UNRESOLVED_EXTERN_CRATE,
            Self::UnresolvedMacroCall => UNRESOLVED_MACRO_CALL,
            Self::UnresolvedMethodCall => UNRESOLVED_METHOD_CALL,
            Self::UnresolvedModule => UNRESOLVED_MODULE,
            Self::UnresolvedPath => UNRESOLVED_PATH,
        }
    }

    fn values(self) -> Option<String> {
        match self {
            Self::ClippyConfigurationAmbiguous | Self::ClippyConfigurationMissing => Some(format!(
                "Clippy configuration files: {CLIPPY_CONFIGURATION_FILE}, {CLIPPY_HIDDEN_CONFIGURATION_FILE}"
            )),
            Self::ClippyLintNotDenied | Self::LintSuppression => Some(format!(
                "Required lints: {}",
                CLIPPY_DENIED_LINTS
                    .map(|lint| clippy_lint_path(lint.name))
                    .join(", ")
            )),
            Self::ClippyTestAllowanceMissing => Some(format!(
                "Test allowances: {}",
                CLIPPY_TEST_ALLOWANCES.join(", ")
            )),
            _ => None,
        }
    }
}

impl From<&ViolationKind> for IssueType {
    fn from(kind: &ViolationKind) -> Self {
        match kind {
            ViolationKind::AdditionalPublicItem => Self::AdditionalPublicItem,
            ViolationKind::ConfigurationSelectionMacro => Self::ConfigurationSelectionMacro,
            ViolationKind::FailedMacroExpansion => Self::FailedMacroExpansion,
            ViolationKind::FunctionUsedOnlyByTests => Self::FunctionUsedOnlyByTests,
            ViolationKind::GlobImport => Self::GlobImport,
            ViolationKind::ImplWithoutLocalType => Self::ImplWithoutLocalType,
            ViolationKind::ItemOutOfOrder { .. } => Self::ItemOutOfOrder,
            ViolationKind::InvalidMacroDefinition => Self::InvalidMacroDefinition,
            ViolationKind::LintSuppression => Self::LintSuppression,
            ViolationKind::MethodOutOfOrder { .. } => Self::MethodOutOfOrder,
            ViolationKind::MisplacedBlanketImpl { .. } => Self::MisplacedBlanketImpl,
            ViolationKind::MisplacedForeignTypeImpl { .. } => Self::MisplacedForeignTypeImpl,
            ViolationKind::MisplacedImpl { .. } => Self::MisplacedImpl,
            ViolationKind::MisplacedTraitObjectImpl { .. } => Self::MisplacedTraitObjectImpl,
            ViolationKind::ModuleFileInsideBlock => Self::ModuleFileInsideBlock,
            ViolationKind::ModuleNotNamedAfterItsItem { .. } => Self::ModuleNotNamedAfterItsItem,
            ViolationKind::NegatedConfiguration => Self::NegatedConfiguration,
            ViolationKind::NonConstantInConstantsModule => Self::NonConstantInConstantsModule,
            ViolationKind::NonDocComment => Self::NonDocComment,
            ViolationKind::NonTestItemInTestTarget => Self::NonTestItemInTestTarget,
            ViolationKind::ProceduralFunction { .. } => Self::ProceduralFunction,
            ViolationKind::PublicAssociatedConstant => Self::PublicAssociatedConstant,
            ViolationKind::PublicConstantOutsideConstantsModule => {
                Self::PublicConstantOutsideConstantsModule
            }
            ViolationKind::Reexport => Self::Reexport,
            ViolationKind::SingleLetterName => Self::SingleLetterName,
            ViolationKind::TestOnlyCodeInProduction => Self::TestOnlyCodeInProduction,
            ViolationKind::TupleInSignature => Self::TupleInSignature,
            ViolationKind::UndestructuredParameter { .. } => Self::UndestructuredParameter,
            ViolationKind::UnimplementedBuiltinMacro => Self::UnimplementedBuiltinMacro,
            ViolationKind::UnmappedItem => Self::UnmappedItem,
            ViolationKind::UnparsableMacroExpansion => Self::UnparsableMacroExpansion,
            ViolationKind::UnprefixedTypeParameter => Self::UnprefixedTypeParameter,
            ViolationKind::UnresolvedExternCrate => Self::UnresolvedExternCrate,
            ViolationKind::UnresolvedMacroCall => Self::UnresolvedMacroCall,
            ViolationKind::UnresolvedMethodCall => Self::UnresolvedMethodCall,
            ViolationKind::UnresolvedModule => Self::UnresolvedModule,
            ViolationKind::UnresolvedPath => Self::UnresolvedPath,
        }
    }
}

impl From<&PackageViolation> for IssueType {
    fn from(violation: &PackageViolation) -> Self {
        match violation {
            PackageViolation::ClippyConfigurationAmbiguous { .. } => {
                Self::ClippyConfigurationAmbiguous
            }
            PackageViolation::ClippyConfigurationMissing { .. } => Self::ClippyConfigurationMissing,
            PackageViolation::ClippyLintNotDenied { .. } => Self::ClippyLintNotDenied,
            PackageViolation::ClippyTestAllowanceMissing { .. } => Self::ClippyTestAllowanceMissing,
            PackageViolation::LintAllowedInManifest { .. } => Self::LintAllowedInManifest,
            PackageViolation::PackageDependencyCycle { .. } => Self::PackageDependencyCycle,
            PackageViolation::PackageDependsOnItself { .. } => Self::PackageDependsOnItself,
            PackageViolation::PublishableTestCrate { .. } => Self::PublishableTestCrate,
            PackageViolation::TestCrateDeclaredTwice { .. } => Self::TestCrateDeclaredTwice,
            PackageViolation::TestCrateNotAMember { .. } => Self::TestCrateNotAMember,
            PackageViolation::TestCrateUsedByProduction { .. } => Self::TestCrateUsedByProduction,
            PackageViolation::TestsOnlyPackageWithoutTestLibraries { .. } => {
                Self::TestsOnlyPackageWithoutTestLibraries
            }
        }
    }
}

impl ValueEnum for IssueType {
    fn to_possible_value(&self) -> Option<PossibleValue> {
        Some(PossibleValue::new(self.text().name))
    }

    fn value_variants<'variants>() -> &'variants [Self] {
        &[
            Self::AdditionalPublicItem,
            Self::ClippyConfigurationAmbiguous,
            Self::ClippyConfigurationMissing,
            Self::ClippyLintNotDenied,
            Self::ClippyTestAllowanceMissing,
            Self::CodeActiveOnNoTarget,
            Self::ConfigurationSelectionMacro,
            Self::DuplicatedFunctionBody,
            Self::FailedMacroExpansion,
            Self::FunctionUsedOnlyByTests,
            Self::GlobImport,
            Self::ImplWithoutLocalType,
            Self::InvalidMacroDefinition,
            Self::ItemOutOfOrder,
            Self::LintAllowedInManifest,
            Self::LintSuppression,
            Self::MethodOutOfOrder,
            Self::MisplacedBlanketImpl,
            Self::MisplacedForeignTypeImpl,
            Self::MisplacedImpl,
            Self::MisplacedTraitObjectImpl,
            Self::ModuleCycle,
            Self::ModuleFileInsideBlock,
            Self::ModuleNotNamedAfterItsItem,
            Self::NegatedConfiguration,
            Self::NonConstantInConstantsModule,
            Self::NonDocComment,
            Self::NonTestItemInTestTarget,
            Self::PackageDependencyCycle,
            Self::PackageDependsOnItself,
            Self::ProceduralFunction,
            Self::PublicAssociatedConstant,
            Self::PublicConstantOutsideConstantsModule,
            Self::PublishableTestCrate,
            Self::Reexport,
            Self::SharedModuleFile,
            Self::SingleLetterName,
            Self::TestCrateDeclaredTwice,
            Self::TestCrateNotAMember,
            Self::TestCrateUsedByProduction,
            Self::TestOnlyCodeInProduction,
            Self::TestsOnlyPackageWithoutTestLibraries,
            Self::TupleInSignature,
            Self::UndestructuredParameter,
            Self::UnimplementedBuiltinMacro,
            Self::UnmappedItem,
            Self::UnparsableMacroExpansion,
            Self::UnprefixedTypeParameter,
            Self::UnresolvedExternCrate,
            Self::UnresolvedMacroCall,
            Self::UnresolvedMethodCall,
            Self::UnresolvedModule,
            Self::UnresolvedPath,
        ]
    }
}
