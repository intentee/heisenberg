use clap::Parser;
use clap::error::ErrorKind;

use heisenberg_command::heisenberg_command::HeisenbergCommand;
use heisenberg_command_tests::constants::COVERAGE_CHECK_SUBCOMMAND;
use heisenberg_command_tests::constants::HELP_ARGUMENT;
use heisenberg_command_tests::constants::PROGRAM_NAME;

const COVERAGE_CHECK_HELP: &str = "Report the production code of a Cargo workspace that its tests leave unexecuted, as the coverage\n\
    export of a cargo llvm-cov run of one declared configuration records it.\n\
    \n\
    Settings: the [workspace.metadata.heisenberg] table that check reads, whose configurations and\n\
    test_crates are validated as check validates them; `heisenberg check --help` describes it.\n\
    \n\
    Precondition: rustc 1.99.1 compiles the workspace in the measured configuration, which\n\
    --configuration selects exactly once, under the preconditions of check, and --coverage-export names\n\
    the JSON export that `cargo llvm-cov report --json --include-build-script` writes after a run of the\n\
    tests of that configuration built from the current sources, without --remap-path-prefix,\n\
    --summary-only, --skip-functions, or branch or MC/DC coverage. Every measured target is built into\n\
    the instrumented run, as --all-targets builds them, and executed under it: build scripts run during\n\
    the build, benches run as tests, and examples run as tests with harness = false or through cargo\n\
    llvm-cov run --example, whose profiles the export merges. An export that cannot be read, that is not\n\
    JSON, not an llvm-cov JSON export in the format 3.1.0 or not made with its functions and regions,\n\
    that covers another workspace, that is older than a module file of a workspace member, that names a\n\
    relative file or workspace manifest, or that records branch or MC/DC coverage, a region that is not\n\
    a code region, lies before the first line or column, ends before it starts or refers to a file its\n\
    function does not list, a region outside the current text of its file, a region in code that the\n\
    configuration leaves inactive, regions in a file of a workspace member that no crate target of the\n\
    configuration includes as a module file, a module file whose metadata or modification time cannot be\n\
    read, a macro call outside attributes, an attribute or a derive that rust-analyzer cannot resolve,\n\
    or a macro invocation that it cannot expand or parse, anywhere in the code of a measured crate\n\
    target outside test code, including the build script output that include! brings in, or production\n\
    code or an attribute that only the build without cfg(test) compiles, which rust-analyzer does not\n\
    analyse, fails the run, as does every failure that keeps check from loading the project.\n\
    \n\
    Measurement: production code is the text written in the module files of every crate target that is\n\
    neither an integration test target nor a target of a test crate, libraries, binaries, build scripts,\n\
    examples and benches alike, active in the measured configuration and outside the modules that\n\
    cfg(test) gates, inline or in their own file; text written outside module files, such as the build\n\
    script output that include! brings in, and test code are not measured. A measured function is a\n\
    function with a body whose opening brace is production code, written directly or by a macro that\n\
    production code calls, a macro_rules! macro, a procedural macro that passes its input through or an\n\
    attribute macro, whose expansion stands for the item it rewrites; functions written from the same\n\
    text are one function, reported where their name is first written, and the functions that procedural\n\
    macros generate are not measured functions, so derive macros are not expanded, nor are the macro\n\
    calls inside attributes, which write no functions. The regions of the export, merged by position\n\
    across every instrumented build and instantiation, each executed when any of them executed it,\n\
    belong to the innermost measured function that holds the text they start at, through every macro\n\
    expansion, and to each of them when several functions are written from that text, the regions of a\n\
    closure to its enclosing function; a region outside every function stands alone. Every measured\n\
    function holds at least one region and every region of production code is executed; there is no\n\
    threshold, no exemption and no filter.\n\
    \n\
    Report: Issues are grouped by type, in one section per type, as check reports them, followed by the\n\
    measured configuration with its crate targets, functions and regions and by the declared\n\
    configurations it does not measure.\n\
    \n\
    Rules:\n  \
    CodeWithoutCoverageData: A production function without coverage data\n  \
    UncoveredCode: Production code that no test executes\n\
    \n\
    Run `heisenberg explain <ISSUE_TYPE>` for the definition of a rule, why it exists and how to satisfy\n\
    it.\n\
    \n\
    Usage: heisenberg coverage-check [OPTIONS] --cargo-home <CARGO_HOME> --sysroot <SYSROOT> --workspace <WORKSPACE> --coverage-export <COVERAGE_EXPORT> --configuration <NAME>\n\
    \n\
    Options:\n      \
    --cargo-home <CARGO_HOME>\n          \
    Absolute path of the Cargo home directory holding the downloaded dependency sources\n          \
    \n          \
    [env: CARGO_HOME]\n\
    \n      \
    --configuration <NAME>\n          \
    Measure the coverage of this declared configuration\n\
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
    \n      \
    --coverage-export <COVERAGE_EXPORT>\n          \
    Absolute path of the coverage export that `cargo llvm-cov report --json` writes\n\
    \n  \
    -h, --help\n          \
    Print help (see a summary with '-h')\n\
    ";

#[test]
fn coverage_check_help_lists_every_coverage_rule() {
    let error =
        HeisenbergCommand::try_parse_from([PROGRAM_NAME, COVERAGE_CHECK_SUBCOMMAND, HELP_ARGUMENT])
            .expect_err("the help must be displayed instead of a command");

    assert_eq!(error.kind(), ErrorKind::DisplayHelp);
    assert_eq!(error.to_string(), COVERAGE_CHECK_HELP);
}
