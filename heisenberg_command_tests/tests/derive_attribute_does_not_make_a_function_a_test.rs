use std::str;

use heisenberg_command::heisenberg_exit_status::HeisenbergExitStatus;
use heisenberg_fixtures::fixture_workspace::FixtureWorkspace;
use heisenberg_test_harness::heisenberg_check::HeisenbergCheck;

#[test]
fn derive_attribute_does_not_make_a_function_a_test() {
    let outcome = HeisenbergCheck {
        workspace: &FixtureWorkspace {
            name: "derive_attribute_on_a_test_target_function",
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
            - tests/derived.rs:2:8: Integration test target holds an item that is neither a test, a constant nor an import\n    \
            pub fn derived_helper() {}\n\
            \n\
            ------------------------------\n\
            1 errors found\n\
            ------------------------------\n\
            Analysed on x86_64-pc-windows-msvc: 2 crate targets, 2 modules\n\
            Analysed on x86_64-unknown-linux-gnu: 2 crate targets, 2 modules\n\
            Precondition: rustc 1.99.1 compiles the workspace for every declared target\n")
    );
    assert_eq!(outcome.status, HeisenbergExitStatus::Violations);
}
