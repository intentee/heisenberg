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
        [possible values: AdditionalPublicItem, BenchmarkTargetExcludedFromBenchmarks, BenchmarkTargetRunByTests, BenchmarkTargetWithLibtestHarness, BenchmarkTargetWithoutCriterionBenchmark, BenchmarksOnlyPackageWithoutLibraries, ClippyConfigurationAmbiguous, ClippyConfigurationMissing, CodeActiveInNoConfiguration, CodeWithoutCoverageData, ConfigurationSelectionMacro, ConfinedPackageReachedOutsideItsUsers, ConfinedPackageUserListedTwice, ConfinedPackageUserNotAMember, ConfinedPackageUserNotReachingIt, DuplicatedFunctionBody, ErrorSourceOutsideChain, ErrorTypeNotAnEnum, FailedMacroExpansion, FunctionUsedOnlyByBenchmarks, FunctionUsedOnlyByTests, GlobImport, HandwrittenErrorType, IgnoredTest, ImplWithoutLocalType, IncludedSourceFile, InvalidMacroDefinition, ItemOutOfOrder, LintAllowedInManifest, LintSuppression, MacroGeneratedErrorType, MethodOutOfOrder, MisplacedBlanketImpl, MisplacedForeignTypeImpl, MisplacedImpl, MisplacedTraitObjectImpl, ModuleCycle, ModuleFileInsideBlock, ModuleNotNamedAfterItsItem, MultipleErrorTypes, MustUseBenchmarkOutput, NegatedConfiguration, NextestConfigurationMissing, NextestOverride, NextestTimeoutNotEnforced, NonBenchmarkItemInBenchmarkTarget, NonConstantInConstantsModule, NonDocComment, NonTestItemInTestTarget, PackageDependencyCycle, PackageDependsOnItself, ProceduralFunction, PublicAssociatedConstant, PublicConstantOutsideConstantsModule, PublishableTestCrate, Reexport, ReferencedTest, RequiredClippySettingMissing, RequiredLintNotDenied, SerializationRoundTripTest, SharedModuleFile, SingleLetterName, SleepInTest, SourceInErrorMessage, SplitIntegrationTests, TestCrateDeclaredTwice, TestCrateNotAMember, TestCrateUsedByProduction, TestOnlyCodeInProduction, TestsOnlyPackageWithoutTestLibraries, TupleInSignature, UncoveredCode, UndestructuredParameter, UndeterminedBenchmarkOutput, UnimplementedBuiltinMacro, UnknownConfinedPackage, UnmappedItem, UnneededNextestConfiguration, UnparsableMacroExpansion, UnprefixedTypeParameter, UnreachableTestFile, UnresolvedExternCrate, UnresolvedMacroCall, UnresolvedMethodCall, UnresolvedModule, UnresolvedPath, UnsafeCode, UntypedResultError, UnvettedClippySetting, UnvettedCriterionMethod, UnvettedNextestSetting]\n\
        \n  \
        tip: a similar value exists: 'UnknownConfinedPackage'\n\
        \n\
        For more information, try '--help'.\n\
        "
    );
    assert_eq!(error.exit_code(), 2);
}
