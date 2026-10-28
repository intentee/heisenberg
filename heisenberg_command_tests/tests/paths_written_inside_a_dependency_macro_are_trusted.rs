use std::str;

use heisenberg_command::heisenberg_exit_status::HeisenbergExitStatus;
use heisenberg_fixtures::fixture_workspace::FixtureWorkspace;
use heisenberg_test_harness::heisenberg_check::HeisenbergCheck;

#[test]
fn paths_written_inside_a_dependency_macro_are_trusted() {
    let outcome = HeisenbergCheck {
        workspace: &FixtureWorkspace {
            name: "dependency_macro_with_unresolvable_internals",
        }
        .path(),
    }
    .run()
    .expect("the arguments must parse");

    assert_eq!(
        str::from_utf8(&outcome.output),
        Ok("heisenberg check\n\
            precondition: rustc 1.99.1 compiles the workspace for every supported target\n\
            analysed on x86_64-pc-windows-msvc: 3 crate targets, 3 modules\n\
            analysed on x86_64-unknown-linux-gnu: 3 crate targets, 3 modules\n\
            result: 0 cycles, 0 violations\n")
    );
    assert_eq!(outcome.status, HeisenbergExitStatus::Compliant);
}
