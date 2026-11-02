use std::str;

use heisenberg_command::heisenberg_exit_status::HeisenbergExitStatus;
use heisenberg_fixtures::fixture_workspace::FixtureWorkspace;
use heisenberg_test_harness::heisenberg_check::HeisenbergCheck;

#[test]
fn reexport_is_a_violation() {
    let outcome = HeisenbergCheck {
        workspace: &FixtureWorkspace { name: "reexport" }.path(),
    }
    .run()
    .expect("the arguments must parse");

    assert_eq!(
        str::from_utf8(&outcome.output),
        Ok("ERROR: Reexport - src/lib.rs:3:9 - Re-export\n\
            pub use inner::Inner;\n\
            \n\
            ERROR: Reexport - src/lib.rs:5:18 - Re-export\n\
            pub extern crate std as standard;\n\
            \n\
            ------------------------------\n\
            2 errors found\n\
            ------------------------------\n\
            Analysed on x86_64-pc-windows-msvc: 1 crate targets, 2 modules\n\
            Analysed on x86_64-unknown-linux-gnu: 1 crate targets, 2 modules\n\
            Precondition: rustc 1.99.1 compiles the workspace for every supported target\n")
    );
    assert_eq!(outcome.status, HeisenbergExitStatus::Violations);
}
