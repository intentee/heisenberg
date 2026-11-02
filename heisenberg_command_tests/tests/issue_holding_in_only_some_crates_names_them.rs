use std::str;

use heisenberg_command::heisenberg_exit_status::HeisenbergExitStatus;
use heisenberg_fixtures::fixture_workspace::FixtureWorkspace;
use heisenberg_test_harness::heisenberg_check::HeisenbergCheck;

#[test]
fn issue_holding_in_only_some_crates_names_them() {
    let outcome = HeisenbergCheck {
        workspace: &FixtureWorkspace {
            name: "issue_narrowed_to_some_crates",
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
            - tests/support/mod.rs:1:8: Integration test target holds an item that is neither a test, a constant nor an import (only in issue_narrowed_to_some_crates test \"first\"; issue_narrowed_to_some_crates test \"second\")\n    \
            pub fn helper() {}\n\
            \n\
            SharedModuleFile: 1 violation\n\
            A file included as several modules has no single owner. Give each module its own file.\n\
            \n\
            - tests/support/mod.rs: Module file belongs to 3 modules\n      \
            crate::support in issue_narrowed_to_some_crates example \"demo\"\n      \
            crate::support in issue_narrowed_to_some_crates test \"first\"\n      \
            crate::support in issue_narrowed_to_some_crates test \"second\"\n\
            \n\
            ------------------------------\n\
            2 errors found\n\
            ------------------------------\n\
            Analysed on x86_64-pc-windows-msvc: 4 crate targets, 7 modules\n\
            Analysed on x86_64-unknown-linux-gnu: 4 crate targets, 7 modules\n\
            Precondition: rustc 1.99.1 compiles the workspace for every supported target\n")
    );
    assert_eq!(outcome.status, HeisenbergExitStatus::Violations);
}
