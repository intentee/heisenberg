use clap::Parser;

use heisenberg_command::heisenberg_command::HeisenbergCommand;
use heisenberg_command_tests::constants::EXPLAIN_SUBCOMMAND;
use heisenberg_command_tests::constants::PROGRAM_NAME;

#[test]
fn unknown_issue_type_is_a_usage_error() {
    let error = HeisenbergCommand::try_parse_from([PROGRAM_NAME, EXPLAIN_SUBCOMMAND, "Unknown"])
        .expect_err("an unknown issue type must be rejected");

    assert_eq!(
        error.to_string(),
        "error: invalid value 'Unknown' for '<ISSUE_TYPE>'\n  \
        [possible values: AdditionalPublicItem, AttributeAppliedInNoConfiguration, BenchmarkTargetExcludedFromBenchmarks, BenchmarkTargetRunByTests, BenchmarkTargetWithLibtestHarness, BenchmarkTargetWithoutCriterionBenchmark, BenchmarksOnlyPackageWithoutLibraries, ClippyConfigurationAmbiguous, ClippyConfigurationMissing, CodeActiveInNoConfiguration, CodeWithoutCoverageData, ConfigurationSelectionMacro, ConfinedPackageReachedOutsideItsUsers, ConfinedPackageUserListedTwice, ConfinedPackageUserNotAMember, ConfinedPackageUserNotReachingIt, DocTestsEnabled, DuplicatedFunctionBody, ErrorMessageFunction, ErrorMessageOnTheEnum, ErrorSourceOutsideChain, ErrorTypeNotAnEnum, ErrorTypeOutsideItsModule, FailedMacroExpansion, GlobImport, HandwrittenErrorDisplay, HandwrittenErrorType, IgnoredTest, ImplWithoutLocalType, ImportInsideBlock, IncludedSourceFile, InvalidMacroDefinition, ItemMoreVisibleThanItsUses, ItemOutOfOrder, ItemUsedOnlyByBenchmarks, ItemUsedOnlyByTests, LintAllowedInManifest, LintSuppression, MacroGeneratedErrorType, MethodOutOfOrder, MisplacedBlanketImpl, MisplacedForeignTypeImpl, MisplacedImpl, MisplacedTraitObjectImpl, ModuleCycle, ModuleFileInsideBlock, ModuleNotNamedAfterItsItem, MultipleErrorTypes, MustUseBenchmarkOutput, NegatedConfiguration, NextestConfigurationMissing, NextestOverride, NextestTimeoutNotEnforced, NonBenchmarkItemInBenchmarkTarget, NonConstantInConstantsModule, NonDocComment, NonTestItemInTestTarget, PackageDependencyCycle, PackageDependsOnItself, PositionalFields, ProceduralFunction, PublicAssociatedConstant, PublicConstantOutsideConstantsModule, PublishableTestCrate, Reexport, ReferencedTest, RequiredClippySettingMissing, RequiredLintNotDenied, ResultErrorNotAnError, RustVersionBelowSupported, RustfmtConfigurationAmbiguous, RustfmtConfigurationInvalid, RustfmtConfigurationMissing, RustfmtConfigurationUnreadable, SharedModuleFile, SingleLetterName, SourceInErrorMessage, SplitIntegrationTests, TestCrateDeclaredTwice, TestCrateNotAMember, TestCrateUsedByProduction, TestModuleInItsOwnFile, TestModuleNotNamedTests, TestOnlyCodeInProduction, TestOutsideTestModule, TestsOnlyPackageWithoutTestLibraries, ToolAttribute, TupleInSignature, UncoveredCode, UndestructuredParameter, UndeterminedBenchmarkOutput, UnimplementedBuiltinMacro, UnknownConfinedPackage, UnmappedItem, UnneededNextestConfiguration, UnparsableMacroExpansion, UnpinnedDependencyRequirement, UnprefixedTypeParameter, UnreachableSourceFile, UnresolvedExternCrate, UnresolvedMacroCall, UnresolvedMethodCall, UnresolvedModule, UnresolvedPath, UnsafeCode, UntypedResultError, UnvettedClippySetting, UnvettedCriterionMethod, UnvettedNextestSetting, UnvettedRustfmtSetting, VacuousNegatedConfiguration]\n\
        \n  \
        tip: a similar value exists: 'UnknownConfinedPackage'\n\
        \n\
        For more information, try '--help'.\n\
        "
    );
    assert_eq!(error.exit_code(), 2);
}
