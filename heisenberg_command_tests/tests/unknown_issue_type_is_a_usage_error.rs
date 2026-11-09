use clap::Parser;

use heisenberg_command::heisenberg_command::HeisenbergCommand;

#[test]
fn unknown_issue_type_is_a_usage_error() {
    let error = HeisenbergCommand::try_parse_from(["heisenberg", "explain", "Unknown"])
        .expect_err("an unknown issue type must be rejected");

    assert_eq!(
        error.to_string(),
        "error: invalid value 'Unknown' for '<ISSUE_TYPE>'\n  \
        [possible values: AdditionalPublicItem, ClippyConfigurationAmbiguous, ClippyConfigurationMissing, CodeActiveInNoConfiguration, ConfigurationSelectionMacro, DuplicatedFunctionBody, FailedMacroExpansion, FunctionUsedOnlyByTests, GlobImport, ImplWithoutLocalType, IncludedSourceFile, InvalidMacroDefinition, ItemOutOfOrder, LintAllowedInManifest, LintSuppression, MethodOutOfOrder, MisplacedBlanketImpl, MisplacedForeignTypeImpl, MisplacedImpl, MisplacedTraitObjectImpl, ModuleCycle, ModuleFileInsideBlock, ModuleNotNamedAfterItsItem, NegatedConfiguration, NextestConfigurationMissing, NextestOverride, NextestTimeoutNotEnforced, NonConstantInConstantsModule, NonDocComment, NonTestItemInTestTarget, PackageDependencyCycle, PackageDependsOnItself, ProceduralFunction, PublicAssociatedConstant, PublicConstantOutsideConstantsModule, PublishableTestCrate, Reexport, ReferencedTest, RequiredClippySettingMissing, RequiredLintNotDenied, SharedModuleFile, SingleLetterName, TestCrateDeclaredTwice, TestCrateNotAMember, TestCrateUsedByProduction, TestOnlyCodeInProduction, TestsOnlyPackageWithoutTestLibraries, TupleInSignature, UndestructuredParameter, UnimplementedBuiltinMacro, UnmappedItem, UnparsableMacroExpansion, UnprefixedTypeParameter, UnresolvedExternCrate, UnresolvedMacroCall, UnresolvedMethodCall, UnresolvedModule, UnresolvedPath, UnsafeCode, UnvettedClippySetting, UnvettedNextestSetting]\n\
        \n\
        For more information, try '--help'.\n\
        "
    );
    assert_eq!(error.exit_code(), 2);
}
