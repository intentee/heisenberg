use std::str;

use heisenberg_command::heisenberg_exit_status::HeisenbergExitStatus;
use heisenberg_command_tests::constants::HOST_TRIPLE;
use heisenberg_command_tests::constants::LEFT_OUT_TRIPLE;
use heisenberg_command_tests::fixture_workspace::FixtureWorkspace;
use heisenberg_command_tests::heisenberg_check::HeisenbergCheck;

#[test]
fn empty_library_is_not_judged_in_a_partial_run() {
    let outcome = HeisenbergCheck {
        workspace: &FixtureWorkspace {
            name: "library_with_only_inactive_items",
        }
        .path(),
    }
    .run_with_arguments(&["--target", HOST_TRIPLE])
    .expect("the arguments must parse");

    assert_eq!(
        str::from_utf8(&outcome.output),
        Ok(format!("------------------------------\n\
            No errors found!\n\
            ------------------------------\n\
            Analysed on {HOST_TRIPLE}: 3 crate targets, 3 modules\n\
            Not analysed: {LEFT_OUT_TRIPLE}\n\
            Features: all\n\
            Not evaluated in a partial run: CodeActiveOnNoTarget, the empty-library check of TestsOnlyPackageWithoutTestLibraries\n\
            Precondition: rustc 1.99.1 compiles the workspace for every analysed target with the selected features\n").as_str())
    );
    assert_eq!(outcome.status, HeisenbergExitStatus::Compliant);
}
