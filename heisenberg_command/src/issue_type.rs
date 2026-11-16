use std::iter;

use clap::ValueEnum;
use clap::builder::PossibleValue;
use humantime::format_duration;

use heisenberg_module_graph::harness_violation::HarnessViolation;
use heisenberg_module_graph::impl_placement::ImplPlacement;
use heisenberg_module_graph::non_production_users::NonProductionUsers;
use heisenberg_module_graph::test_violation::TestViolation;
use heisenberg_module_graph::violation_kind::ViolationKind;
use heisenberg_project_model::benchmark_target_violation::BenchmarkTargetViolation;
use heisenberg_project_model::confinement_violation::ConfinementViolation;
use heisenberg_project_model::constants::CLIPPY_CONFIGURATION_FILE;
use heisenberg_project_model::constants::CLIPPY_HIDDEN_CONFIGURATION_FILE;
use heisenberg_project_model::constants::MAXIMUM_SLOW_TEST_PERIOD;
use heisenberg_project_model::constants::MAXIMUM_TEST_DURATION;
use heisenberg_project_model::constants::PANICKING_LINTS;
use heisenberg_project_model::constants::REQUIRED_CLIPPY_SETTINGS;
use heisenberg_project_model::constants::REQUIRED_LINT_GROUPS;
use heisenberg_project_model::constants::RESTRICTING_CLIPPY_SETTINGS;
use heisenberg_project_model::constants::VETTED_BENCHMARK_HARNESS_METHODS;
use heisenberg_project_model::constants::VETTED_NEXTEST_PROFILE_SETTINGS;
use heisenberg_project_model::constants::VETTED_NEXTEST_SETTINGS;
use heisenberg_project_model::constants::VETTED_NEXTEST_SLOW_TIMEOUT_SETTINGS;
use heisenberg_project_model::harness_methods::HarnessMethods;
use heisenberg_project_model::libraryless_contents::LibrarylessContents;
use heisenberg_project_model::libraryless_package::LibrarylessPackage;
use heisenberg_project_model::nextest_presence_violation::NextestPresenceViolation;
use heisenberg_project_model::package_violation::PackageViolation;
use heisenberg_project_model::required_clippy_setting::RequiredClippySetting;

