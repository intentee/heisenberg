use std::str;

use heisenberg_command::heisenberg_exit_status::HeisenbergExitStatus;
use heisenberg_fixtures::fixture_workspace::FixtureWorkspace;
use heisenberg_test_harness::heisenberg_check::HeisenbergCheck;

#[test]
fn private_items_of_a_test_file_are_accepted_only_when_only_that_file_uses_them() {
    let outcome = HeisenbergCheck {
        workspace: &FixtureWorkspace {
            name: "file_local_test_helpers",
        }
        .path(),
    }
    .run()
    .expect("the arguments must parse");

    assert_eq!(
        str::from_utf8(&outcome.output),
        Ok("NonTestItemInTestTarget: 2 violations\n\
            Integration test files hold only tests. Move helpers and fixtures into a library crate.\n\
            \n\
            - tests/helper_used_by_a_submodule.rs:3:8: Integration test target holds an item that is neither a test, a constant nor an import\n    \
            struct Shared;\n\
            - tests/unused_helper.rs:1:4: Integration test target holds an item that is neither a test, a constant nor an import\n    \
            fn unused() {}\n\
            \n\
            ------------------------------\n\
            2 errors found\n\
            ------------------------------\n\
            Analysed on x86_64-pc-windows-msvc: 4 crate targets, 5 modules\n\
            Analysed on x86_64-unknown-linux-gnu: 4 crate targets, 5 modules\n\
            Precondition: rustc 1.99.1 compiles the workspace for every declared target\n")
    );
    assert_eq!(outcome.status, HeisenbergExitStatus::Violations);
}
