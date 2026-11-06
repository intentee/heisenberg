use std::str;

use heisenberg_command::heisenberg_exit_status::HeisenbergExitStatus;
use heisenberg_command_tests::fixture_workspace::FixtureWorkspace;
use heisenberg_command_tests::heisenberg_check::HeisenbergCheck;
use heisenberg_command_tests::host_footer::host_footer;

#[test]
fn reexport_is_a_violation() {
    let outcome = HeisenbergCheck {
        workspace: &FixtureWorkspace { name: "reexport" }.path(),
    }
    .run()
    .expect("the arguments must parse");

    assert_eq!(
        str::from_utf8(&outcome.output),
        Ok(["Reexport: 2 violations\n\
            Every item is reachable through exactly one path. Import the item from where it is defined instead of re-exporting it.\n\
            \n\
            - src/lib.rs:3:18: Re-export\n    \
            pub extern crate std as standard;\n\
            - src/lib.rs:5:9: Re-export\n    \
            pub use inner::Inner;\n\
            \n\
            ------------------------------\n\
            2 errors found\n\
            ------------------------------\n", host_footer(1, 2).as_str()].concat().as_str())
    );
    assert_eq!(outcome.status, HeisenbergExitStatus::Violations);
}