use crate::constants::ADDITIONAL_PUBLIC_ITEM;
use crate::constants::BENCHMARK_TARGET_EXCLUDED_FROM_BENCHMARKS;
use crate::constants::BENCHMARK_TARGET_RUN_BY_TESTS;
use crate::constants::BENCHMARK_TARGET_WITH_LIBTEST_HARNESS;
use crate::constants::BENCHMARK_TARGET_WITHOUT_CRITERION_BENCHMARK;
use crate::constants::BENCHMARKS_ONLY_PACKAGE_WITHOUT_LIBRARIES;
use crate::constants::CLIPPY_CONFIGURATION_AMBIGUOUS;
use crate::constants::CLIPPY_CONFIGURATION_MISSING;
use crate::constants::CODE_ACTIVE_IN_NO_CONFIGURATION;
use crate::constants::CODE_WITHOUT_COVERAGE_DATA;
use crate::constants::CONFIGURATION_SELECTION_MACRO;
use crate::constants::CONFINED_PACKAGE_REACHED_OUTSIDE_ITS_USERS;
use crate::constants::CONFINED_PACKAGE_USER_LISTED_TWICE;
use crate::constants::CONFINED_PACKAGE_USER_NOT_A_MEMBER;
use crate::constants::CONFINED_PACKAGE_USER_NOT_REACHING_IT;
use crate::constants::DUPLICATED_FUNCTION_BODY;
use crate::constants::FAILED_MACRO_EXPANSION;
use crate::constants::FUNCTION_USED_ONLY_BY_BENCHMARKS;
use crate::constants::FUNCTION_USED_ONLY_BY_TESTS;
use crate::constants::GLOB_IMPORT;
use crate::constants::IMPL_WITHOUT_LOCAL_TYPE;
use crate::constants::INCLUDED_SOURCE_FILE;
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
use crate::constants::MUST_USE_BENCHMARK_OUTPUT;
use crate::constants::NEGATED_CONFIGURATION;
use crate::constants::NEXTEST_CONFIGURATION_MISSING;
use crate::constants::NEXTEST_OVERRIDE;
use crate::constants::NEXTEST_TIMEOUT_NOT_ENFORCED;
use crate::constants::NON_BENCHMARK_ITEM_IN_BENCHMARK_TARGET;
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
use crate::constants::REFERENCED_TEST;
use crate::constants::REQUIRED_CLIPPY_SETTING_MISSING;
use crate::constants::REQUIRED_LINT_NOT_DENIED;
use crate::constants::SHARED_MODULE_FILE;
use crate::constants::SINGLE_LETTER_NAME;
use crate::constants::SPLIT_INTEGRATION_TESTS;
use crate::constants::TEST_CRATE_DECLARED_TWICE;
use crate::constants::TEST_CRATE_NOT_A_MEMBER;
use crate::constants::TEST_CRATE_USED_BY_PRODUCTION;
use crate::constants::TEST_ONLY_CODE_IN_PRODUCTION;
use crate::constants::TESTS_ONLY_PACKAGE_WITHOUT_TEST_LIBRARIES;
use crate::constants::TUPLE_IN_SIGNATURE;
use crate::constants::UNCOVERED_CODE;
use crate::constants::UNDESTRUCTURED_PARAMETER;
use crate::constants::UNDETERMINED_BENCHMARK_OUTPUT;
use crate::constants::UNIMPLEMENTED_BUILTIN_MACRO;
use crate::constants::UNKNOWN_CONFINED_PACKAGE;
use crate::constants::UNMAPPED_ITEM;
use crate::constants::UNNEEDED_NEXTEST_CONFIGURATION;
use crate::constants::UNPARSABLE_MACRO_EXPANSION;
use crate::constants::UNPREFIXED_TYPE_PARAMETER;
use crate::constants::UNREACHABLE_TEST_FILE;
use crate::constants::UNRESOLVED_EXTERN_CRATE;
use crate::constants::UNRESOLVED_MACRO_CALL;
use crate::constants::UNRESOLVED_METHOD_CALL;
use crate::constants::UNRESOLVED_MODULE;
use crate::constants::UNRESOLVED_PATH;
use crate::constants::UNSAFE_CODE;
use crate::constants::UNVETTED_CLIPPY_SETTING;
use crate::constants::UNVETTED_CRITERION_METHOD;
use crate::constants::UNVETTED_NEXTEST_SETTING;
use crate::error_issue::ErrorIssue;
use crate::issue_text::IssueText;
use crate::test_issue::TestIssue;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum IssueType {
    Error(ErrorIssue),
    Test(TestIssue),
    AdditionalPublicItem,
    BenchmarkTargetExcludedFromBenchmarks,
    BenchmarkTargetRunByTests,
    BenchmarkTargetWithLibtestHarness,
    BenchmarkTargetWithoutCriterionBenchmark,
    BenchmarksOnlyPackageWithoutLibraries,
    ClippyConfigurationAmbiguous,
    ClippyConfigurationMissing,
    CodeActiveInNoConfiguration,
    CodeWithoutCoverageData,
    ConfigurationSelectionMacro,
    ConfinedPackageReachedOutsideItsUsers,
    ConfinedPackageUserListedTwice,
    ConfinedPackageUserNotAMember,
    ConfinedPackageUserNotReachingIt,
    DuplicatedFunctionBody,
    FailedMacroExpansion,
    FunctionUsedOnlyByBenchmarks,
    FunctionUsedOnlyByTests,
    GlobImport,
    ImplWithoutLocalType,
    IncludedSourceFile,
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
    MustUseBenchmarkOutput,
    NegatedConfiguration,
    NextestConfigurationMissing,
    NextestOverride,
    NextestTimeoutNotEnforced,
    NonBenchmarkItemInBenchmarkTarget,
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
    ReferencedTest,
    RequiredClippySettingMissing,
    RequiredLintNotDenied,
    SharedModuleFile,
    SingleLetterName,
    SplitIntegrationTests,
    TestCrateDeclaredTwice,
    TestCrateNotAMember,
    TestCrateUsedByProduction,
    TestOnlyCodeInProduction,
    TestsOnlyPackageWithoutTestLibraries,
    TupleInSignature,
    UncoveredCode,
    UndestructuredParameter,
    UndeterminedBenchmarkOutput,
    UnimplementedBuiltinMacro,
    UnknownConfinedPackage,
    UnmappedItem,
    UnneededNextestConfiguration,
    UnparsableMacroExpansion,
    UnprefixedTypeParameter,
    UnreachableTestFile,
    UnresolvedExternCrate,
    UnresolvedMacroCall,
    UnresolvedMethodCall,
    UnresolvedModule,
    UnresolvedPath,
    UnsafeCode,
    UnvettedClippySetting,
    UnvettedCriterionMethod,
    UnvettedNextestSetting,
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
    pub const fn measures_coverage(self) -> bool {
        matches!(self, Self::CodeWithoutCoverageData | Self::UncoveredCode)
    }

    #[must_use]
    pub const fn text(self) -> IssueText {
        match self {
            Self::Error(issue) => issue.text(),
            Self::Test(issue) => issue.text(),
            Self::AdditionalPublicItem => ADDITIONAL_PUBLIC_ITEM,
            Self::BenchmarkTargetExcludedFromBenchmarks => {
                BENCHMARK_TARGET_EXCLUDED_FROM_BENCHMARKS
            }
            Self::BenchmarkTargetRunByTests => BENCHMARK_TARGET_RUN_BY_TESTS,
            Self::BenchmarkTargetWithLibtestHarness => BENCHMARK_TARGET_WITH_LIBTEST_HARNESS,
            Self::BenchmarkTargetWithoutCriterionBenchmark => {
                BENCHMARK_TARGET_WITHOUT_CRITERION_BENCHMARK
            }
            Self::BenchmarksOnlyPackageWithoutLibraries => {
                BENCHMARKS_ONLY_PACKAGE_WITHOUT_LIBRARIES
            }
            Self::ClippyConfigurationAmbiguous => CLIPPY_CONFIGURATION_AMBIGUOUS,
            Self::ClippyConfigurationMissing => CLIPPY_CONFIGURATION_MISSING,
            Self::CodeActiveInNoConfiguration => CODE_ACTIVE_IN_NO_CONFIGURATION,
            Self::CodeWithoutCoverageData => CODE_WITHOUT_COVERAGE_DATA,
            Self::ConfigurationSelectionMacro => CONFIGURATION_SELECTION_MACRO,
            Self::ConfinedPackageReachedOutsideItsUsers => {
                CONFINED_PACKAGE_REACHED_OUTSIDE_ITS_USERS
            }
            Self::ConfinedPackageUserListedTwice => CONFINED_PACKAGE_USER_LISTED_TWICE,
            Self::ConfinedPackageUserNotAMember => CONFINED_PACKAGE_USER_NOT_A_MEMBER,
            Self::ConfinedPackageUserNotReachingIt => CONFINED_PACKAGE_USER_NOT_REACHING_IT,
            Self::DuplicatedFunctionBody => DUPLICATED_FUNCTION_BODY,
            Self::FailedMacroExpansion => FAILED_MACRO_EXPANSION,
            Self::FunctionUsedOnlyByBenchmarks => FUNCTION_USED_ONLY_BY_BENCHMARKS,
            Self::FunctionUsedOnlyByTests => FUNCTION_USED_ONLY_BY_TESTS,
            Self::GlobImport => GLOB_IMPORT,
            Self::ImplWithoutLocalType => IMPL_WITHOUT_LOCAL_TYPE,
            Self::IncludedSourceFile => INCLUDED_SOURCE_FILE,
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
            Self::MustUseBenchmarkOutput => MUST_USE_BENCHMARK_OUTPUT,
            Self::NegatedConfiguration => NEGATED_CONFIGURATION,
            Self::NextestConfigurationMissing => NEXTEST_CONFIGURATION_MISSING,
            Self::NextestOverride => NEXTEST_OVERRIDE,
            Self::NextestTimeoutNotEnforced => NEXTEST_TIMEOUT_NOT_ENFORCED,
            Self::NonBenchmarkItemInBenchmarkTarget => NON_BENCHMARK_ITEM_IN_BENCHMARK_TARGET,
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
            Self::ReferencedTest => REFERENCED_TEST,
            Self::RequiredClippySettingMissing => REQUIRED_CLIPPY_SETTING_MISSING,
            Self::RequiredLintNotDenied => REQUIRED_LINT_NOT_DENIED,
            Self::SharedModuleFile => SHARED_MODULE_FILE,
            Self::SingleLetterName => SINGLE_LETTER_NAME,
            Self::SplitIntegrationTests => SPLIT_INTEGRATION_TESTS,
            Self::TestCrateDeclaredTwice => TEST_CRATE_DECLARED_TWICE,
            Self::TestCrateNotAMember => TEST_CRATE_NOT_A_MEMBER,
            Self::TestCrateUsedByProduction => TEST_CRATE_USED_BY_PRODUCTION,
            Self::TestOnlyCodeInProduction => TEST_ONLY_CODE_IN_PRODUCTION,
            Self::TestsOnlyPackageWithoutTestLibraries => TESTS_ONLY_PACKAGE_WITHOUT_TEST_LIBRARIES,
            Self::TupleInSignature => TUPLE_IN_SIGNATURE,
            Self::UncoveredCode => UNCOVERED_CODE,
            Self::UndestructuredParameter => UNDESTRUCTURED_PARAMETER,
            Self::UndeterminedBenchmarkOutput => UNDETERMINED_BENCHMARK_OUTPUT,
            Self::UnimplementedBuiltinMacro => UNIMPLEMENTED_BUILTIN_MACRO,
            Self::UnknownConfinedPackage => UNKNOWN_CONFINED_PACKAGE,
            Self::UnmappedItem => UNMAPPED_ITEM,
            Self::UnneededNextestConfiguration => UNNEEDED_NEXTEST_CONFIGURATION,
            Self::UnparsableMacroExpansion => UNPARSABLE_MACRO_EXPANSION,
            Self::UnprefixedTypeParameter => UNPREFIXED_TYPE_PARAMETER,
            Self::UnreachableTestFile => UNREACHABLE_TEST_FILE,
            Self::UnresolvedExternCrate => UNRESOLVED_EXTERN_CRATE,
            Self::UnresolvedMacroCall => UNRESOLVED_MACRO_CALL,
            Self::UnresolvedMethodCall => UNRESOLVED_METHOD_CALL,
            Self::UnresolvedModule => UNRESOLVED_MODULE,
            Self::UnresolvedPath => UNRESOLVED_PATH,
            Self::UnsafeCode => UNSAFE_CODE,
            Self::UnvettedClippySetting => UNVETTED_CLIPPY_SETTING,
            Self::UnvettedCriterionMethod => UNVETTED_CRITERION_METHOD,
            Self::UnvettedNextestSetting => UNVETTED_NEXTEST_SETTING,
        }
    }

    fn values(self) -> Vec<String> {
        match self {
            Self::ClippyConfigurationAmbiguous | Self::ClippyConfigurationMissing => vec![format!(
                "Clippy configuration files: {CLIPPY_CONFIGURATION_FILE}, {CLIPPY_HIDDEN_CONFIGURATION_FILE}"
            )],
            Self::LintSuppression | Self::RequiredLintNotDenied => vec![
                format!(
                    "Required lint groups: {}",
                    REQUIRED_LINT_GROUPS.map(|lint| lint.path()).join(", ")
                ),
                format!(
                    "Panicking lints: {}",
                    PANICKING_LINTS.map(|lint| lint.path()).join(", ")
                ),
            ],
            Self::RequiredClippySettingMissing => vec![format!(
                "Required settings: {}",
                REQUIRED_CLIPPY_SETTINGS
                    .map(|RequiredClippySetting { name, value }| format!("{name} = {value}"))
                    .join(", ")
            )],
            Self::NextestTimeoutNotEnforced => vec![format!(
                "Limits: slow after at most {}, terminated after at most {}",
                format_duration(MAXIMUM_SLOW_TEST_PERIOD),
                format_duration(MAXIMUM_TEST_DURATION)
            )],
            Self::UnvettedClippySetting => vec![format!(
                "Settings that only add restrictions: {}",
                RESTRICTING_CLIPPY_SETTINGS.join(", ")
            )],
            Self::UnvettedNextestSetting => vec![format!(
                "Vetted settings: {}; in a profile: {}; in a slow-timeout: {}",
                VETTED_NEXTEST_SETTINGS.join(", "),
                VETTED_NEXTEST_PROFILE_SETTINGS.join(", "),
                VETTED_NEXTEST_SLOW_TIMEOUT_SETTINGS.join(", ")
            )],
            Self::UnvettedCriterionMethod => vec![format!(
                "Vetted methods: {}",
                VETTED_BENCHMARK_HARNESS_METHODS
                    .iter()
                    .map(|HarnessMethods { methods, type_name }| format!(
                        "{type_name}::{{{}}}",
                        methods.join(", ")
                    ))
                    .collect::<Vec<String>>()
                    .join(", ")
            )],
            _ => Vec::new(),
        }
    }
}

