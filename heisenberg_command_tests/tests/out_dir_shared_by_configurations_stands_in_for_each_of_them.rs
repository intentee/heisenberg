use std::str;

use heisenberg_command::heisenberg_exit_status::HeisenbergExitStatus;
use heisenberg_command_tests::fixture_workspace::FixtureWorkspace;
use heisenberg_command_tests::heisenberg_check::HeisenbergCheck;
use heisenberg_command_tests::nextest_presence_evaluation::NextestPresenceEvaluation;

#[test]
fn out_dir_shared_by_configurations_stands_in_for_each_of_them() {
    let outcome = HeisenbergCheck {
        workspace: &FixtureWorkspace {
            name: "out_dir_shared_by_configurations",
        }
        .path(),
    }
    .run()
    .expect("the arguments must parse");

    assert_eq!(
        str::from_utf8(&outcome.output),
        Ok([
            "BuildScriptEnvironmentVariable: 1 violation\n\
            Cargo hands such a variable to the compiler after the variables it sets itself, so it reaches every tool that runs in place of the compiler: clippy reads its lint arguments and the location of its configuration from that environment, and a build script can switch off the lints the workspace denies. Write the value into a file in OUT_DIR and bring it in with include!, or declare a fixed value in the [env] table of the cargo configuration.\n\
            \n\
            - Cargo.toml: Declared output of the build script of bindings_sys sets environment variables for the compiler\n      \
            sets DECLARED_VALUE\n\
            \n\
            ------------------------------\n\
            1 errors found in 1 rule\n\
            ------------------------------\n",
            NextestPresenceEvaluation::NotEvaluated
                .host_footer(3, 5)
                .as_str()
        ]
        .concat()
        .as_str())
    );
    assert_eq!(outcome.status, HeisenbergExitStatus::Violations);
}
