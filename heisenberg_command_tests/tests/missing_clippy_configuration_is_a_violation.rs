use std::str;

use heisenberg_command::heisenberg_exit_status::HeisenbergExitStatus;
use heisenberg_command_tests::fixture_workspace::FixtureWorkspace;
use heisenberg_command_tests::heisenberg_check::HeisenbergCheck;
use heisenberg_command_tests::host_footer::host_footer;

#[test]
fn missing_clippy_configuration_is_a_violation() {
    let outcome = HeisenbergCheck {
        workspace: &FixtureWorkspace {
            name: "clippy_configuration_missing",
        }
        .path(),
    }
    .run()
    .expect("the arguments must parse");

    assert_eq!(
        str::from_utf8(&outcome.output),
        Ok(["ClippyConfigurationMissing: 1 violation\n\
            Tests may fail fast while production code may not, and clippy needs a configuration to tell them apart. Add a clippy.toml with the test allowances.\n\
            \n\
            - Cargo.toml: Package clippy_configuration_missing has no clippy.toml or .clippy.toml within the workspace\n\
            \n\
            ------------------------------\n\
            1 errors found\n\
            ------------------------------\n", host_footer(1, 1).as_str()].concat().as_str())
    );
    assert_eq!(outcome.status, HeisenbergExitStatus::Violations);
}