impl From<&ViolationKind> for IssueType {
    fn from(kind: &ViolationKind) -> Self {
        match kind {
            ViolationKind::AdditionalPublicItem => Self::AdditionalPublicItem,
            ViolationKind::BenchmarkHarness(HarnessViolation::MustUseOutput { .. }) => {
                Self::MustUseBenchmarkOutput
            }
            ViolationKind::BenchmarkHarness(HarnessViolation::TargetWithoutBenchmark) => {
                Self::BenchmarkTargetWithoutCriterionBenchmark
            }
            ViolationKind::BenchmarkHarness(HarnessViolation::UndeterminedOutput { .. }) => {
                Self::UndeterminedBenchmarkOutput
            }
            ViolationKind::BenchmarkHarness(HarnessViolation::UnvettedMethod { .. }) => {
                Self::UnvettedCriterionMethod
            }
            ViolationKind::Error(violation) => Self::Error(ErrorIssue::from(violation)),
            ViolationKind::FunctionUsedOnlyBy(NonProductionUsers::Benchmarks) => {
                Self::FunctionUsedOnlyByBenchmarks
            }
            ViolationKind::Test(TestViolation::Ignored) => Self::Test(TestIssue::Ignored),
            ViolationKind::NonBenchmarkItemInBenchmarkTarget => {
                Self::NonBenchmarkItemInBenchmarkTarget
            }
            ViolationKind::ConfigurationSelectionMacro => Self::ConfigurationSelectionMacro,
            ViolationKind::FailedMacroExpansion => Self::FailedMacroExpansion,
            ViolationKind::FunctionUsedOnlyBy(NonProductionUsers::Tests) => {
                Self::FunctionUsedOnlyByTests
            }
            ViolationKind::GlobImport => Self::GlobImport,
            ViolationKind::ImplWithoutLocalType => Self::ImplWithoutLocalType,
            ViolationKind::ItemOutOfOrder { .. } => Self::ItemOutOfOrder,
            ViolationKind::InvalidMacroDefinition => Self::InvalidMacroDefinition,
            ViolationKind::LintSuppression => Self::LintSuppression,
            ViolationKind::MethodOutOfOrder { .. } => Self::MethodOutOfOrder,
            ViolationKind::MisplacedImpl(ImplPlacement::Blanket { .. }) => {
                Self::MisplacedBlanketImpl
            }
            ViolationKind::MisplacedImpl(ImplPlacement::ForeignType { .. }) => {
                Self::MisplacedForeignTypeImpl
            }
            ViolationKind::MisplacedImpl(ImplPlacement::TraitObject { .. }) => {
                Self::MisplacedTraitObjectImpl
            }
            ViolationKind::MisplacedImpl(ImplPlacement::Type { .. }) => Self::MisplacedImpl,
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
            ViolationKind::Test(TestViolation::SerializationRoundTrip) => {
                Self::Test(TestIssue::SerializationRoundTrip)
            }
            ViolationKind::Test(TestViolation::Sleeping) => Self::Test(TestIssue::Sleeping),
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
            ViolationKind::IncludedSourceFile { .. } => Self::IncludedSourceFile,
            ViolationKind::UnsafeCode => Self::UnsafeCode,
            ViolationKind::ReferencedTest => Self::ReferencedTest,
        }
    }
}

