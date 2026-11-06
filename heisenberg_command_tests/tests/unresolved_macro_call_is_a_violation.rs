use std::str;

use heisenberg_command::heisenberg_exit_status::HeisenbergExitStatus;
use heisenberg_command_tests::fixture_workspace::FixtureWorkspace;
use heisenberg_command_tests::heisenberg_check::HeisenbergCheck;
use heisenberg_command_tests::host_footer::host_footer;

#[test]
fn unresolved_macro_call_is_a_violation() {
    let outcome = HeisenbergCheck {
        workspace: &FixtureWorkspace {
            name: "unresolved_macro_call",
        }
        .path(),
    }
    .run()
    .expect("the arguments must parse");

    assert_eq!(
        str::from_utf8(&outcome.output),
        Ok([
            "UnresolvedMacroCall: 5 violations\n\
            Every name must resolve for the analysis to be complete. Define or import the macro.\n\
            \n\
            - src/lib.rs:1:1: Macro call is unresolved\n    \
            Marker!();\n\
            - src/lib.rs:3:8: Macro call is unresolved\n    \
            crate::missing!();\n\
            - src/lib.rs:4:16: Macro call is unresolved\n    \
            crate::absent::missing!();\n\
            - src/lib.rs:6:3: Macro call is unresolved\n    \
            #[missing_attribute]\n\
            - src/lib.rs:9:10: Macro call is unresolved\n    \
            #[derive(MissingDerive)]\n\
            \n\
            ------------------------------\n\
            5 errors found\n\
            ------------------------------\n",
            host_footer(1, 1).as_str()
        ]
        .concat()
        .as_str())
    );
    assert_eq!(outcome.status, HeisenbergExitStatus::Violations);
}
