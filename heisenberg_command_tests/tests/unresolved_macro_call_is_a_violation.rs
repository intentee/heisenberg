use std::str;

use heisenberg_command::heisenberg_exit_status::HeisenbergExitStatus;
use heisenberg_fixtures::fixture_workspace::FixtureWorkspace;
use heisenberg_test_harness::heisenberg_check::HeisenbergCheck;

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
        Ok("UnresolvedMacroCall: 5 violations\n\
            Every name must resolve for the analysis to be complete. Define or import the macro.\n\
            \n\
            - src/lib.rs:3:1: Macro call is unresolved\n    \
            Marker!();\n\
            - src/lib.rs:5:8: Macro call is unresolved\n    \
            crate::missing!();\n\
            - src/lib.rs:6:16: Macro call is unresolved\n    \
            crate::absent::missing!();\n\
            - src/lib.rs:8:10: Macro call is unresolved\n    \
            #[derive(MissingDerive)]\n\
            - src/lib.rs:11:3: Macro call is unresolved\n    \
            #[missing_attribute]\n\
            \n\
            ------------------------------\n\
            5 errors found\n\
            ------------------------------\n\
            Analysed on x86_64-pc-windows-msvc: 1 crate targets, 1 modules\n\
            Analysed on x86_64-unknown-linux-gnu: 1 crate targets, 1 modules\n\
            Precondition: rustc 1.99.1 compiles the workspace for every supported target\n")
    );
    assert_eq!(outcome.status, HeisenbergExitStatus::Violations);
}