impl From<&PackageViolation> for IssueType {
    fn from(violation: &PackageViolation) -> Self {
        match violation {
            PackageViolation::BenchmarkTarget(
                BenchmarkTargetViolation::ExcludedFromBenchmarks { .. },
            ) => Self::BenchmarkTargetExcludedFromBenchmarks,
            PackageViolation::BenchmarkTarget(BenchmarkTargetViolation::RunByTests { .. }) => {
                Self::BenchmarkTargetRunByTests
            }
            PackageViolation::BenchmarkTarget(BenchmarkTargetViolation::WithLibtestHarness {
                ..
            }) => Self::BenchmarkTargetWithLibtestHarness,
            PackageViolation::Confinement(ConfinementViolation::UnknownPackage { .. }) => {
                Self::UnknownConfinedPackage
            }
            PackageViolation::IntegrationTests(_) => Self::SplitIntegrationTests,
            PackageViolation::UnreachableTestFile { .. } => Self::UnreachableTestFile,
            PackageViolation::NextestPresence(NextestPresenceViolation::Unneeded { .. }) => {
                Self::UnneededNextestConfiguration
            }
            PackageViolation::Confinement(ConfinementViolation::UserListedTwice { .. }) => {
                Self::ConfinedPackageUserListedTwice
            }
            PackageViolation::Confinement(ConfinementViolation::UserNotAMember { .. }) => {
                Self::ConfinedPackageUserNotAMember
            }
            PackageViolation::Confinement(ConfinementViolation::UserNotReachingIt { .. }) => {
                Self::ConfinedPackageUserNotReachingIt
            }
            PackageViolation::ClippyConfigurationAmbiguous { .. } => {
                Self::ClippyConfigurationAmbiguous
            }
            PackageViolation::ClippyConfigurationMissing { .. } => Self::ClippyConfigurationMissing,
            PackageViolation::LintAllowedInManifest { .. } => Self::LintAllowedInManifest,
            PackageViolation::NextestPresence(NextestPresenceViolation::Missing { .. }) => {
                Self::NextestConfigurationMissing
            }
            PackageViolation::NextestOverride { .. } => Self::NextestOverride,
            PackageViolation::NextestSlowPeriodTooLong { .. }
            | PackageViolation::NextestSlowTimeoutUnset { .. }
            | PackageViolation::NextestTestNeverTerminated { .. }
            | PackageViolation::NextestTestTerminatedTooLate { .. }
            | PackageViolation::NextestTimeoutPasses { .. } => Self::NextestTimeoutNotEnforced,
            PackageViolation::PackageDependencyCycle { .. } => Self::PackageDependencyCycle,
            PackageViolation::PackageDependsOnItself { .. } => Self::PackageDependsOnItself,
            PackageViolation::PublishableTestCrate { .. } => Self::PublishableTestCrate,
            PackageViolation::TestCrateDeclaredTwice { .. } => Self::TestCrateDeclaredTwice,
            PackageViolation::TestCrateNotAMember { .. } => Self::TestCrateNotAMember,
            PackageViolation::TestCrateUsedByProduction { .. } => Self::TestCrateUsedByProduction,
            PackageViolation::WithoutLibrary(LibrarylessPackage {
                contents: LibrarylessContents::Benchmarks,
                ..
            }) => Self::BenchmarksOnlyPackageWithoutLibraries,
            PackageViolation::WithoutLibrary(LibrarylessPackage {
                contents: LibrarylessContents::Tests,
                ..
            }) => Self::TestsOnlyPackageWithoutTestLibraries,
            PackageViolation::RequiredLintNotDenied { .. } => Self::RequiredLintNotDenied,
            PackageViolation::RequiredClippySettingMissing { .. } => {
                Self::RequiredClippySettingMissing
            }
            PackageViolation::UnvettedClippySetting { .. } => Self::UnvettedClippySetting,
            PackageViolation::UnvettedNextestSetting { .. } => Self::UnvettedNextestSetting,
        }
    }
}

