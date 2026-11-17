use std::collections::BTreeMap;
use std::str;

use clap::Parser;

use heisenberg_command::heisenberg_command::HeisenbergCommand;
use heisenberg_command::heisenberg_exit_status::HeisenbergExitStatus;
use heisenberg_command_tests::constants::CATALOGUE_SUBCOMMAND;
use heisenberg_command_tests::constants::PROGRAM_NAME;

const CATALOGUE: &str = "Heisenberg 0.1.0 rule catalogue\n\
    Every rule is mandatory and decidable: no configuration relaxes or exempts from a rule, and every rule is decided from the declared inputs alone.\n\
        \n\
        Section 1: Analysable code\n\
        \n\
        Rule 1.1 ConfigurationSelectionMacro: A cfg_select! call\n  \
        Scope: module file\n  \
        Applies to: all code\n  \
        Evaluated: in each analysed configuration\n\
        \n\
        Rule 1.2 FailedMacroExpansion: A macro call that fails to expand\n  \
        Scope: module file\n  \
        Applies to: all code\n  \
        Evaluated: in each analysed configuration\n\
        \n\
        Rule 1.3 IncludedSourceFile: An include! of a file that is not a build script output\n  \
        Scope: module file\n  \
        Applies to: all code\n  \
        Evaluated: in each analysed configuration\n\
        \n\
        Rule 1.4 InvalidMacroDefinition: A macro definition that is not valid\n  \
        Scope: module file\n  \
        Applies to: all code\n  \
        Evaluated: in each analysed configuration\n\
        \n\
        Rule 1.5 ModuleFileInsideBlock: A module file declared inside a block\n  \
        Scope: module file\n  \
        Applies to: all code\n  \
        Evaluated: in each analysed configuration\n\
        \n\
        Rule 1.6 SharedModuleFile: A module file included as several modules\n  \
        Scope: workspace\n  \
        Applies to: all code\n  \
        Evaluated: in each analysed configuration\n\
        \n\
        Rule 1.7 UnimplementedBuiltinMacro: A built-in macro that rust-analyzer cannot expand\n  \
        Scope: module file\n  \
        Applies to: all code\n  \
        Evaluated: in each analysed configuration\n\
        \n\
        Rule 1.8 UnmappedItem: An item that cannot be mapped to its definition\n  \
        Scope: module file\n  \
        Applies to: all code\n  \
        Evaluated: in each analysed configuration\n\
        \n\
        Rule 1.9 UnparsableMacroExpansion: A macro expansion that does not parse\n  \
        Scope: module file\n  \
        Applies to: all code\n  \
        Evaluated: in each analysed configuration\n\
        \n\
        Rule 1.10 UnreachableSourceFile: A source file that no crate target includes\n  \
        Scope: package\n  \
        Applies to: all code\n  \
        Evaluated: only when every declared configuration is analysed\n\
        \n\
        Rule 1.11 UnresolvedExternCrate: An extern crate that rust-analyzer cannot resolve\n  \
        Scope: module file\n  \
        Applies to: all code\n  \
        Evaluated: in each analysed configuration\n\
        \n\
        Rule 1.12 UnresolvedMacroCall: A macro call, attribute or derive whose macro rust-analyzer cannot resolve\n  \
        Scope: module file\n  \
        Applies to: all code\n  \
        Evaluated: in each analysed configuration\n\
        \n\
        Rule 1.13 UnresolvedMethodCall: A method call that rust-analyzer cannot resolve\n  \
        Scope: module file\n  \
        Applies to: all code\n  \
        Evaluated: in each analysed configuration\n\
        \n\
        Rule 1.14 UnresolvedModule: A module declaration whose file rust-analyzer cannot resolve\n  \
        Scope: module file\n  \
        Applies to: all code\n  \
        Evaluated: in each analysed configuration\n\
        \n\
        Rule 1.15 UnresolvedPath: A path that rust-analyzer cannot resolve\n  \
        Scope: module file\n  \
        Applies to: all code\n  \
        Evaluated: in each analysed configuration\n\
        \n\
        Rule 1.16 UnsafeCode: Unsafe code in a crate that integrates no foreign code\n  \
        Scope: module file\n  \
        Applies to: all code\n  \
        Evaluated: in each analysed configuration\n\
        \n\
        Rule 1.17 DocTestsEnabled: A target whose doc tests are enabled\n  \
        Scope: package\n  \
        Applies to: all code\n  \
        Evaluated: in each analysed configuration\n\
        \n\
        Section 2: Declared configurations\n\
        \n\
        Rule 2.1 AttributeAppliedInNoConfiguration: A cfg_attr that applies in no declared configuration\n  \
        Scope: module file\n  \
        Applies to: all code\n  \
        Evaluated: only when every declared configuration is analysed\n\
        \n\
        Rule 2.2 CodeActiveInNoConfiguration: Code that no declared configuration builds\n  \
        Scope: module file\n  \
        Applies to: all code\n  \
        Evaluated: only when every declared configuration is analysed\n\
        \n\
        Rule 2.3 NegatedConfiguration: A cfg predicate that negates a feature, test or debug_assertions\n  \
        Scope: module file\n  \
        Applies to: all code\n  \
        Evaluated: in each analysed configuration\n\
        \n\
        Rule 2.4 VacuousNegatedConfiguration: A cfg predicate that negates a configuration no declared configuration sets\n  \
        Scope: module file\n  \
        Applies to: all code\n  \
        Evaluated: only when every declared configuration is analysed\n\
        \n\
        Section 3: Lints and formatting\n\
        \n\
        Rule 3.1 ClippyConfigurationAmbiguous: A directory that holds both clippy configuration files\n  \
        Scope: workspace\n  \
        Applies to: all code\n  \
        Evaluated: in each analysed configuration\n\
        \n\
        Rule 3.2 ClippyConfigurationMissing: A workspace member without a clippy configuration\n  \
        Scope: workspace\n  \
        Applies to: all code\n  \
        Evaluated: in each analysed configuration\n\
        \n\
        Rule 3.3 LintAllowedInManifest: A manifest lints table that allows a lint\n  \
        Scope: package\n  \
        Applies to: all code\n  \
        Evaluated: in each analysed configuration\n\
        \n\
        Rule 3.4 LintSuppression: An attribute that suppresses or lowers lints over code written in the workspace\n  \
        Scope: module file\n  \
        Applies to: all code\n  \
        Evaluated: in each analysed configuration\n\
        \n\
        Rule 3.5 RequiredClippySettingMissing: A clippy configuration without a required setting\n  \
        Scope: workspace\n  \
        Applies to: all code\n  \
        Evaluated: in each analysed configuration\n\
        \n\
        Rule 3.6 RequiredLintNotDenied: A workspace member that leaves a required lint or lint group below deny\n  \
        Scope: package\n  \
        Applies to: all code\n  \
        Evaluated: in each analysed configuration\n\
        \n\
        Rule 3.7 RustVersionBelowSupported: A rust-version older than the supported Rust\n  \
        Scope: package\n  \
        Applies to: all code\n  \
        Evaluated: in each analysed configuration\n\
        \n\
        Rule 3.8 RustfmtConfigurationAmbiguous: A directory with both rustfmt configuration files\n  \
        Scope: workspace\n  \
        Applies to: all code\n  \
        Evaluated: in each analysed configuration\n\
        \n\
        Rule 3.9 RustfmtConfigurationInvalid: A rustfmt configuration that is not valid TOML\n  \
        Scope: workspace\n  \
        Applies to: all code\n  \
        Evaluated: in each analysed configuration\n\
        \n\
        Rule 3.10 RustfmtConfigurationMissing: A workspace without a rustfmt configuration\n  \
        Scope: workspace\n  \
        Applies to: all code\n  \
        Evaluated: in each analysed configuration\n\
        \n\
        Rule 3.11 RustfmtConfigurationUnreadable: A rustfmt configuration that cannot be read\n  \
        Scope: workspace\n  \
        Applies to: all code\n  \
        Evaluated: in each analysed configuration\n\
        \n\
        Rule 3.12 ToolAttribute: An attribute of the clippy or rustfmt tool\n  \
        Scope: module file\n  \
        Applies to: all code\n  \
        Evaluated: in each analysed configuration\n\
        \n\
        Rule 3.13 UnvettedClippySetting: A clippy configuration setting that Heisenberg has not vetted\n  \
        Scope: workspace\n  \
        Applies to: all code\n  \
        Evaluated: in each analysed configuration\n\
        \n\
        Rule 3.14 UnvettedRustfmtSetting: A rustfmt configuration setting that Heisenberg has not vetted\n  \
        Scope: workspace\n  \
        Applies to: all code\n  \
        Evaluated: in each analysed configuration\n\
        \n\
        Section 4: Packages and dependencies\n\
        \n\
        Rule 4.1 ConfinedPackageReachedOutsideItsUsers: A member that reaches a confined package without being one of its users\n  \
        Scope: workspace\n  \
        Applies to: all code\n  \
        Evaluated: in each analysed configuration\n\
        \n\
        Rule 4.2 ConfinedPackageUserListedTwice: A confined package user listed more than once\n  \
        Scope: workspace\n  \
        Applies to: all code\n  \
        Evaluated: in each analysed configuration\n\
        \n\
        Rule 4.3 ConfinedPackageUserNotAMember: A confined package user that is not a workspace member\n  \
        Scope: workspace\n  \
        Applies to: all code\n  \
        Evaluated: in each analysed configuration\n\
        \n\
        Rule 4.4 ConfinedPackageUserNotReachingIt: A confined package user that never reaches the package\n  \
        Scope: workspace\n  \
        Applies to: all code\n  \
        Evaluated: only when every declared configuration is analysed\n\
        \n\
        Rule 4.5 PackageDependencyCycle: Packages that depend on each other through dev-dependencies\n  \
        Scope: workspace\n  \
        Applies to: all code\n  \
        Evaluated: in each analysed configuration\n\
        \n\
        Rule 4.6 PackageDependsOnItself: A package that depends on itself through dev-dependencies\n  \
        Scope: package\n  \
        Applies to: all code\n  \
        Evaluated: in each analysed configuration\n\
        \n\
        Rule 4.7 UnknownConfinedPackage: A confined package that the workspace does not resolve\n  \
        Scope: workspace\n  \
        Applies to: all code\n  \
        Evaluated: in each analysed configuration\n\
        \n\
        Rule 4.8 UnpinnedDependencyRequirement: A dependency requirement that is not a MAJOR.MINOR version or a git rev\n  \
        Scope: package\n  \
        Applies to: all code\n  \
        Evaluated: in each analysed configuration\n\
        \n\
        Section 5: Modules and items\n\
        \n\
        Rule 5.1 AdditionalPublicItem: A module with more than one public item\n  \
        Scope: module file\n  \
        Applies to: production code, not to test code\n  \
        Evaluated: in each analysed configuration\n\
        \n\
        Rule 5.2 GlobImport: A glob import\n  \
        Scope: module file\n  \
        Applies to: all code\n  \
        Evaluated: in each analysed configuration\n\
        \n\
        Rule 5.3 ImplWithoutLocalType: An impl of another crate's trait with no home in this crate\n  \
        Scope: module file\n  \
        Applies to: production code, not to test code\n  \
        Evaluated: in each analysed configuration\n\
        \n\
        Rule 5.4 ImportInsideBlock: An import written inside a block\n  \
        Scope: module file\n  \
        Applies to: all code\n  \
        Evaluated: in each analysed configuration\n\
        \n\
        Rule 5.5 ItemOutOfOrder: An item that follows an item of a later group\n  \
        Scope: module file\n  \
        Applies to: all code\n  \
        Evaluated: in each analysed configuration\n\
        \n\
        Rule 5.6 MethodOutOfOrder: A function of an impl block or trait that follows one it belongs before\n  \
        Scope: module file\n  \
        Applies to: all code\n  \
        Evaluated: in each analysed configuration\n\
        \n\
        Rule 5.7 MisplacedBlanketImpl: A blanket impl outside the file of its trait\n  \
        Scope: module file\n  \
        Applies to: production code, not to test code\n  \
        Evaluated: in each analysed configuration\n\
        \n\
        Rule 5.8 MisplacedForeignTypeImpl: A local trait's impl for a foreign type outside the trait's file\n  \
        Scope: module file\n  \
        Applies to: production code, not to test code\n  \
        Evaluated: in each analysed configuration\n\
        \n\
        Rule 5.9 MisplacedImpl: An impl block outside the file of the type it belongs to\n  \
        Scope: module file\n  \
        Applies to: production code, not to test code\n  \
        Evaluated: in each analysed configuration\n\
        \n\
        Rule 5.10 MisplacedTraitObjectImpl: An impl for a trait object outside the file of its trait\n  \
        Scope: module file\n  \
        Applies to: production code, not to test code\n  \
        Evaluated: in each analysed configuration\n\
        \n\
        Rule 5.11 ModuleCycle: Modules that depend on each other\n  \
        Scope: crate target\n  \
        Applies to: all code\n  \
        Evaluated: in each analysed configuration\n\
        \n\
        Rule 5.12 ModuleNotNamedAfterItsItem: A module that is not named after its public item\n  \
        Scope: module file\n  \
        Applies to: production code, not to test code\n  \
        Evaluated: in each analysed configuration\n\
        \n\
        Rule 5.13 NonConstantInConstantsModule: A public item other than a constant in crate::constants\n  \
        Scope: module file\n  \
        Applies to: production code, not to test code\n  \
        Evaluated: in each analysed configuration\n\
        \n\
        Rule 5.14 PublicAssociatedConstant: A public associated constant of an inherent impl\n  \
        Scope: module file\n  \
        Applies to: production code, not to test code\n  \
        Evaluated: in each analysed configuration\n\
        \n\
        Rule 5.15 PublicConstantOutsideConstantsModule: A public constant outside crate::constants\n  \
        Scope: module file\n  \
        Applies to: production code, not to test code\n  \
        Evaluated: in each analysed configuration\n\
        \n\
        Rule 5.16 Reexport: A re-export\n  \
        Scope: module file\n  \
        Applies to: all code\n  \
        Evaluated: in each analysed configuration\n\
        \n\
        Rule 5.17 ItemMoreVisibleThanItsUses: An exported item that no other crate target uses\n  \
        Scope: workspace\n  \
        Applies to: production code, not to test code\n  \
        Evaluated: only when every declared configuration is analysed\n\
        \n\
        Section 6: Names, data and functions\n\
        \n\
        Rule 6.1 DuplicatedFunctionBody: Functions whose signatures and bodies are identical\n  \
        Scope: workspace\n  \
        Applies to: all code\n  \
        Evaluated: in each analysed configuration\n\
        \n\
        Rule 6.2 NonDocComment: A comment that is not a doc comment\n  \
        Scope: module file\n  \
        Applies to: all code\n  \
        Evaluated: in each analysed configuration\n\
        \n\
        Rule 6.3 PositionalFields: A struct or variant with two or more positional fields\n  \
        Scope: module file\n  \
        Applies to: all code\n  \
        Evaluated: in each analysed configuration\n\
        \n\
        Rule 6.4 ProceduralFunction: A function that takes a type of its crate without belonging to it\n  \
        Scope: module file\n  \
        Applies to: all code\n  \
        Evaluated: in each analysed configuration\n\
        \n\
        Rule 6.5 SingleLetterName: A parameter name of a single character\n  \
        Scope: module file\n  \
        Applies to: all code\n  \
        Evaluated: in each analysed configuration\n\
        \n\
        Rule 6.6 TupleInSignature: A tuple type in an item signature\n  \
        Scope: module file\n  \
        Applies to: all code\n  \
        Evaluated: in each analysed configuration\n\
        \n\
        Rule 6.7 UndestructuredParameter: A parameter used only through its fields\n  \
        Scope: module file\n  \
        Applies to: all code\n  \
        Evaluated: in each analysed configuration\n\
        \n\
        Rule 6.8 UnprefixedTypeParameter: A type parameter not named T followed by a capitalized word\n  \
        Scope: module file\n  \
        Applies to: all code\n  \
        Evaluated: in each analysed configuration\n\
        \n\
        Section 7: Error types\n\
        \n\
        Rule 7.1 ErrorMessageFunction: An error message formatted by a function\n  \
        Scope: module file\n  \
        Applies to: production code, not to test code\n  \
        Evaluated: in each analysed configuration\n\
        \n\
        Rule 7.2 ErrorMessageOnTheEnum: An error message written on the enum instead of its variants\n  \
        Scope: module file\n  \
        Applies to: production code, not to test code\n  \
        Evaluated: in each analysed configuration\n\
        \n\
        Rule 7.3 ErrorSourceOutsideChain: An error held outside the source chain\n  \
        Scope: module file\n  \
        Applies to: production code, not to test code\n  \
        Evaluated: in each analysed configuration\n\
        \n\
        Rule 7.4 ErrorTypeNotAnEnum: An error type deriving thiserror::Error that is not an enum\n  \
        Scope: module file\n  \
        Applies to: production code, not to test code\n  \
        Evaluated: in each analysed configuration\n\
        \n\
        Rule 7.5 ErrorTypeOutsideItsModule: An error type outside a module file named after it\n  \
        Scope: module file\n  \
        Applies to: production code, not to test code\n  \
        Evaluated: in each analysed configuration\n\
        \n\
        Rule 7.6 HandwrittenErrorDisplay: An error type that formats its message by hand\n  \
        Scope: module file\n  \
        Applies to: production code, not to test code\n  \
        Evaluated: in each analysed configuration\n\
        \n\
        Rule 7.7 HandwrittenErrorType: An error type implemented by hand\n  \
        Scope: module file\n  \
        Applies to: production code, not to test code\n  \
        Evaluated: in each analysed configuration\n\
        \n\
        Rule 7.8 MacroGeneratedErrorType: An error type implemented by a macro other than thiserror::Error\n  \
        Scope: module file\n  \
        Applies to: production code, not to test code\n  \
        Evaluated: in each analysed configuration\n\
        \n\
        Rule 7.9 MultipleErrorTypes: A crate with more than one error type\n  \
        Scope: crate target\n  \
        Applies to: production code, not to test code\n  \
        Evaluated: in each analysed configuration\n\
        \n\
        Rule 7.10 SourceInErrorMessage: An error message that formats its own source\n  \
        Scope: module file\n  \
        Applies to: production code, not to test code\n  \
        Evaluated: in each analysed configuration\n\
        \n\
        Rule 7.11 UntypedResultError: A Result whose error type erases the failure\n  \
        Scope: module file\n  \
        Applies to: production code, not to test code\n  \
        Evaluated: in each analysed configuration\n\
        \n\
        Rule 7.12 ResultErrorNotAnError: A Result whose error type is a workspace type that is not an error\n  \
        Scope: module file\n  \
        Applies to: production code, not to test code\n  \
        Evaluated: in each analysed configuration\n\
        \n\
        Section 8: Tests\n\
        \n\
        Rule 8.1 IgnoredTest: A test that the test run skips\n  \
        Scope: module file\n  \
        Applies to: all code\n  \
        Evaluated: in each analysed configuration\n\
        \n\
        Rule 8.2 ItemUsedOnlyByTests: A production item that only tests use\n  \
        Scope: workspace\n  \
        Applies to: all code\n  \
        Evaluated: in each analysed configuration\n\
        \n\
        Rule 8.3 NextestConfigurationMissing: A workspace without a nextest configuration\n  \
        Scope: workspace\n  \
        Applies to: all code\n  \
        Evaluated: only when every declared configuration is analysed\n\
        \n\
        Rule 8.4 NextestOverride: A per-test override in the nextest configuration\n  \
        Scope: workspace\n  \
        Applies to: all code\n  \
        Evaluated: in each analysed configuration\n\
        \n\
        Rule 8.5 NextestTimeoutNotEnforced: A nextest profile whose slow-timeout exceeds the limits\n  \
        Scope: workspace\n  \
        Applies to: all code\n  \
        Evaluated: in each analysed configuration\n\
        \n\
        Rule 8.6 NonTestItemInTestTarget: An integration test item that is neither a test nor a file-local helper\n  \
        Scope: module file\n  \
        Applies to: all code\n  \
        Evaluated: in each analysed configuration\n\
        \n\
        Rule 8.7 PublishableTestCrate: A declared test crate that can be published\n  \
        Scope: package\n  \
        Applies to: all code\n  \
        Evaluated: in each analysed configuration\n\
        \n\
        Rule 8.8 ReferencedTest: A test that other code references\n  \
        Scope: module file\n  \
        Applies to: all code\n  \
        Evaluated: in each analysed configuration\n\
        \n\
        Rule 8.9 SplitIntegrationTests: Integration tests spread over several targets or discovered automatically\n  \
        Scope: package\n  \
        Applies to: all code\n  \
        Evaluated: in each analysed configuration\n\
        \n\
        Rule 8.10 TestCrateDeclaredTwice: A test crate listed more than once\n  \
        Scope: workspace\n  \
        Applies to: all code\n  \
        Evaluated: in each analysed configuration\n\
        \n\
        Rule 8.11 TestCrateNotAMember: A declared test crate that is not a workspace member\n  \
        Scope: workspace\n  \
        Applies to: all code\n  \
        Evaluated: in each analysed configuration\n\
        \n\
        Rule 8.12 TestCrateUsedByProduction: A declared test crate that production code depends on\n  \
        Scope: workspace\n  \
        Applies to: all code\n  \
        Evaluated: in each analysed configuration\n\
        \n\
        Rule 8.13 TestModuleInItsOwnFile: A test module declared in a file of its own\n  \
        Scope: module file\n  \
        Applies to: all code\n  \
        Evaluated: in each analysed configuration\n\
        \n\
        Rule 8.14 TestModuleNotNamedTests: A test module that is not named tests\n  \
        Scope: module file\n  \
        Applies to: all code\n  \
        Evaluated: in each analysed configuration\n\
        \n\
        Rule 8.15 TestOnlyCodeInProduction: Code that depends on cfg(test) outside a test module\n  \
        Scope: module file\n  \
        Applies to: all code\n  \
        Evaluated: in each analysed configuration\n\
        \n\
        Rule 8.16 TestOutsideTestModule: A test written outside a test module\n  \
        Scope: module file\n  \
        Applies to: all code\n  \
        Evaluated: in each analysed configuration\n\
        \n\
        Rule 8.17 TestsOnlyPackageWithoutTestLibraries: A package of tests that holds no test library of its own\n  \
        Scope: package\n  \
        Applies to: all code\n  \
        Evaluated: in each analysed configuration, and in full only when every declared configuration is analysed\n\
        \n\
        Rule 8.18 UnneededNextestConfiguration: A nextest configuration in a workspace without libtest tests\n  \
        Scope: workspace\n  \
        Applies to: all code\n  \
        Evaluated: only when every declared configuration is analysed\n\
        \n\
        Rule 8.19 UnvettedNextestSetting: A nextest configuration setting that Heisenberg has not vetted\n  \
        Scope: workspace\n  \
        Applies to: all code\n  \
        Evaluated: in each analysed configuration\n\
        \n\
        Section 9: Benchmarks\n\
        \n\
        Rule 9.1 BenchmarkTargetExcludedFromBenchmarks: A benchmark target that cargo bench never runs\n  \
        Scope: package\n  \
        Applies to: all code\n  \
        Evaluated: in each analysed configuration\n\
        \n\
        Rule 9.2 BenchmarkTargetRunByTests: A benchmark target that the test run runs\n  \
        Scope: package\n  \
        Applies to: all code\n  \
        Evaluated: in each analysed configuration\n\
        \n\
        Rule 9.3 BenchmarkTargetWithLibtestHarness: A benchmark target built with libtest's harness\n  \
        Scope: package\n  \
        Applies to: all code\n  \
        Evaluated: in each analysed configuration\n\
        \n\
        Rule 9.4 BenchmarkTargetWithoutCriterionBenchmark: A benchmark target that registers no criterion benchmark\n  \
        Scope: crate target\n  \
        Applies to: all code\n  \
        Evaluated: in each analysed configuration\n\
        \n\
        Rule 9.5 BenchmarksOnlyPackageWithoutLibraries: A package of benchmarks that holds no library of its own\n  \
        Scope: package\n  \
        Applies to: all code\n  \
        Evaluated: in each analysed configuration, and in full only when every declared configuration is analysed\n\
        \n\
        Rule 9.6 ItemUsedOnlyByBenchmarks: A production item that only benchmarks use\n  \
        Scope: workspace\n  \
        Applies to: all code\n  \
        Evaluated: in each analysed configuration\n\
        \n\
        Rule 9.7 MustUseBenchmarkOutput: A measured routine whose must-use result criterion drops\n  \
        Scope: module file\n  \
        Applies to: all code\n  \
        Evaluated: in each analysed configuration\n\
        \n\
        Rule 9.8 NonBenchmarkItemInBenchmarkTarget: A benchmark target item that is neither its entry point nor a file-local helper\n  \
        Scope: module file\n  \
        Applies to: all code\n  \
        Evaluated: in each analysed configuration\n\
        \n\
        Rule 9.9 UndeterminedBenchmarkOutput: A measured routine whose return type the analysis cannot determine\n  \
        Scope: module file\n  \
        Applies to: all code\n  \
        Evaluated: in each analysed configuration\n\
        \n\
        Rule 9.10 UnvettedCriterionMethod: A criterion method that Heisenberg has not vetted\n  \
        Scope: module file\n  \
        Applies to: all code\n  \
        Evaluated: in each analysed configuration\n\
        \n\
        Section 10: Coverage\n\
        \n\
        Rule 10.1 CodeWithoutCoverageData: A production function without coverage data\n  \
        Scope: module file\n  \
        Applies to: all code\n  \
        Evaluated: in each analysed configuration\n\
        \n\
        Rule 10.2 UncoveredCode: Production code that no test executes\n  \
        Scope: module file\n  \
        Applies to: all code\n  \
        Evaluated: in each analysed configuration\n\
        ";

#[test]
fn catalogue_lists_every_rule_with_its_scope_and_evaluation() {
    let mut output = Vec::new();
    let mut errors = Vec::new();
    let status = HeisenbergCommand::try_parse_from([PROGRAM_NAME, CATALOGUE_SUBCOMMAND])
        .expect("the arguments must parse")
        .run(&BTreeMap::new(), &mut output, &mut errors)
        .expect("the errors of the run must be reported");

    assert_eq!(str::from_utf8(&output), Ok(CATALOGUE));
    assert_eq!(errors, b"");
    assert_eq!(status, HeisenbergExitStatus::Explained);
}
