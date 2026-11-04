use clap::Parser;

use heisenberg_command::heisenberg_command::HeisenbergCommand;

#[test]
fn unknown_issue_type_is_a_usage_error() {
    let error = HeisenbergCommand::try_parse_from(["heisenberg", "explain", "Unknown"])
        .expect_err("an unknown issue type must be rejected");

    assert_eq!(
        error.to_string(),
        "error: invalid value 'Unknown' for '<ISSUE_TYPE>'\n  \
        [possible values: AdditionalPublicItem, ClippyConfigurationAmbiguous, ClippyConfigurationMissing, ClippyLintNotDenied, ClippyTestAllowanceMissing, CodeActiveOnNoTarget, ComputedEnvironmentVariableName, ConfigurationSelectionMacro, DuplicatedFunctionBody, FailedMacroExpansion, FunctionUsedOnlyByTests, GlobImport, ImplWithoutLocalType, InvalidMacroDefinition, LintAllowedInManifest, LintSuppression, MisplacedBlanketImpl, MisplacedForeignTypeImpl, MisplacedImpl, MisplacedTraitObjectImpl, ModuleCycle, ModuleFileInsideBlock, ModuleNotNamedAfterItsItem, NegatedConfiguration, NonConstantInConstantsModule, NonDocComment, NonTestItemInTestTarget, PackageDependencyCycle, PackageDependsOnItself, ProceduralFunction, PublicAssociatedConstant, PublicConstantOutsideConstantsModule, PublishableTestCrate, Reexport, SharedModuleFile, TestCrateDeclaredTwice, TestCrateNotAMember, TestCrateUsedByProduction, TestOnlyCodeInProduction, TestsOnlyPackageWithoutTestLibraries, UndeclaredEnvironmentVariable, UnimplementedBuiltinMacro, UnmappedItem, UnparsableMacroExpansion, UnresolvedExternCrate, UnresolvedMacroCall, UnresolvedMethodCall, UnresolvedModule, UnresolvedPath]\n\
        \n\
        For more information, try '--help'.\n\
        "
    );
    assert_eq!(error.exit_code(), 2);
}
