use std::str;

use heisenberg_command::heisenberg_exit_status::HeisenbergExitStatus;
use heisenberg_command_tests::fixture_workspace::FixtureWorkspace;
use heisenberg_command_tests::heisenberg_check::HeisenbergCheck;
use heisenberg_command_tests::nextest_presence_evaluation::NextestPresenceEvaluation;

#[test]
fn build_script_environment_variables_are_violations() {
    let outcome = HeisenbergCheck {
        workspace: &FixtureWorkspace {
            name: "build_script_environment",
        }
        .path(),
    }
    .run()
    .expect("the arguments must parse");

    assert_eq!(
        str::from_utf8(&outcome.output),
        Ok([
            "BuildScriptEnvironmentVariable: 2 violations\n\
            Cargo hands such a variable to the compiler after the variables it sets itself, so it reaches every tool that runs in place of the compiler: clippy reads its lint arguments and the location of its configuration from that environment, and a build script can switch off the lints the workspace denies. Write the value into a file in OUT_DIR and bring it in with include!, or declare a fixed value in the [env] table of the cargo configuration.\n\
            \n\
            - Cargo.toml: Declared output of the build script of declared sets environment variables for the compiler\n      \
            sets DECLARED_STAMP\n\
            - emitter/build.rs: Build script of emitter sets environment variables for the compiler\n      \
            sets CLIPPY_ARGS\n      \
            sets GENERATED_STAMP\n\
            \n\
            ------------------------------\n\
            2 errors found in 1 rule\n\
            ------------------------------\n",
            NextestPresenceEvaluation::NotEvaluated
                .host_footer(4, 6)
                .as_str()
        ]
        .concat()
        .as_str())
    );
    assert_eq!(outcome.status, HeisenbergExitStatus::Violations);
}
