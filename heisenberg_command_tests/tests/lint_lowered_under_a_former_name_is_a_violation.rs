use std::str;

use heisenberg_command::heisenberg_exit_status::HeisenbergExitStatus;
use heisenberg_command_tests::heisenberg_check::HeisenbergCheck;
use heisenberg_fixtures::fixture_workspace::FixtureWorkspace;

#[test]
fn lint_lowered_under_a_former_name_is_a_violation() {
    let outcome = HeisenbergCheck {
        workspace: &FixtureWorkspace {
            name: "lint_lowered_under_a_former_name",
        }
        .path(),
    }
    .run()
    .expect("the arguments must parse");

    assert_eq!(
        str::from_utf8(&outcome.output),
        Ok("LintSuppression: 1 violation\n\
            Suppressing lints is a slippery slope. Remove the attribute and fix the code the lint reports.\n\
            \n\
            - src/lib.rs:1:3: Attribute suppresses lints\n    \
            #[warn(clippy::result_expect_used)]\n\
            \n\
            ------------------------------\n\
            1 errors found\n\
            ------------------------------\n\
            Analysed on x86_64-pc-windows-msvc: 1 crate targets, 1 modules\n\
            Analysed on x86_64-unknown-linux-gnu: 1 crate targets, 1 modules\n\
            Precondition: rustc 1.99.1 compiles the workspace for every declared target\n")
    );
    assert_eq!(outcome.status, HeisenbergExitStatus::Violations);
}
