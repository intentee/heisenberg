use std::str;

use heisenberg_command::heisenberg_exit_status::HeisenbergExitStatus;
use heisenberg_fixtures::fixture_workspace::FixtureWorkspace;
use heisenberg_test_harness::heisenberg_check::HeisenbergCheck;

#[test]
fn library_with_only_inactive_items_is_empty() {
    let outcome = HeisenbergCheck {
        workspace: &FixtureWorkspace {
            name: "library_with_only_inactive_items",
        }
        .path(),
    }
    .run()
    .expect("the arguments must parse");

    assert_eq!(
        str::from_utf8(&outcome.output),
        Ok("CodeActiveOnNoTarget: 1 violation\n\
            Code active on no declared target is never built or analysed. Remove it, or make it active on a declared target.\n\
            \n\
            - library_tests/src/lib.rs:2:8: Code is active on no declared target\n    \
            pub fn never() {}\n\
            \n\
            TestsOnlyPackageWithoutTestLibraries: 1 violation\n\
            A separate tests package is justified only by test-only libraries. Move the tests into the tests directory of the package they test.\n\
            \n\
            - library_tests/Cargo.toml: Package library_tests holds only tests that need no test library; move them into the tests directory of library\n\
            \n\
            ------------------------------\n\
            2 errors found\n\
            ------------------------------\n\
            Analysed on x86_64-pc-windows-msvc: 3 crate targets, 3 modules\n\
            Analysed on x86_64-unknown-linux-gnu: 3 crate targets, 3 modules\n\
            Precondition: rustc 1.99.1 compiles the workspace for every declared target\n")
    );
    assert_eq!(outcome.status, HeisenbergExitStatus::Violations);
}
