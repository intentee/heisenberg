use std::str;

use heisenberg_command::heisenberg_exit_status::HeisenbergExitStatus;
use heisenberg_command_tests::heisenberg_check::HeisenbergCheck;
use heisenberg_fixtures::fixture_workspace::FixtureWorkspace;

#[test]
fn environment_variable_declared_in_the_configuration_has_its_value_in_the_analysis() {
    let outcome = HeisenbergCheck {
        workspace: &FixtureWorkspace {
            name: "declared_environment_value",
        }
        .path(),
    }
    .run()
    .expect("the arguments must parse");

    assert_eq!(
        str::from_utf8(&outcome.output),
        Ok("------------------------------\n\
            No errors found!\n\
            ------------------------------\n\
            Analysed on x86_64-pc-windows-msvc: 1 crate targets, 1 modules\n\
            Analysed on x86_64-unknown-linux-gnu: 1 crate targets, 1 modules\n\
            Precondition: rustc 1.99.1 compiles the workspace for every declared target\n")
    );
    assert_eq!(outcome.status, HeisenbergExitStatus::Compliant);
}
