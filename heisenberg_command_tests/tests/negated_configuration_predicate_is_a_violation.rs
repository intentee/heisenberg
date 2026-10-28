use std::str;

use heisenberg_command::heisenberg_exit_status::HeisenbergExitStatus;
use heisenberg_fixtures::fixture_workspace::FixtureWorkspace;
use heisenberg_test_harness::heisenberg_check::HeisenbergCheck;

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
        Ok("heisenberg check\n\
            precondition: rustc 1.99.1 compiles the workspace for every supported target\n\
            analysed on x86_64-pc-windows-msvc: 1 crate targets, 1 modules\n\
            analysed on x86_64-unknown-linux-gnu: 1 crate targets, 1 modules\n\
            violation in negated_configuration_predicates lib: configuration predicate negates a feature, test or debug_assertions at src/lib.rs:1:1 (targets: x86_64-pc-windows-msvc, x86_64-unknown-linux-gnu)\n\
            violation in negated_configuration_predicates lib: configuration predicate negates a feature, test or debug_assertions at src/lib.rs:4:1 (targets: x86_64-pc-windows-msvc, x86_64-unknown-linux-gnu)\n\
            violation in negated_configuration_predicates lib: configuration predicate negates a feature, test or debug_assertions at src/lib.rs:9:1 (targets: x86_64-pc-windows-msvc, x86_64-unknown-linux-gnu)\n\
            violation in negated_configuration_predicates lib: configuration predicate negates a feature, test or debug_assertions at src/lib.rs:12:1 (targets: x86_64-pc-windows-msvc, x86_64-unknown-linux-gnu)\n\
            result: 0 cycles, 4 violations\n")
    );
    assert_eq!(outcome.status, HeisenbergExitStatus::Violations);
}
