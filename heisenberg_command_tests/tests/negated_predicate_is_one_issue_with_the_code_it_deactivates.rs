use std::str;

use heisenberg_command::heisenberg_exit_status::HeisenbergExitStatus;
use heisenberg_command_tests::heisenberg_check::HeisenbergCheck;
use heisenberg_fixtures::fixture_workspace::FixtureWorkspace;

#[test]
fn negated_predicate_is_one_issue_with_the_code_it_deactivates() {
    let outcome = HeisenbergCheck {
        workspace: &FixtureWorkspace {
            name: "negated_feature_alternative",
        }
        .path(),
    }
    .run()
    .expect("the arguments must parse");

    assert_eq!(
        str::from_utf8(&outcome.output),
        Ok("NegatedConfiguration: 1 violation\n\
            Code that exists only without a feature, outside tests or outside debug builds escapes testing and makes features non-additive. Express the condition positively.\n\
            \n\
            - src/lib.rs:6:1: Configuration predicate negates a feature, test or debug_assertions\n    \
            #[cfg(not(feature = \"web_admin_panel\"))]\n      \
            code active on no declared target at src/lib.rs:7:8\n\
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
