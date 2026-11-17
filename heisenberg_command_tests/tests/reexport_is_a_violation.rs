use std::str;

use heisenberg_command::heisenberg_exit_status::HeisenbergExitStatus;
use heisenberg_command_tests::fixture_workspace::FixtureWorkspace;
use heisenberg_command_tests::heisenberg_check::HeisenbergCheck;
use heisenberg_command_tests::nextest_presence_evaluation::NextestPresenceEvaluation;

#[test]
fn reexport_is_a_violation() {
    let outcome = HeisenbergCheck {
        workspace: &FixtureWorkspace { name: "reexport" }.path(),
    }
    .run()
    .expect("the arguments must parse");

    assert_eq!(
        str::from_utf8(&outcome.output),
        Ok(["Reexport: 3 violations\n\
            Every item is reachable through exactly one path. Import the item from where it is defined instead of re-exporting it.\n\
            \n\
            - src/exported.rs:8:9: Re-export\n    \
            pub use exported;\n\
            - src/lib.rs:5:18: Re-export\n    \
            pub extern crate std as standard;\n\
            - src/lib.rs:7:9: Re-export\n    \
            pub use inner::Inner;\n\
            \n\
            ------------------------------\n\
            3 errors found in 1 rule\n\
            ------------------------------\n", NextestPresenceEvaluation::NotEvaluated.host_footer(1, 4).as_str()].concat().as_str())
    );
    assert_eq!(outcome.status, HeisenbergExitStatus::Violations);
}
