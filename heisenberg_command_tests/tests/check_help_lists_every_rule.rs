use clap::Parser;
use clap::error::ErrorKind;

use heisenberg_command::heisenberg_command::HeisenbergCommand;

const CHECK_HELP: &str = "Report module dependency cycles and structural violations in every crate of a Cargo workspace, for\n\
    the targets it declares.\n\
    \n\
    Settings: the workspace root manifest holds every Heisenberg setting in its\n\
    [workspace.metadata.heisenberg] table, whose required key, targets, lists the target triples the\n\
    workspace supports, each once, among aarch64-apple-darwin, x86_64-pc-windows-msvc,\n\
    x86_64-unknown-linux-gnu, and whose optional out_dirs table declares, for a package of the\n\
    workspace, member or dependency, whose build script cannot run on every host, a directory, relative\n\
    to the workspace root, that holds one subdirectory per declared target with the files the build\n\
    script writes to OUT_DIR, and a targets table that gives, for each declared target, the cfgs and env\n\
    the build script emits; that build script then never runs, and the declared files, cfgs and env\n\
    stand in for its output; its optional test_crates list names the workspace members that hold only\n\
    code for tests. A missing table, an unknown key, an empty target list, an unknown target, a target\n\
    listed twice, an out-dir for a package the workspace does not resolve, for a name that several\n\
    resolved versions share or for a package without a build script, an out-dir outside the workspace,\n\
    one without a readable subdirectory or without the output of a declared target, one with the output\n\
    of an undeclared target, and a declared cfg that does not parse fail the run. A test_crates entry\n\
    that is not a workspace member, that is listed twice, whose package can be published, or that a\n\
    member other than a test crate depends on through a normal or build dependency is a violation, and\n\
    an entry that fails these conditions is not a test crate. No setting exempts code from any rule:\n\
    test_crates declares which members are test code, and Heisenberg verifies the declaration.\n\
    \n\
    Precondition: rustc 1.99.1 compiles the workspace for every declared target with the standard\n\
    library of each installed, Cargo.lock is up to date, the cargo home holds every package of\n\
    Cargo.lock (`cargo fetch`), the build runs from the workspace directory with the environment\n\
    Heisenberg runs with and without --config arguments, and clippy runs without command-line arguments.\n\
    An environment variable whose name or value is not UTF-8, CARGO_ENCODED_RUSTFLAGS, RUSTFLAGS,\n\
    CLIPPY_ARGS, CLIPPY_CONF_DIR or RUSTC_BOOTSTRAP, __CARGO_TEST_CHANNEL_OVERRIDE_DO_NOT_USE_THIS in\n\
    the environment, a workspace directory that is not the root of its cargo workspace, a workspace\n\
    member or crate root outside the workspace root, a package whose metadata cargo_metadata cannot\n\
    read, a crate root that two targets share or that cannot be read, a metabuild script, which has no\n\
    source file, a workspace root, target directory or crate root that is not an absolute UTF-8 path, a\n\
    clippy.toml or .clippy.toml that discovery finds and cannot read, or that is the only one in its\n\
    directory and is not valid TOML, rustflags in the [build] or [target] tables of the cargo\n\
    configuration or in the CARGO_BUILD_RUSTFLAGS and CARGO_TARGET_<TRIPLE>_RUSTFLAGS variables of a\n\
    supported target, and CLIPPY_ARGS, CLIPPY_CONF_DIR or RUSTC_BOOTSTRAP in the [env] table of the\n\
    cargo configuration, under any letter case, fail the run.\n\
    \n\
    Analysis: Values that the [env] table of the cargo configuration declares reach the analysis as\n\
    cargo resolves them, those of the variables cargo sets per build, such as CARGO_BIN_EXE_<name> and\n\
    CARGO_TARGET_TMPDIR, are trusted, and OUT_DIR and the cfgs and env a build script emits come from\n\
    running it, or from its out_dirs declaration. The analysis enables all features and debug\n\
    assertions, and test only for the targets cargo builds with libtest's harness, as rustc's --test\n\
    does: a target with harness = false, or one that cargo neither tests nor benchmarks, such as a build\n\
    script or an example, is analysed without it. Build scripts and procedural macros run: for each\n\
    declared target, cargo builds the procedural macros offline and runs the build scripts, apart from\n\
    those out_dirs declares, and the sysroot's rust-analyzer-proc-macro-srv expands the procedural\n\
    macros, whose output is analysed like any macro expansion, its input where it is written and the\n\
    tokens it generates trusted, and a build script that fails or emits a cfg that does not parse, one\n\
    that needs a build script out_dirs declares, a build script output that cannot be read, a procedural\n\
    macro that fails to build or that the server cannot load, a declared target without its standard\n\
    library and a sysroot without the server, or whose server cannot start, fail the run. Every decision\n\
    is semantic, as rust-analyzer resolves the code. Shared module files are analysed in every crate\n\
    that includes them. The public item, constant and integration test target rules apply to items\n\
    written in module files; an item that an attribute macro rewrites is analysed through the items of\n\
    its expansion that rust-analyzer maps back to it, and the rest of the expansion, like the items\n\
    other macros generate, is trusted. Derive macro output is analysed like any macro expansion, except\n\
    that of the built-in derives that rust-analyzer records as impls of their trait without expanding\n\
    them, which is trusted; text written outside a crate's module files, such as in other crates or in\n\
    included files, is trusted, apart from the comments of included files, and results reflect the files\n\
    on disk, including generated ones. Test code is every target of a test crate, every integration test\n\
    target and every module that cfg(test) gates, inline or in its own file and under any name; a test\n\
    crate is a workspace member that test_crates lists, whose package cannot be published (publish =\n\
    false), and that no member other than a test crate depends on through a normal or build dependency.\n\
    \n\
    Report: Issues are grouped by type, in one section per type: a `<IssueType>: <count> violation(s)`\n\
    line, a line that states the purpose of the rule and what to do, an empty line, and one `-\n\
    <file>[:<line>:<column>]: <message>` line per issue, followed, four spaces in, by its source line\n\
    when it has one and by further indented lines with its details, such as the dependencies of a cycle,\n\
    the inclusions of a shared module file, the failing expansions of a macro call or the impl blocks of\n\
    a flagged type. Each issue is reported once, at the name of the item it concerns; when it holds only\n\
    in some of the crate targets or platforms that analyse its file, the message names them. A cause and\n\
    its direct consequences form one issue.\n\
    \n\
    Rules:\n  \
    AdditionalPublicItem: A module with more than one public item\n  \
    ClippyConfigurationAmbiguous: A directory that holds both clippy configuration files\n  \
    ClippyConfigurationMissing: A workspace member without a clippy configuration\n  \
    ClippyLintNotDenied: A workspace member that leaves a required clippy lint below deny\n  \
    ClippyTestAllowanceMissing: A clippy configuration that does not allow tests to fail fast\n  \
    CodeActiveOnNoTarget: Code that no declared target builds\n  \
    ComputedEnvironmentVariableName: An env! or option_env! argument that is no string literal\n  \
    ConfigurationSelectionMacro: A cfg_select! call\n  \
    DuplicatedFunctionBody: Functions whose signatures and bodies are identical\n  \
    FailedMacroExpansion: A macro call that fails to expand\n  \
    FunctionUsedOnlyByTests: A production function that only tests use\n  \
    GlobImport: A glob import\n  \
    ImplWithoutLocalType: An impl of another crate's trait with no home in this crate\n  \
    InvalidMacroDefinition: A macro definition that is not valid\n  \
    LintAllowedInManifest: A manifest lints table that allows a lint\n  \
    LintSuppression: An attribute that allows or expects a lint, or lowers a required one\n  \
    MisplacedBlanketImpl: A blanket impl outside the file of its trait\n  \
    MisplacedForeignTypeImpl: A local trait's impl for a foreign type outside the trait's file\n  \
    MisplacedImpl: An impl block outside the file of the type it belongs to\n  \
    MisplacedTraitObjectImpl: An impl for a trait object outside the file of its trait\n  \
    ModuleCycle: Modules that depend on each other\n  \
    ModuleFileInsideBlock: A module file declared inside a block\n  \
    ModuleNotNamedAfterItsItem: A module that is not named after its public item\n  \
    NegatedConfiguration: A cfg predicate that negates a feature, test or debug_assertions\n  \
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
    SharedModuleFile: A module file included as several modules\n  \
    TestCrateDeclaredTwice: A test crate listed more than once\n  \
    TestCrateNotAMember: A declared test crate that is not a workspace member\n  \
    TestCrateUsedByProduction: A declared test crate that production code depends on\n  \
    TestOnlyCodeInProduction: Code that exists only under cfg(test) outside test code\n  \
    TestsOnlyPackageWithoutTestLibraries: A package of tests that holds no test library of its own\n  \
    UndeclaredEnvironmentVariable: An env! or option_env! read of a variable only the shell sets\n  \
    UnimplementedBuiltinMacro: A built-in macro that rust-analyzer cannot expand\n  \
    UnmappedItem: An item that cannot be mapped to its definition\n  \
    UnparsableMacroExpansion: A macro expansion that does not parse\n  \
    UnresolvedExternCrate: An extern crate that does not resolve\n  \
    UnresolvedMacroCall: A macro call, attribute or derive whose macro does not resolve\n  \
    UnresolvedMethodCall: A method call that does not resolve\n  \
    UnresolvedModule: A module declaration whose file does not exist\n  \
    UnresolvedPath: A path that does not resolve\n\
    \n\
    Run `heisenberg explain <ISSUE_TYPE>` for the definition of a rule, why it exists and how to satisfy\n\
    it.\n\
    \n\
    Usage: heisenberg check --cargo-home <CARGO_HOME> --sysroot <SYSROOT> --workspace <WORKSPACE>\n\
    \n\
    Options:\n      \
    --cargo-home <CARGO_HOME>\n          \
    Absolute path of the Cargo home directory holding the downloaded dependency sources\n          \
    \n          \
    [env: CARGO_HOME]\n\
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
