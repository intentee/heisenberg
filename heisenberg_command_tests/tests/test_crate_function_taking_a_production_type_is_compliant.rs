use std::str;

use heisenberg_command::heisenberg_exit_status::HeisenbergExitStatus;
use heisenberg_command_tests::fixture_workspace::FixtureWorkspace;
use heisenberg_command_tests::heisenberg_check::HeisenbergCheck;

#[test]
fn test_crate_function_taking_a_production_type_is_compliant() {
    let outcome = HeisenbergCheck {
        workspace: &FixtureWorkspace {
            name: "procedural_function_in_a_test_crate",
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
            Analysed on x86_64-pc-windows-msvc: 3 crate targets, 4 modules\n\
            Analysed on x86_64-unknown-linux-gnu: 3 crate targets, 4 modules\n\
            Precondition: rustc 1.99.1 compiles the workspace for every declared target\n")
    );
    assert_eq!(outcome.status, HeisenbergExitStatus::Compliant);
}
