use std::str;

use heisenberg_command::heisenberg_exit_status::HeisenbergExitStatus;
use heisenberg_command_tests::fixture_workspace::FixtureWorkspace;
use heisenberg_command_tests::heisenberg_check::HeisenbergCheck;
use heisenberg_command_tests::nextest_presence_evaluation::NextestPresenceEvaluation;

#[test]
fn undeclared_environment_variable_is_a_violation() {
    let outcome = HeisenbergCheck {
        workspace: &FixtureWorkspace {
            name: "undeclared_environment_variables",
        }
        .path(),
    }
    .run()
    .expect("the arguments must parse");

    assert_eq!(
        str::from_utf8(&outcome.output),
        Ok(["UndeclaredEnvironmentVariable: 3 violations\n\
            The compiler reads such a variable from the shell that starts the build, so the code that is compiled depends on an input that no file declares, and the analysis sees a placeholder in its place. Declare the variable in the [env] table of the cargo configuration, or have the build script emit it with cargo::rustc-env.\n\
            \n\
            - src/undeclared.rs:4:14: Environment variable UNDECLARED_BY_THE_FIXTURE is declared by neither cargo, the cargo configuration nor the build script\n            \
            Some(env!(\"UNDECLARED_BY_THE_FIXTURE\")),\n\
            - src/undeclared.rs:5:9: Environment variable OPTIONAL_AND_UNDECLARED_BY_THE_FIXTURE is declared by neither cargo, the cargo configuration nor the build script\n            \
            option_env!(\"OPTIONAL_AND_UNDECLARED_BY_THE_FIXTURE\"),\n\
            - src/undeclared.rs:6:14: Environment variable CARGO_BIN_NAME is declared by neither cargo, the cargo configuration nor the build script\n            \
            Some(env!(\"CARGO_BIN_NAME\")),\n\
            \n\
            ------------------------------\n\
            3 errors found in 1 rule\n\
            ------------------------------\n", NextestPresenceEvaluation::Evaluated.host_footer(4, 6).as_str()].concat().as_str())
    );
    assert_eq!(outcome.status, HeisenbergExitStatus::Violations);
}
