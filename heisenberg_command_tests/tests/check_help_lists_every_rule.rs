use clap::Parser;
use clap::error::ErrorKind;

use heisenberg_command::heisenberg_command::HeisenbergCommand;
use heisenberg_command_tests::constants::CHECK_SUBCOMMAND;
use heisenberg_command_tests::constants::HELP_ARGUMENT;
use heisenberg_command_tests::constants::PROGRAM_NAME;

const CHECK_HELP: &str = "Report module dependency cycles and structural violations in every crate of a Cargo workspace, in\n\
    the configurations it declares.\n\
    \n\
    Settings: the workspace root manifest holds every Heisenberg setting in its\n\
    [workspace.metadata.heisenberg] table, whose required configurations array lists the configurations\n\
    the workspace supports, each a table of a unique, nonempty name, a target among\n\
    aarch64-apple-darwin, x86_64-pc-windows-msvc, x86_64-unknown-linux-gnu, default_features, which\n\
    keeps or leaves out the default features, and features, the names of features that workspace members\n\
    define, each listed once, no two configurations sharing their target and features, and whose\n\
    optional out_dirs table declares, for a package of the workspace, member or dependency, whose build\n\
    script cannot run on every host, the version of that package that the workspace resolves and a\n\
    configurations table that gives, for each declared configuration, a directory, relative to the\n\
    workspace root, with the files the build script writes to OUT_DIR, and the cfgs and env the build\n\
    script emits; that build script then never runs, and the declared files, cfgs and env stand in for\n\
    its output; its optional test_crates list names the workspace members that hold only code for tests;\n\
    and its optional confined_packages table maps the name of a package, which names every resolved\n\
    version of it, to the workspace members that may depend on it, directly or transitively. A missing\n\
    table, an unknown key, an empty configuration list, an empty or repeated configuration name, a\n\
    configuration with an unknown target, with a feature that no workspace member defines or with a\n\
    feature listed twice, two configurations with the same target and features, a configuration that\n\
    selects features other than the default ones in a workspace that uses resolver 1 with a root package\n\
    and other members, where cargo applies a feature selection to the root package only, an out-dir for\n\
    a package the workspace does not resolve, for a version of it other than the resolved ones, for a\n\
    name and version that packages from several sources share or for a package without a build script,\n\
    an out-dir directory outside the workspace or that cannot be read, an out-dir without the output of\n\
    a declared configuration or with the output of an undeclared configuration, and a declared cfg that\n\
    does not parse fail the run. A test_crates entry that is not a workspace member, that is listed\n\
    twice, whose package can be published, or that a member other than a test crate depends on through a\n\
    normal or build dependency is a violation, and an entry that fails these conditions is not a test\n\
    crate. A confined_packages entry for a package that the workspace does not resolve, a user that is\n\
    listed twice for one package or that is not a workspace member, a member other than the users of a\n\
    package for which cargo builds the package in an analysed configuration, through the normal and\n\
    build dependencies of any package and the member's own dev-dependencies, on the target or the host\n\
    where cargo builds it, a member never reaching its own package, and a user for which cargo builds\n\
    the package in no declared configuration are violations. No setting exempts code from any rule:\n\
    test_crates declares which members are test code, confined_packages declares which members may\n\
    depend on a package, and Heisenberg verifies both declarations.\n\
    \n\
    Precondition: rustc 1.99.1 compiles the workspace in every analysed configuration with the standard\n\
    library of its target installed, Cargo.lock is up to date, the cargo home holds every package of\n\
    Cargo.lock (`cargo fetch`), the build runs from the workspace directory with the environment\n\
    Heisenberg runs with and without --config arguments, clippy runs without command-line arguments,\n\
    nextest runs without the command-line arguments that replace or extend its configuration, retry\n\
    tests or disable timeouts (--config-file, --tool-config-file, --retries, --debugger and --tracer),\n\
    the tests run without the target selections that build and run benchmarks (--benches, --bench and\n\
    --all-targets), and benchmarks run without criterion's command-line arguments that change how it\n\
    measures or judges them (--quick, --sample-size, --warm-up-time, --measurement-time, --nresamples,\n\
    --noise-threshold, --confidence-level, --significance-level and --profile-time). An environment\n\
    variable whose name or value is not UTF-8, CARGO_ENCODED_RUSTFLAGS, RUSTFLAGS, RUSTC,\n\
    RUSTC_WORKSPACE_WRAPPER, RUSTC_WRAPPER, RUSTDOC, CLIPPY_ARGS, CLIPPY_CONF_DIR, NEXTEST_RETRIES or\n\
    RUSTC_BOOTSTRAP, __CARGO_TEST_CHANNEL_OVERRIDE_DO_NOT_USE_THIS in the environment, a workspace\n\
    directory that is not the root of its cargo workspace, a workspace member or crate root outside the\n\
    workspace root, a package whose metadata cargo_metadata cannot read, a crate root that two targets\n\
    share or that cannot be read, a metabuild script, which has no source file, a workspace root, target\n\
    directory or crate root that is not an absolute UTF-8 path, a clippy.toml or .clippy.toml that\n\
    discovery finds and cannot read, or that is the only one in its directory and is not valid TOML, a\n\
    .config/nextest.toml that cannot be read or is not a valid nextest configuration, rustflags in the\n\
    [build] or [target] tables of the cargo configuration or in the CARGO_BUILD_RUSTFLAGS and\n\
    CARGO_TARGET_<TRIPLE>_RUSTFLAGS variables of a supported target, rustc, rustc-workspace-wrapper,\n\
    rustc-wrapper or rustdoc in the [build] table of the cargo configuration, including through the\n\
    environment variables that set them, a [patch] table in the cargo configuration, whose patches\n\
    belong in the workspace manifest, a build script override of a supported target in the cargo\n\
    configuration ([target.<triple>.<links>]), and CLIPPY_ARGS, CLIPPY_CONF_DIR or RUSTC_BOOTSTRAP in\n\
    the [env] table of the cargo configuration, under any letter case, fail the run. A failing run\n\
    reports the first failure among loading the project, building it, resolving the standard library of\n\
    its targets and reading its clippy and nextest configurations, in that order, and otherwise the\n\
    analysis failures of every configuration.\n\
    \n\
    Analysis: Values that the [env] table of the cargo configuration declares reach the analysis as\n\
    cargo resolves them, OUT_DIR and the cfgs and env a build script emits come from running it, or from\n\
    its out_dirs declaration, and an env! or option_env! of a variable that neither they nor cargo set\n\
    for the target is a violation. The values of the variables that cargo sets only while it builds a\n\
    target (CARGO_BIN_NAME, CARGO_BIN_EXE_<name>, CARGO_TARGET_TMPDIR and CARGO_PRIMARY_PACKAGE) are not\n\
    known to the analysis, and the files that include_str! and include_bytes! name are not read. The\n\
    analysis enables debug assertions, the features of the analysed configuration, each on every\n\
    workspace member that defines it, as cargo does, and test only for the targets cargo builds with\n\
    libtest's harness, as rustc's --test does: a target with harness = false, or one that cargo neither\n\
    tests nor benchmarks, such as a build script or an example, is analysed without it. Every run\n\
    analyses the declared configurations that --configuration selects, which it requires, and a\n\
    --configuration the workspace does not declare or selects twice fails the run. Build scripts and\n\
    procedural macros run: for each analysed configuration, cargo builds the procedural macros offline\n\
    with the rustc of the sysroot and --jobs parallel jobs, which build scripts see in NUM_JOBS, and\n\
    runs the build scripts, apart from those out_dirs declares, and --jobs processes of the sysroot's\n\
    rust-analyzer-proc-macro-srv expand the procedural macros, whose output is analysed like any macro\n\
    expansion, its input where it is written and the tokens it generates trusted, and a build script\n\
    that fails or emits a cfg that does not parse, one that needs a build script out_dirs declares, a\n\
    build script output that cannot be read, a procedural macro that fails to build or that the server\n\
    cannot load, a declared target without its standard library, a sysroot without rustc and a sysroot\n\
    without the server, or whose server cannot start, fail the run. Every decision is semantic, as\n\
    rust-analyzer resolves the code. The analysis runs on --jobs worker threads and reports the same\n\
    issues for every count. Shared module files are analysed in every crate that includes them. The\n\
    public item, constant, method order, integration test target and benchmark target rules apply to\n\
    items written in module files, including those that a macro_rules! macro of the crate writes, the\n\
    item order rule to the items written in module files, where a macro call takes one place whatever it\n\
    writes, and the name, tuple and parameter rules to the text written in them; an item that an\n\
    attribute macro rewrites is analysed through the items of its expansion that rust-analyzer maps back\n\
    to it, and the rest of the expansion, like the items that procedural macros and the macros of other\n\
    crates generate, is trusted. Derive macro output is analysed like any macro expansion, except that\n\
    of the built-in derives that rust-analyzer records as impls of their trait without expanding them,\n\
    which is trusted; text written outside a crate's module files, such as in other crates or in the\n\
    build script output that include! brings in, is trusted by the rules about how code is written, and\n\
    results reflect the files on disk, including generated ones. Trust ends where the analysis would be\n\
    incomplete: a name, method or macro that rust-analyzer cannot resolve and an expansion that fails\n\
    are violations in all the code of a crate target, whoever wrote the macro that generates it, and\n\
    unsafe code is a violation in everything the workspace wrote, wherever it expands, in what its\n\
    procedural macros generate and in the build script output that include! brings in. Heisenberg does\n\
    not verify what build scripts and procedural macros do when they run, the code of registry and git\n\
    dependencies, the linker and the native libraries, or that the workspace compiles and passes clippy:\n\
    it checks that the required lints are denied, not that the code meets them, and clippy decides by\n\
    itself which code its lints skip, such as code it attributes to a macro of another crate or to a\n\
    procedural macro. A partial run evaluates the rules its footer lists and exits with 0 when they find\n\
    no issue. Test code is every integration test target, every target of a test crate other than a\n\
    benchmark target and every module that cfg(test) gates, inline or in its own file and under any\n\
    name, and benchmark code is every benchmark target, measured through criterion, the criterion\n\
    package from crates.io, whose calls the criterion rules check wherever workspace code makes them; a\n\
    test crate is a workspace member that test_crates lists, whose package cannot be published (publish\n\
    = false), and that no member other than a test crate depends on through a normal or build\n\
    dependency. The error type rules count every implementation of std::error::Error in the production\n\
    code of a crate target, those that macros generate included, and accept only those that the Error\n\
    derive of thiserror, the thiserror-impl package from crates.io, generates for an enum. The error\n\
    type rules and the rules about where items live apply to production code and not to test code, where\n\
    nobody matches on an error or finds an item by its file; `heisenberg explain` and `heisenberg\n\
    catalogue` state for each rule whether it applies to all code or to production code. A standard\n\
    library without the items that the analysis relies on fails the run.\n\
    \n\
    Report: Issues are grouped by type, in one section per type: a `<IssueType>: <count> violation(s)`\n\
    line, a line that states the purpose of the rule and what to do, an empty line, and one `-\n\
    <file>[:<line>:<column>]: <message>` line per issue, followed, four spaces in, by its source line\n\
    when it has one and by further indented lines with its details, such as the dependencies of a cycle,\n\
    the inclusions of a shared module file, the failing expansions of a macro call or the impl blocks of\n\
    a flagged type. Each issue is reported once, at the name of the item it concerns; when it holds only\n\
    in some of the crate targets or configurations that analyse its file, the message names them. A\n\
    cause and its direct consequences form one issue. A partial run, one that leaves out a declared\n\
    configuration, names the configurations it left out after the analysed ones and does not evaluate\n\
    AttributeAppliedInNoConfiguration, CodeActiveInNoConfiguration, ConfinedPackageUserNotReachingIt,\n\
    ItemMoreVisibleThanItsUses, NextestConfigurationMissing, TestDependencyInProduction,\n\
    TestLibraryInProductionPackage, UnneededNextestConfiguration, UnreachableSourceFile,\n\
    UnusedDependency, VacuousNegatedConfiguration, which hold only over every declared configuration,\n\
    although it does evaluate NextestConfigurationMissing and UnneededNextestConfiguration when an\n\
    analysed configuration runs a test under libtest's harness, evaluates only in part\n\
    BenchmarksOnlyPackageWithoutLibraries, TestsOnlyPackageWithoutTestLibraries, whose empty-library\n\
    checks hold only over every declared configuration, and names the rules it does not evaluate and\n\
    those it evaluates in part; a partial run without issues exits as compliant.\n\
    \n\
    Rules: every rule is mandatory and decidable, and is listed under its section with its identifier.\n  \
    1 Analysable code\n    \
    H001 ConfigurationSelectionMacro: A cfg_select! call\n    \
    H002 FailedMacroExpansion: A macro call that fails to expand\n    \
    H003 IncludedSourceFile: An include! of a file that is not a build script output\n    \
    H004 InvalidMacroDefinition: A macro definition that is not valid\n    \
    H005 ModuleFileInsideBlock: A module file declared inside a block\n    \
    H006 SharedModuleFile: A module file included as several modules\n    \
    H007 UnimplementedBuiltinMacro: A built-in macro that rust-analyzer cannot expand\n    \
    H008 UnmappedItem: An item that cannot be mapped to its definition\n    \
    H009 UnparsableMacroExpansion: A macro expansion that does not parse\n    \
    H010 UnreachableSourceFile: A source file that no crate target includes\n    \
    H011 UnresolvedExternCrate: An extern crate that rust-analyzer cannot resolve\n    \
    H012 UnresolvedMacroCall: A macro call, attribute or derive whose macro rust-analyzer cannot\n    \
    resolve\n    \
    H013 UnresolvedMethodCall: A method call that rust-analyzer cannot resolve\n    \
    H014 UnresolvedModule: A module declaration whose file rust-analyzer cannot resolve\n    \
    H015 UnresolvedPath: A path that rust-analyzer cannot resolve\n    \
    H016 UnsafeCode: Unsafe code in a crate that integrates no foreign code\n    \
    H017 DocTestsEnabled: A target whose doc tests are enabled\n    \
    H018 UndeclaredEnvironmentVariable: An env! or option_env! of a variable that nothing declares\n    \
    H118 BuildScriptEnvironmentVariable: A variable that a member's build script sets for the\n    \
    compiler\n  \
    2 Declared configurations\n    \
    H019 AttributeAppliedInNoConfiguration: A cfg_attr that applies in no declared configuration\n    \
    H020 CodeActiveInNoConfiguration: Code that no declared configuration builds\n    \
    H021 NegatedConfiguration: A cfg predicate that negates a feature, test or debug_assertions\n    \
    H022 VacuousNegatedConfiguration: A cfg predicate that negates a configuration no declared\n    \
    configuration sets\n  \
    3 Lints and formatting\n    \
    H023 ClippyConfigurationAmbiguous: A directory that holds both clippy configuration files\n    \
    H024 ClippyConfigurationMissing: A workspace member without a clippy configuration\n    \
    H025 LintAllowedInManifest: A manifest lints table that allows a lint\n    \
    H026 LintSuppression: An attribute that suppresses or lowers lints over code written in the\n    \
    workspace\n    \
    H027 RequiredClippySettingMissing: A clippy configuration without a required setting\n    \
    H028 RequiredLintNotDenied: A workspace member that leaves a required lint or lint group below\n    \
    deny\n    \
    H029 RustVersionBelowSupported: A rust-version older than the supported Rust\n    \
    H030 RustfmtConfigurationAmbiguous: A directory with both rustfmt configuration files\n    \
    H031 RustfmtConfigurationInvalid: A rustfmt configuration that is not valid TOML\n    \
    H032 RustfmtConfigurationMissing: A workspace without a rustfmt configuration\n    \
    H033 RustfmtConfigurationUnreadable: A rustfmt configuration that cannot be read\n    \
    H034 ToolAttribute: An attribute of the clippy or rustfmt tool\n    \
    H035 UnvettedClippySetting: A clippy configuration setting that Heisenberg has not vetted\n    \
    H036 UnvettedRustfmtSetting: A rustfmt configuration setting that Heisenberg has not vetted\n  \
    4 Packages and dependencies\n    \
    H037 ConfinedPackageReachedOutsideItsUsers: A member that reaches a confined package without\n    \
    being one of its users\n    \
    H038 ConfinedPackageUserListedTwice: A confined package user listed more than once\n    \
    H039 ConfinedPackageUserNotAMember: A confined package user that is not a workspace member\n    \
    H040 ConfinedPackageUserNotReachingIt: A confined package user that never reaches the package\n    \
    H041 PackageDependencyCycle: Packages that depend on each other through dev-dependencies\n    \
    H042 PackageDependsOnItself: A package that depends on itself through dev-dependencies\n    \
    H043 UnknownConfinedPackage: A confined package that the workspace does not resolve\n    \
    H044 UnpinnedDependencyRequirement: A dependency requirement that is not a MAJOR.MINOR version\n    \
    or a git rev\n    \
    H045 TestDependencyInProduction: A production dependency that only test code uses\n    \
    H046 TestLibraryInProductionPackage: A test library declared by a production package\n    \
    H047 UnusedDependency: A dependency that no code uses\n    \
    H048 PathPackageOutsideWorkspace: A package built from a local path that is not a workspace\n    \
    member\n  \
    5 Modules and items\n    \
    H049 AdditionalPublicItem: A module with more than one public item\n    \
    H050 GlobImport: A glob import\n    \
    H051 ImplWithoutLocalType: An impl of another crate's trait with no home in this crate\n    \
    H052 ImportInsideBlock: An import written inside a block\n    \
    H053 ItemOutOfOrder: An item that follows an item of a later group\n    \
    H054 MethodOutOfOrder: A function of an impl block or trait that follows one it belongs before\n    \
    H055 MisplacedBlanketImpl: A blanket impl outside the file of its trait\n    \
    H056 MisplacedForeignTypeImpl: A local trait's impl for a foreign type outside the trait's file\n    \
    H057 MisplacedImpl: An impl block outside the file of the type it belongs to\n    \
    H058 MisplacedTraitObjectImpl: An impl for a trait object outside the file of its trait\n    \
    H059 ModuleCycle: Modules that depend on each other\n    \
    H060 ModuleNotNamedAfterItsItem: A module that is not named after its public item\n    \
    H061 NonConstantInConstantsModule: A public item other than a constant in crate::constants\n    \
    H062 PublicAssociatedConstant: A public associated constant of an inherent impl\n    \
    H063 PublicConstantOutsideConstantsModule: A public constant outside crate::constants\n    \
    H064 Reexport: A re-export\n    \
    H065 ItemMoreVisibleThanItsUses: An exported item that no other crate target uses\n  \
    6 Names, data and functions\n    \
    H066 DuplicatedFunctionBody: Functions whose signatures and bodies are identical\n    \
    H067 NonDocComment: A comment that is not a doc comment\n    \
    H068 PositionalFields: A struct or variant with two or more positional fields\n    \
    H069 ProceduralFunction: A function that takes a type of its crate without belonging to it\n    \
    H070 SingleLetterName: A parameter name of a single character\n    \
    H071 TupleInSignature: A tuple type in an item signature\n    \
    H072 UndestructuredParameter: A parameter used only through its fields\n    \
    H073 UnprefixedTypeParameter: A type parameter not named T followed by a capitalized word\n  \
    7 Error types\n    \
    H074 ErrorMessageFunction: An error message formatted by a function\n    \
    H075 ErrorMessageOnTheEnum: An error message written on the enum instead of its variants\n    \
    H076 ErrorSourceOutsideChain: An error held outside the source chain\n    \
    H077 ErrorTypeNotAnEnum: An error type deriving thiserror::Error that is not an enum\n    \
    H078 ErrorTypeOutsideItsModule: An error type outside a module file named after it\n    \
    H079 HandwrittenErrorDisplay: An error type that formats its message by hand\n    \
    H080 HandwrittenErrorType: An error type implemented by hand\n    \
    H081 MacroGeneratedErrorType: An error type implemented by a macro other than thiserror::Error\n    \
    H082 MultipleErrorTypes: A crate with more than one error type\n    \
    H083 SourceInErrorMessage: An error message that formats its own source\n    \
    H084 UntypedResultError: A Result whose error type erases the failure\n    \
    H085 ResultErrorNotAnError: A Result whose error type is a workspace type that is not an error\n    \
    H086 ErrorIssuedOutsideItsCrate: An error type returned or constructed by a crate that does not\n    \
    own it\n  \
    8 Tests\n    \
    H087 IgnoredTest: A test that the test run skips\n    \
    H088 ItemUsedOnlyByTests: A production item that only tests use\n    \
    H089 NextestConfigurationMissing: A workspace without a nextest configuration\n    \
    H090 NextestOverride: A per-test override in the nextest configuration\n    \
    H091 NextestTimeoutNotEnforced: A nextest profile whose slow-timeout exceeds the limits\n    \
    H092 NonTestItemInTestTarget: An integration test item that is neither a test nor a file-local\n    \
    helper\n    \
    H093 PublishableTestCrate: A declared test crate that can be published\n    \
    H094 ReferencedTest: A test that other code references\n    \
    H095 SplitIntegrationTests: Integration tests spread over several targets or discovered\n    \
    automatically\n    \
    H096 TestCrateDeclaredTwice: A test crate listed more than once\n    \
    H097 TestCrateNotAMember: A declared test crate that is not a workspace member\n    \
    H098 TestCrateUsedByProduction: A declared test crate that production code depends on\n    \
    H099 TestModuleInItsOwnFile: A test module declared in a file of its own\n    \
    H100 TestModuleNotNamedTests: A test module that is not named tests\n    \
    H101 TestOnlyCodeInProduction: Code that depends on cfg(test) outside a test module\n    \
    H102 TestOutsideTestModule: A test written outside a test module\n    \
    H103 TestsOnlyPackageWithoutTestLibraries: A package of tests that holds no test library of its\n    \
    own\n    \
    H104 UnneededNextestConfiguration: A nextest configuration in a workspace without libtest tests\n    \
    H105 UnvettedNextestSetting: A nextest configuration setting that Heisenberg has not vetted\n  \
    9 Benchmarks\n    \
    H106 BenchmarkTargetExcludedFromBenchmarks: A benchmark target that cargo bench never runs\n    \
    H107 BenchmarkTargetRunByTests: A benchmark target that the test run runs\n    \
    H108 BenchmarkTargetWithLibtestHarness: A benchmark target built with libtest's harness\n    \
    H109 BenchmarkTargetWithoutCriterionBenchmark: A benchmark target that registers no criterion\n    \
    benchmark\n    \
    H110 BenchmarksOnlyPackageWithoutLibraries: A package of benchmarks that holds no library of its\n    \
    own\n    \
    H111 ItemUsedOnlyByBenchmarks: A production item that only benchmarks use\n    \
    H112 MustUseBenchmarkOutput: A measured routine whose must-use result criterion drops\n    \
    H113 NonBenchmarkItemInBenchmarkTarget: A benchmark target item that is neither its entry point\n    \
    nor a file-local helper\n    \
    H114 UndeterminedBenchmarkOutput: A measured routine whose return type the analysis cannot\n    \
    determine\n    \
    H115 UnvettedCriterionMethod: A criterion method that Heisenberg has not vetted\n\
    \n\
    Run `heisenberg explain <ISSUE_TYPE>` for the definition of a rule, why it exists and how to satisfy\n\
    it, and `heisenberg catalogue` for the scope and the evaluation of every rule.\n\
    \n\
    Usage: heisenberg check [OPTIONS] --cargo-home <CARGO_HOME> --configuration <NAME> --sysroot <SYSROOT> --workspace <WORKSPACE>\n\
    \n\
    Options:\n      \
    --cargo-home <CARGO_HOME>\n          \
    Absolute path of the Cargo home directory holding the downloaded dependency sources\n          \
    \n          \
    [env: CARGO_HOME]\n\
    \n      \
    --configuration <NAME>\n          \
    Analyse this declared configuration; repeat it to select several\n\
    \n      \
    --jobs <JOBS>\n          \
    Number of analysis worker threads, cargo jobs and proc-macro server processes\n          \
    \n          \
    [default: 8]\n\
    \n      \
    --sysroot <SYSROOT>\n          \
    Absolute path of the Rust 1.99.1 sysroot that contains the rust-src component\n\
    \n      \
    --workspace <WORKSPACE>\n          \
    Absolute path of the Cargo workspace directory that contains Cargo.toml\n\
    \n  \
    -h, --help\n          \
    Print help (see a summary with '-h')\n";

#[test]
fn check_help_lists_every_rule() {
    let error = HeisenbergCommand::try_parse_from([PROGRAM_NAME, CHECK_SUBCOMMAND, HELP_ARGUMENT])
        .expect_err("the help must be displayed instead of a command");

    assert_eq!(error.kind(), ErrorKind::DisplayHelp);
    assert_eq!(error.to_string(), CHECK_HELP);
}
