use std::str;

use heisenberg_command::heisenberg_exit_status::HeisenbergExitStatus;
use heisenberg_command_tests::heisenberg_check::HeisenbergCheck;
use heisenberg_fixtures::fixture_workspace::FixtureWorkspace;

#[test]
fn suppression_in_a_workspace_member_macro_is_a_violation() {
    let outcome = HeisenbergCheck {
        workspace: &FixtureWorkspace {
            name: "suppression_in_a_workspace_member_macro",
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
            - user/src/lib.rs:1:1: Attribute suppresses lints\n    \
            quiet::quiet! {}\n\
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
