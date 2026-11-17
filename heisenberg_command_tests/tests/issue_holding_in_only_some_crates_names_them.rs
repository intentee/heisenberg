use std::str;

use heisenberg_command::heisenberg_exit_status::HeisenbergExitStatus;
use heisenberg_command_tests::fixture_workspace::FixtureWorkspace;
use heisenberg_command_tests::heisenberg_check::HeisenbergCheck;
use heisenberg_command_tests::nextest_presence_evaluation::NextestPresenceEvaluation;

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
        Ok([
            "NonTestItemInTestTarget: 1 violation\n\
            A helper used beyond its own test file is a test library, and test libraries never live in a tests directory. Move it into a library crate; when that crate depends on the package under test, move the tests that use it there too, so package dependencies never form a cycle.\n\
            \n\
            - tests/support/mod.rs:1:8: Integration test target holds an item that is neither a test, a constant, an import nor a private helper used only in its own file (only in issue_narrowed_to_some_crates test \"first\"; issue_narrowed_to_some_crates test \"second\")\n    \
            pub fn support() {}\n\
            \n\
            SharedModuleFile: 1 violation\n\
            A file included as several modules has no single owner. Give each module its own file.\n\
            \n\
            - tests/support/mod.rs: Module file belongs to 3 modules\n      \
            crate::support in issue_narrowed_to_some_crates example \"demo\"\n      \
            crate::support in issue_narrowed_to_some_crates test \"first\"\n      \
            crate::support in issue_narrowed_to_some_crates test \"second\"\n\
            \n\
            SplitIntegrationTests: 1 violation\n\
            Every integration test target is its own binary, linked and started separately, which slows every test run and multiplies the long paths of build artifacts, and automatic discovery builds every file in tests/ as a target, including files that are meant to be modules. Set autotests = false, declare one [[test]] target, and include the other test files as its modules.\n\
            \n\
            - Cargo.toml: Package builds 2 integration test targets with libtest's harness instead of one\n      \
            test \"first\"\n      \
            test \"second\"\n\
            \n\
            ------------------------------\n\
            3 errors found in 3 rules\n\
            ------------------------------\n",
            NextestPresenceEvaluation::Evaluated.host_footer(4, 7).as_str()
        ]
        .concat()
        .as_str())
    );
    assert_eq!(outcome.status, HeisenbergExitStatus::Violations);
}
