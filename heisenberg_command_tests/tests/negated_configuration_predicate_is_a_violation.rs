use std::str;

use heisenberg_command::heisenberg_exit_status::HeisenbergExitStatus;
use heisenberg_command_tests::heisenberg_check::HeisenbergCheck;
use heisenberg_fixtures::fixture_workspace::FixtureWorkspace;

#[test]
fn negated_configuration_predicate_is_a_violation() {
    let outcome = HeisenbergCheck {
        workspace: &FixtureWorkspace {
            name: "negated_configuration_predicates",
        }
        .path(),
    }
    .run()
    .expect("the arguments must parse");

    assert_eq!(
        str::from_utf8(&outcome.output),
        Ok("NegatedConfiguration: 5 violations\n\
            Code that exists only without a feature, outside tests or outside debug builds escapes testing and makes features non-additive. Express the condition positively.\n\
            \n\
            - src/lib.rs:1:1: Configuration predicate negates a feature, test or debug_assertions\n    \
            #![cfg_attr(not(feature = \"optional\"), allow(dead_code))]\n\
            - src/lib.rs:6:5: Configuration predicate negates a feature, test or debug_assertions\n        \
            #[cfg(any(windows, not(feature = \"optional\")))]\n\
            - src/lib.rs:9:5: Configuration predicate negates a feature, test or debug_assertions\n        \
            #[cfg_attr(not(test), must_use)]\n\
            - src/lib.rs:14:5: Configuration predicate negates a feature, test or debug_assertions\n        \
            #[cfg(any(windows, all(unix, not(debug_assertions))))]\n\
            - src/lib.rs:17:5: Configuration predicate negates a feature, test or debug_assertions\n        \
            #[cfg_attr(unix, cfg(not(feature = \"optional\")))]\n\
            \n\
            ------------------------------\n\
            5 errors found\n\
            ------------------------------\n\
            Analysed on x86_64-pc-windows-msvc: 1 crate targets, 1 modules\n\
            Analysed on x86_64-unknown-linux-gnu: 1 crate targets, 1 modules\n\
            Precondition: rustc 1.99.1 compiles the workspace for every declared target\n")
    );
    assert_eq!(outcome.status, HeisenbergExitStatus::Violations);
}
