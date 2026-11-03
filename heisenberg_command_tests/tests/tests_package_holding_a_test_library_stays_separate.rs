use std::str;

use heisenberg_command::heisenberg_exit_status::HeisenbergExitStatus;
use heisenberg_command_tests::fixture_workspace::FixtureWorkspace;
use heisenberg_command_tests::heisenberg_check::HeisenbergCheck;

#[test]
fn tests_package_holding_a_test_library_stays_separate() {
    let outcome = HeisenbergCheck {
        workspace: &FixtureWorkspace {
            name: "tests_package_holding_a_test_library",
        }
        .path(),
    }
    .run()
    .expect("the arguments must parse");

    assert_eq!(
        str::from_utf8(&outcome.output),
        Ok("NonTestItemInTestTarget: 1 violation\n\
            A helper used beyond its own test file is a test library, and test libraries never live in a tests directory. Move it into a library crate; when that crate depends on the package under test, move the tests that use it there too, so package dependencies never form a cycle.\n\
            \n\
            - library_tests/tests/support/mod.rs:3:8: Integration test target holds an item that is neither a test, a constant, an import nor a private helper used only in its own file\n    \
            pub fn support() -> Library {\n\
            \n\
            ------------------------------\n\
            1 errors found\n\
            ------------------------------\n\
            Analysed on x86_64-pc-windows-msvc: 3 crate targets, 4 modules\n\
            Analysed on x86_64-unknown-linux-gnu: 3 crate targets, 4 modules\n\
            Precondition: rustc 1.99.1 compiles the workspace for every declared target\n")
    );
    assert_eq!(outcome.status, HeisenbergExitStatus::Violations);
}
