use clap::Parser;
use clap::error::ErrorKind;

use heisenberg_command::heisenberg_command::HeisenbergCommand;

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
    script cannot run on every host, a configurations table that gives, for each declared configuration,\n\
    a directory, relative to the workspace root, with the files the build script writes to OUT_DIR, and\n\
    the cfgs and env the build script emits; that build script then never runs, and the declared files,\n\
    cfgs and env stand in for its output; its optional test_crates list names the workspace members that\n\
    hold only code for tests. A missing table, an unknown key, an empty configuration list, an empty or\n\
    repeated configuration name, a configuration with an unknown target, with a feature that no\n\
    workspace member defines or with a feature listed twice, two configurations with the same target and\n\
    features, a configuration that selects features other than the default ones in a workspace that uses\n\
    resolver 1 with a root package and other members, where cargo applies a feature selection to the\n\
    root package only, an out-dir for a package the workspace does not resolve, for a name that several\n\
    resolved versions share or for a package without a build script, an out-dir directory outside the\n\
    workspace or that cannot be read, an out-dir without the output of a declared configuration or with\n\
    the output of an undeclared configuration, and a declared cfg that does not parse fail the run. A\n\
    test_crates entry that is not a workspace member, that is listed twice, whose package can be\n\
    published, or that a member other than a test crate depends on through a normal or build dependency\n\
    is a violation, and an entry that fails these conditions is not a test crate. No setting exempts\n\
    code from any rule: test_crates declares which members are test code, and Heisenberg verifies the\n\
    declaration.\n\
    \n\
    Precondition: rustc 1.99.1 compiles the workspace in every analysed configuration with the standard\n\
    library of its target installed, Cargo.lock is up to date, the cargo home holds every package of\n\
    Cargo.lock (`cargo fetch`), the build runs from the workspace directory with the environment\n\
    Heisenberg runs with and without --config arguments, clippy runs without command-line arguments, and\n\
    nextest runs without the command-line arguments that replace or extend its configuration, retry\n\
    tests or disable timeouts (--config-file, --tool-config-file, --retries, --debugger and --tracer).\n\
    An environment variable whose name or value is not UTF-8, CARGO_ENCODED_RUSTFLAGS, RUSTFLAGS, RUSTC,\n\
    RUSTC_WORKSPACE_WRAPPER, RUSTC_WRAPPER, CLIPPY_ARGS, CLIPPY_CONF_DIR, NEXTEST_RETRIES or\n\
    RUSTC_BOOTSTRAP, __CARGO_TEST_CHANNEL_OVERRIDE_DO_NOT_USE_THIS in the environment, a workspace\n\
    directory that is not the root of its cargo workspace, a workspace member or crate root outside the\n\
    workspace root, a package whose metadata cargo_metadata cannot read, a crate root that two targets\n\
    share or that cannot be read, a metabuild script, which has no source file, a workspace root, target\n\
    directory or crate root that is not an absolute UTF-8 path, a clippy.toml or .clippy.toml that\n\
    discovery finds and cannot read, or that is the only one in its directory and is not valid TOML, a\n\
    .config/nextest.toml that cannot be read or is not a valid nextest configuration, rustflags in the\n\
    [build] or [target] tables of the cargo configuration or in the CARGO_BUILD_RUSTFLAGS and\n\
    CARGO_TARGET_<TRIPLE>_RUSTFLAGS variables of a supported target, rustc, rustc-workspace-wrapper or\n\
    rustc-wrapper in the [build] table of the cargo configuration, including through the environment\n\
    variables that set them, and CLIPPY_ARGS, CLIPPY_CONF_DIR or RUSTC_BOOTSTRAP in the [env] table of\n\
    the cargo configuration, under any letter case, fail the run. A failing run reports the first\n\
    failure among loading the project, building it, resolving the standard library of its targets and\n\
    reading its clippy and nextest configurations, in that order, and otherwise the analysis failures of\n\
    every configuration.\n\
    \n\
    Analysis: Values that the [env] table of the cargo configuration declares reach the analysis as\n\
    cargo resolves them, OUT_DIR and the cfgs and env a build script emits come from running it, or from\n\
    its out_dirs declaration, and an env! or option_env! of a variable that none of them sets expands as\n\
    rust-analyzer expands it. The analysis enables debug assertions, the features of the analysed\n\
    configuration, each on every workspace member that defines it, as cargo does, and test only for the\n\
    targets cargo builds with libtest's harness, as rustc's --test does: a target with harness = false,\n\
    or one that cargo neither tests nor benchmarks, such as a build script or an example, is analysed\n\
    without it. Every run analyses the declared configurations that --configuration selects, which it\n\
    requires, and a --configuration the workspace does not declare or selects twice fails the run. Build\n\
    scripts and procedural macros run: for each analysed configuration, cargo builds the procedural\n\
    macros offline with the rustc of the sysroot and --jobs parallel jobs, which build scripts see in\n\
    NUM_JOBS, and runs the build scripts, apart from those out_dirs declares, and --jobs processes of\n\
    the sysroot's rust-analyzer-proc-macro-srv expand the procedural macros, whose output is analysed\n\
    like any macro expansion, its input where it is written and the tokens it generates trusted, and a\n\
    build script that fails or emits a cfg that does not parse, one that needs a build script out_dirs\n\
    declares, a build script output that cannot be read, a procedural macro that fails to build or that\n\
    the server cannot load, a declared target without its standard library, a sysroot without rustc and\n\
    a sysroot without the server, or whose server cannot start, fail the run. Every decision is\n\
    semantic, as rust-analyzer resolves the code. The analysis runs on --jobs worker threads and reports\n\
    the same issues for every count. Shared module files are analysed in every crate that includes them.\n\
    The public item, constant, item order, method order and integration test target rules apply to items\n\
    written in module files, including those that a macro_rules! macro of the crate writes, and the\n\
    name, tuple and parameter rules to the text written in them; an item that an attribute macro\n\
    rewrites is analysed through the items of its expansion that rust-analyzer maps back to it, and the\n\
    rest of the expansion, like the items that procedural macros and the macros of other crates\n\
    generate, is trusted. Derive macro output is analysed like any macro expansion, except that of the\n\
    built-in derives that rust-analyzer records as impls of their trait without expanding them, which is\n\
    trusted; text written outside a crate's module files, such as in other crates or in the build script\n\
    output that include! brings in, is trusted, and results reflect the files on disk, including\n\
    generated ones. Test code is every target of a test crate, every integration test target and every\n\
    module that cfg(test) gates, inline or in its own file and under any name; a test crate is a\n\
    workspace member that test_crates lists, whose package cannot be published (publish = false), and\n\
    that no member other than a test crate depends on through a normal or build dependency.\n\
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
    CodeActiveInNoConfiguration or the empty-library check of TestsOnlyPackageWithoutTestLibraries,\n\
    which hold only over every declared configuration; a partial run without issues exits as compliant.\n\
    \n\
    Rules:\n  \
    AdditionalPublicItem: A module with more than one public item\n  \
    ClippyConfigurationAmbiguous: A directory that holds both clippy configuration files\n  \
    ClippyConfigurationMissing: A workspace member without a clippy configuration\n  \
    CodeActiveInNoConfiguration: Code that no declared configuration builds\n  \
    ConfigurationSelectionMacro: A cfg_select! call\n  \
    DuplicatedFunctionBody: Functions whose signatures and bodies are identical\n  \
    FailedMacroExpansion: A macro call that fails to expand\n  \
    FunctionUsedOnlyByTests: A production function that only tests use\n  \
    GlobImport: A glob import\n  \
    ImplWithoutLocalType: An impl of another crate's trait with no home in this crate\n  \
    IncludedSourceFile: An include! of a file that is not a build script output\n  \
    InvalidMacroDefinition: A macro definition that is not valid\n  \
    ItemOutOfOrder: An item that follows an item of a later group\n  \
    LintAllowedInManifest: A manifest lints table that allows a lint\n  \
    LintSuppression: An attribute that suppresses or lowers lints over code written in the workspace\n  \
    MethodOutOfOrder: A function of an impl block or trait that follows one it belongs before\n  \
    MisplacedBlanketImpl: A blanket impl outside the file of its trait\n  \
    MisplacedForeignTypeImpl: A local trait's impl for a foreign type outside the trait's file\n  \
    MisplacedImpl: An impl block outside the file of the type it belongs to\n  \
    MisplacedTraitObjectImpl: An impl for a trait object outside the file of its trait\n  \
    ModuleCycle: Modules that depend on each other\n  \
    ModuleFileInsideBlock: A module file declared inside a block\n  \
    ModuleNotNamedAfterItsItem: A module that is not named after its public item\n  \
    NegatedConfiguration: A cfg predicate that negates a feature, test or debug_assertions\n  \
    NextestConfigurationMissing: A workspace without a nextest configuration\n  \
    NextestOverride: A per-test override in the nextest configuration\n  \
    NextestTimeoutNotEnforced: A nextest profile whose slow-timeout exceeds the limits\n  \
    NonConstantInConstantsModule: A public item other than a constant in crate::constants\n  \
    NonDocComment: A comment that is not a doc comment\n  \
    NonTestItemInTestTarget: An integration test item that is neither a test nor a file-local helper\n  \
    PackageDependencyCycle: Packages that depend on each other through dev-dependencies\n  \
    PackageDependsOnItself: A package that depends on itself through dev-dependencies\n  \
    ProceduralFunction: A function that takes a type of its crate without belonging to it\n  \
    PublicAssociatedConstant: A public associated constant of an inherent impl\n  \
    PublicConstantOutsideConstantsModule: A public constant outside crate::constants\n  \
    PublishableTestCrate: A declared test crate that can be published\n  \
    Reexport: A re-export\n  \
    ReferencedTest: A test that other code references\n  \
    RequiredClippySettingMissing: A clippy configuration without a required setting\n  \
    RequiredLintNotDenied: A workspace member that leaves a required lint or lint group below deny\n  \
    SharedModuleFile: A module file included as several modules\n  \
    SingleLetterName: A parameter name of a single character\n  \
    TestCrateDeclaredTwice: A test crate listed more than once\n  \
    TestCrateNotAMember: A declared test crate that is not a workspace member\n  \
    TestCrateUsedByProduction: A declared test crate that production code depends on\n  \
    TestOnlyCodeInProduction: Code that exists only under cfg(test) outside test code\n  \
    TestsOnlyPackageWithoutTestLibraries: A package of tests that holds no test library of its own\n  \
    TupleInSignature: A tuple type in an item signature\n  \
    UndestructuredParameter: A parameter used only through its fields\n  \
    UnimplementedBuiltinMacro: A built-in macro that rust-analyzer cannot expand\n  \
    UnmappedItem: An item that cannot be mapped to its definition\n  \
    UnparsableMacroExpansion: A macro expansion that does not parse\n  \
    UnprefixedTypeParameter: A type parameter not named T followed by a capitalized word\n  \
    UnresolvedExternCrate: An extern crate that rust-analyzer cannot resolve\n  \
    UnresolvedMacroCall: A macro call, attribute or derive whose macro rust-analyzer cannot resolve\n  \
    UnresolvedMethodCall: A method call that rust-analyzer cannot resolve\n  \
    UnresolvedModule: A module declaration whose file rust-analyzer cannot resolve\n  \
    UnresolvedPath: A path that rust-analyzer cannot resolve\n  \
    UnsafeCode: Unsafe code in a crate that integrates no foreign code\n  \
    UnvettedClippySetting: A clippy configuration setting that Heisenberg has not vetted\n  \
    UnvettedNextestSetting: A nextest configuration setting that Heisenberg has not vetted\n\
    \n\
    Run `heisenberg explain <ISSUE_TYPE>` for the definition of a rule, why it exists and how to satisfy\n\
    it.\n\
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
    Print help (see a summary with '-h')\n\
    ";

#[test]
fn check_help_lists_every_rule() {
    let error = HeisenbergCommand::try_parse_from(["heisenberg", "check", "--help"])
        .expect_err("the help must be displayed instead of a command");

    assert_eq!(error.kind(), ErrorKind::DisplayHelp);
    assert_eq!(error.to_string(), CHECK_HELP);
}
