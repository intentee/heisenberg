use std::str;

use heisenberg_command::heisenberg_exit_status::HeisenbergExitStatus;
use heisenberg_fixtures::fixture_workspace::FixtureWorkspace;
use heisenberg_test_harness::heisenberg_check::HeisenbergCheck;

#[test]
fn tests_package_without_test_libraries_is_a_violation() {
    let outcome = HeisenbergCheck {
        workspace: &FixtureWorkspace {
            name: "tests_package_without_test_libraries",
        }
        .path(),
    }
    .run()
    .expect("the arguments must parse");

    assert_eq!(
        str::from_utf8(&outcome.output),
        Ok("heisenberg check\n\
            precondition: rustc 1.99.1 compiles the workspace for every supported target\n\
            analysed on x86_64-pc-windows-msvc: 8 crate targets, 8 modules\n\
            analysed on x86_64-unknown-linux-gnu: 8 crate targets, 8 modules\n\
            violation: package library_tests holds only tests that need no test library; move them into the tests directory of library\n\
            result: 0 cycles, 1 violations\n")
    );
    assert_eq!(outcome.status, HeisenbergExitStatus::Violations);
}