impl ValueEnum for IssueType {
    fn value_variants<'variants>() -> &'variants [Self] {
        &[
            Self::AdditionalPublicItem,
            Self::BenchmarkTargetExcludedFromBenchmarks,
            Self::BenchmarkTargetRunByTests,
            Self::BenchmarkTargetWithLibtestHarness,
            Self::BenchmarkTargetWithoutCriterionBenchmark,
            Self::BenchmarksOnlyPackageWithoutLibraries,
            Self::ClippyConfigurationAmbiguous,
            Self::ClippyConfigurationMissing,
            Self::CodeActiveInNoConfiguration,
            Self::CodeWithoutCoverageData,
            Self::ConfigurationSelectionMacro,
            Self::ConfinedPackageReachedOutsideItsUsers,
            Self::ConfinedPackageUserListedTwice,
            Self::ConfinedPackageUserNotAMember,
            Self::ConfinedPackageUserNotReachingIt,
            Self::DuplicatedFunctionBody,
            Self::Error(ErrorIssue::ErrorSourceOutsideChain),
            Self::Error(ErrorIssue::ErrorTypeNotAnEnum),
            Self::FailedMacroExpansion,
            Self::FunctionUsedOnlyByBenchmarks,
            Self::FunctionUsedOnlyByTests,
            Self::GlobImport,
            Self::Error(ErrorIssue::HandwrittenErrorType),
            Self::Test(TestIssue::Ignored),
            Self::ImplWithoutLocalType,
            Self::IncludedSourceFile,
            Self::InvalidMacroDefinition,
            Self::ItemOutOfOrder,
            Self::LintAllowedInManifest,
            Self::LintSuppression,
            Self::Error(ErrorIssue::MacroGeneratedErrorType),
            Self::MethodOutOfOrder,
            Self::MisplacedBlanketImpl,
            Self::MisplacedForeignTypeImpl,
            Self::MisplacedImpl,
            Self::MisplacedTraitObjectImpl,
            Self::ModuleCycle,
            Self::ModuleFileInsideBlock,
            Self::ModuleNotNamedAfterItsItem,
            Self::Error(ErrorIssue::MultipleErrorTypes),
            Self::MustUseBenchmarkOutput,
            Self::NegatedConfiguration,
            Self::NextestConfigurationMissing,
            Self::NextestOverride,
            Self::NextestTimeoutNotEnforced,
            Self::NonBenchmarkItemInBenchmarkTarget,
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
            Self::ReferencedTest,
            Self::RequiredClippySettingMissing,
            Self::RequiredLintNotDenied,
            Self::Test(TestIssue::SerializationRoundTrip),
            Self::SharedModuleFile,
            Self::SingleLetterName,
            Self::Test(TestIssue::Sleeping),
            Self::Error(ErrorIssue::SourceInErrorMessage),
            Self::SplitIntegrationTests,
            Self::TestCrateDeclaredTwice,
            Self::TestCrateNotAMember,
            Self::TestCrateUsedByProduction,
            Self::TestOnlyCodeInProduction,
            Self::TestsOnlyPackageWithoutTestLibraries,
            Self::TupleInSignature,
            Self::UncoveredCode,
            Self::UndestructuredParameter,
            Self::UndeterminedBenchmarkOutput,
            Self::UnimplementedBuiltinMacro,
            Self::UnknownConfinedPackage,
            Self::UnmappedItem,
            Self::UnneededNextestConfiguration,
            Self::UnparsableMacroExpansion,
            Self::UnprefixedTypeParameter,
            Self::UnreachableTestFile,
            Self::UnresolvedExternCrate,
            Self::UnresolvedMacroCall,
            Self::UnresolvedMethodCall,
            Self::UnresolvedModule,
            Self::UnresolvedPath,
            Self::UnsafeCode,
            Self::Error(ErrorIssue::UntypedResultError),
            Self::UnvettedClippySetting,
            Self::UnvettedCriterionMethod,
            Self::UnvettedNextestSetting,
        ]
    }

    fn to_possible_value(&self) -> Option<PossibleValue> {
        Some(PossibleValue::new(self.text().name))
    }
}
