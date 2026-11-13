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
        [possible values: AdditionalPublicItem, BenchmarkTargetExcludedFromBenchmarks, BenchmarkTargetRunByTests, BenchmarkTargetWithLibtestHarness, BenchmarkTargetWithoutCriterionBenchmark, BenchmarksOnlyPackageWithoutLibraries, ClippyConfigurationAmbiguous, ClippyConfigurationMissing, CodeActiveInNoConfiguration, CodeWithoutCoverageData, ConfigurationSelectionMacro, ConfinedPackageReachedOutsideItsUsers, ConfinedPackageUserListedTwice, ConfinedPackageUserNotAMember, ConfinedPackageUserNotReachingIt, DuplicatedFunctionBody, FailedMacroExpansion, FunctionUsedOnlyByBenchmarks, FunctionUsedOnlyByTests, GlobImport, ImplWithoutLocalType, IncludedSourceFile, InvalidMacroDefinition, ItemOutOfOrder, LintAllowedInManifest, LintSuppression, MethodOutOfOrder, MisplacedBlanketImpl, MisplacedForeignTypeImpl, MisplacedImpl, MisplacedTraitObjectImpl, ModuleCycle, ModuleFileInsideBlock, ModuleNotNamedAfterItsItem, MustUseBenchmarkOutput, NegatedConfiguration, NextestConfigurationMissing, NextestOverride, NextestTimeoutNotEnforced, NonBenchmarkItemInBenchmarkTarget, NonConstantInConstantsModule, NonDocComment, NonTestItemInTestTarget, PackageDependencyCycle, PackageDependsOnItself, ProceduralFunction, PublicAssociatedConstant, PublicConstantOutsideConstantsModule, PublishableTestCrate, Reexport, ReferencedTest, RequiredClippySettingMissing, RequiredLintNotDenied, SharedModuleFile, SingleLetterName, TestCrateDeclaredTwice, TestCrateNotAMember, TestCrateUsedByProduction, TestOnlyCodeInProduction, TestsOnlyPackageWithoutTestLibraries, TupleInSignature, UncoveredCode, UndestructuredParameter, UndeterminedBenchmarkOutput, UnimplementedBuiltinMacro, UnknownConfinedPackage, UnmappedItem, UnparsableMacroExpansion, UnprefixedTypeParameter, UnresolvedExternCrate, UnresolvedMacroCall, UnresolvedMethodCall, UnresolvedModule, UnresolvedPath, UnsafeCode, UnvettedClippySetting, UnvettedCriterionMethod, UnvettedNextestSetting]\n\
        \n  \
        tip: a similar value exists: 'UnknownConfinedPackage'\n\
        \n\
        For more information, try '--help'.\n\
        "
    );
    assert_eq!(error.exit_code(), 2);
}
