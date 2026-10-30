use std::str;

use heisenberg_command::heisenberg_exit_status::HeisenbergExitStatus;
use heisenberg_fixtures::fixture_workspace::FixtureWorkspace;
use heisenberg_test_harness::heisenberg_check::HeisenbergCheck;

#[test]
fn package_cycle_through_dev_dependencies_is_a_violation() {
    let outcome = HeisenbergCheck {
        workspace: &FixtureWorkspace {
            name: "dev_dependency_cycle",
        }
        .path(),
    }
    .run()
    .expect("the arguments must parse");

    assert_eq!(
        str::from_utf8(&outcome.output),
        Ok("heisenberg check\n\
            precondition: rustc 1.99.1 compiles the workspace for every supported target\n\
            analysed on x86_64-pc-windows-msvc: 2 crate targets, 2 modules\n\
            analysed on x86_64-unknown-linux-gnu: 2 crate targets, 2 modules\n\
            violation: packages alpha, beta depend on each other through dev-dependencies\n\
            result: 0 cycles, 1 violations\n")
    );
    assert_eq!(outcome.status, HeisenbergExitStatus::Violations);
}
