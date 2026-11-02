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
        Ok("Reexport: 2 violations\n\
            Every item is reachable through exactly one path. Import the item from where it is defined instead of re-exporting it.\n\
            \n\
            - src/lib.rs:3:9: Re-export\n    \
            pub use inner::Inner;\n\
            - src/lib.rs:5:18: Re-export\n    \
            pub extern crate std as standard;\n\
            \n\
            ------------------------------\n\
            2 errors found\n\
            ------------------------------\n\
            Analysed on x86_64-pc-windows-msvc: 1 crate targets, 2 modules\n\
            Analysed on x86_64-unknown-linux-gnu: 1 crate targets, 2 modules\n\
            Precondition: rustc 1.99.1 compiles the workspace for every declared target\n")
    );
    assert_eq!(outcome.status, HeisenbergExitStatus::Violations);
}
