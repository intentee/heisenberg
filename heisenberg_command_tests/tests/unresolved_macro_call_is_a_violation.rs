use std::str;

use heisenberg_command::heisenberg_exit_status::HeisenbergExitStatus;
use heisenberg_command_tests::fixture_workspace::FixtureWorkspace;
use heisenberg_command_tests::heisenberg_check::HeisenbergCheck;
use heisenberg_command_tests::nextest_presence_evaluation::NextestPresenceEvaluation;

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
            The code compiles, so a name that rust-analyzer cannot resolve leaves a gap in the analysis. Define or import the macro so that rust-analyzer resolves it.\n\
            \n\
            - src/lib.rs:1:1: Macro call compiles, but rust-analyzer cannot resolve its macro\n    \
            Marker!();\n\
            - src/lib.rs:3:8: Macro call compiles, but rust-analyzer cannot resolve its macro\n    \
            crate::missing!();\n\
            - src/lib.rs:4:16: Macro call compiles, but rust-analyzer cannot resolve its macro\n    \
            crate::absent::missing!();\n\
            - src/lib.rs:6:3: Macro call compiles, but rust-analyzer cannot resolve its macro\n    \
            #[missing_attribute]\n\
            - src/lib.rs:9:10: Macro call compiles, but rust-analyzer cannot resolve its macro\n    \
            #[derive(MissingDerive)]\n\
            \n\
            ------------------------------\n\
            5 errors found\n\
            ------------------------------\n",
            NextestPresenceEvaluation::NotEvaluated.host_footer(1, 1).as_str()
        ]
        .concat()
        .as_str())
    );
    assert_eq!(outcome.status, HeisenbergExitStatus::Violations);
}
