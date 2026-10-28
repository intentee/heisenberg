use std::str;

use heisenberg_command::heisenberg_exit_status::HeisenbergExitStatus;
use heisenberg_fixtures::fixture_workspace::FixtureWorkspace;
use heisenberg_test_harness::heisenberg_check::HeisenbergCheck;

#[test]
fn path_with_a_foreign_prefix_is_not_ambiguous() {
    let outcome = HeisenbergCheck {
        workspace: &FixtureWorkspace {
            name: "foreign_prefix_path_into_a_build_script_shaped_dependency",
        }
        .path(),
    }
    .run()
    .expect("the arguments must parse");

    assert_eq!(
        str::from_utf8(&outcome.output),
        Ok("heisenberg check\n\
            precondition: rustc 1.99.1 compiles the workspace for every supported target\n\
            analysed on x86_64-pc-windows-msvc: 3 crate targets, 4 modules\n\
            analysed on x86_64-unknown-linux-gnu: 3 crate targets, 4 modules\n\
            result: 0 cycles, 0 violations\n")
    );
    assert_eq!(outcome.status, HeisenbergExitStatus::Compliant);
}
