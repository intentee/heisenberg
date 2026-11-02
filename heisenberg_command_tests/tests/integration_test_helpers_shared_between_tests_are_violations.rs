use std::str;

use heisenberg_command::heisenberg_exit_status::HeisenbergExitStatus;
use heisenberg_fixtures::fixture_workspace::FixtureWorkspace;
use heisenberg_test_harness::heisenberg_check::HeisenbergCheck;

#[test]
fn integration_test_helpers_shared_between_tests_are_violations() {
    let outcome = HeisenbergCheck {
        workspace: &FixtureWorkspace {
            name: "integration_test_helpers",
        }
        .path(),
    }
    .run()
    .expect("the arguments must parse");

    assert_eq!(
        str::from_utf8(&outcome.output),
        Ok("NonTestItemInTestTarget: 1 violation\n\
            Integration test files hold only tests. Move helpers and fixtures into a library crate.\n\
            \n\
            - tests/support/fixture_value.rs:1:12: Integration test target holds an item that is neither a test, a constant nor an import\n    \
            pub struct FixtureValue {\n      \
            impl block at tests/support/fixture_value.rs:5:6\n\
            \n\
            SharedModuleFile: 1 violation\n\
            A file included as several modules has no single owner. Give each module its own file.\n\
            \n\
            - tests/support/mod.rs: Module file belongs to 2 modules\n      \
            crate::support in integration_test_helpers test \"first_shared\"\n      \
            crate::support in integration_test_helpers test \"second_shared\"\n      \
            files of its submodules:\n        \
            tests/support/fixture_value.rs\n\
            \n\
            ------------------------------\n\
            2 errors found\n\
            ------------------------------\n\
            Analysed on x86_64-pc-windows-msvc: 5 crate targets, 9 modules\n\
            Analysed on x86_64-unknown-linux-gnu: 5 crate targets, 9 modules\n\
            Precondition: rustc 1.99.1 compiles the workspace for every declared target\n")
    );
    assert_eq!(outcome.status, HeisenbergExitStatus::Violations);
}
