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
    Precondition: rustc 1.100.0 compiles the workspace in the measured configuration, which\n\
    --configuration selects exactly once, under the preconditions of check, and --coverage-export names\n\
    the JSON export that `cargo llvm-cov report --json --include-build-script` writes after a run of the\n\
    tests of that configuration built from the current sources, without --remap-path-prefix,\n\
    --summary-only, --skip-functions, or branch or MC/DC coverage. Every measured target is built into\n\
    the instrumented run, as --lib --bins --tests --examples builds them, and executed under it: build\n\
    scripts run during the build, and examples run as tests with harness = false or through cargo\n\
    llvm-cov run --example, whose profiles the export merges; benchmarks never run in the test run, so\n\
    the run builds no benchmark target, and an export that records regions in a module file that only\n\
    benchmark targets include fails the run. An export that cannot be read, that is not JSON, not an\n\
    llvm-cov JSON export in the format 3.1.0 or not made with its functions and regions, that covers\n\
    another workspace, that is older than a module file of a workspace member, that names a relative\n\
    file or workspace manifest, or that records branch or MC/DC coverage, a region that is not a code\n\
    region, lies before the first line or column, ends before it starts or refers to a file its function\n\
    does not list, a region outside the current text of its file, a region in code that the\n\
    configuration leaves inactive, regions in a file of a workspace member that no crate target of the\n\
    configuration includes as a module file, a module file whose metadata or modification time cannot be\n\
    read, a macro call outside attributes, an attribute or a derive that rust-analyzer cannot resolve,\n\
    or a macro invocation that it cannot expand or parse, anywhere in the code of a measured crate\n\
    target outside test code, including the build script output that include! brings in, or production\n\
    code or an attribute that only the build without cfg(test) compiles, which rust-analyzer does not\n\
    analyse, fails the run, as does every failure that keeps check from loading the project.\n\
    \n\
    Measurement: production code is the text written in the module files of every crate target that is\n\
    neither an integration test target, a benchmark target nor a target of a test crate, libraries,\n\
    binaries, build scripts and examples alike, active in the measured configuration and outside the\n\
    modules that cfg(test) gates, inline or in their own file; text written outside module files, such\n\
    as the build script output that include! brings in, and test code are not measured. A measured\n\
    function is a function with a body whose opening brace is production code, written directly or by a\n\
    macro that production code calls, a macro_rules! macro, a procedural macro that passes its input\n\
    through or an attribute macro, whose expansion stands for the item it rewrites; functions written\n\
    from the same text are one function, reported where their name is first written, and the functions\n\
    that procedural macros generate are not measured functions, so derive macros are not expanded, nor\n\
    are the macro calls inside attributes, which write no functions. The regions of the export, merged\n\
    by position across every instrumented build and instantiation, each executed when any of them\n\
    executed it, belong to the innermost measured function that holds the text they start at, through\n\
    every macro expansion, and to each of them when several functions are written from that text, the\n\
    regions of a closure to its enclosing function; a region outside every function belongs to the\n\
    innermost item that holds the text it starts at, such as a static or a macro_rules! definition.\n\
    Every measured function holds at least one region and every region of production code is executed;\n\
    there is no threshold, no exemption and no filter.\n\
    \n\
    Report: Issues are grouped by type, in one section per type, as check reports them, followed by the\n\
    measured configuration with its crate targets, functions and regions and by the declared\n\
    configurations it does not measure.\n\
    \n\
    Rules: every rule is mandatory and decidable, and is listed under its section with its identifier.\n  \
    10 Coverage\n    \
    H116 CodeWithoutCoverageData: A production function without coverage data\n    \
    H117 UncoveredCode: Production code that no test executes\n\
    \n\
    Run `heisenberg explain <ISSUE_TYPE>` for the definition of a rule, why it exists and how to satisfy\n\
    it, and `heisenberg catalogue` for the scope and the evaluation of every rule.\n\
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
    Absolute path of the Rust 1.100.0 sysroot that contains the rust-src component\n\
    \n      \
    --workspace <WORKSPACE>\n          \
    Absolute path of the Cargo workspace directory that contains Cargo.toml\n\
    \n      \
    --coverage-export <COVERAGE_EXPORT>\n          \
    Absolute path of the coverage export that `cargo llvm-cov report --json` writes\n\
    \n  \
    -h, --help\n          \
    Print help (see a summary with '-h')\n";

#[test]
fn coverage_check_help_lists_every_coverage_rule() {
    let error =
        HeisenbergCommand::try_parse_from([PROGRAM_NAME, COVERAGE_CHECK_SUBCOMMAND, HELP_ARGUMENT])
            .expect_err("the help must be displayed instead of a command");

    assert_eq!(error.kind(), ErrorKind::DisplayHelp);
    assert_eq!(error.to_string(), COVERAGE_CHECK_HELP);
}
