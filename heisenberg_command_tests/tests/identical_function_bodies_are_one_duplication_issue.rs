use std::str;

use heisenberg_command::heisenberg_exit_status::HeisenbergExitStatus;
use heisenberg_fixtures::fixture_workspace::FixtureWorkspace;
use heisenberg_test_harness::heisenberg_check::HeisenbergCheck;

#[test]
fn identical_function_bodies_are_one_duplication_issue() {
    let outcome = HeisenbergCheck {
        workspace: &FixtureWorkspace {
            name: "identical_function_bodies",
        }
        .path(),
    }
    .run()
    .expect("the arguments must parse");

    assert_eq!(
        str::from_utf8(&outcome.output),
        Ok("DuplicatedFunctionBody: 1 violation\n\
            Duplicated code drifts apart when one copy changes. Keep one function and call it from every place that needs it.\n\
            \n\
            - src/first.rs:1:8: Function body is duplicated\n    \
            pub fn total_length(names: &[String]) -> usize {\n      \
            duplicate at src/second.rs:1:8\n\
            \n\
            ------------------------------\n\
            1 errors found\n\
            ------------------------------\n\
            Analysed on x86_64-pc-windows-msvc: 1 crate targets, 3 modules\n\
            Analysed on x86_64-unknown-linux-gnu: 1 crate targets, 3 modules\n\
            Precondition: rustc 1.99.1 compiles the workspace for every declared target\n")
    );
    assert_eq!(outcome.status, HeisenbergExitStatus::Violations);
}
