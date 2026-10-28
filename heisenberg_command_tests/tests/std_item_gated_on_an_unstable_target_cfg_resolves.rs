use std::str;

use heisenberg_command::heisenberg_exit_status::HeisenbergExitStatus;
use heisenberg_fixtures::fixture_workspace::FixtureWorkspace;
use heisenberg_test_harness::heisenberg_check::HeisenbergCheck;

#[test]
fn std_item_gated_on_an_unstable_target_cfg_resolves() {
    let outcome = HeisenbergCheck {
        workspace: &FixtureWorkspace {
            name: "std_atomic_import",
        }
        .path(),
    }
    .run()
    .expect("the arguments must parse");

    assert_eq!(
        str::from_utf8(&outcome.output),
        Ok("heisenberg check\n\
            precondition: rustc 1.99.1 compiles the workspace for every supported target\n\
            analysed on x86_64-pc-windows-msvc: 1 crate targets, 2 modules\n\
            analysed on x86_64-unknown-linux-gnu: 1 crate targets, 2 modules\n\
            result: 0 cycles, 0 violations\n")
    );
    assert_eq!(outcome.status, HeisenbergExitStatus::Compliant);
}
