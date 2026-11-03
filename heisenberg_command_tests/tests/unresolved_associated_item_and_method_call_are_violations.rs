use std::str;

use heisenberg_command::heisenberg_exit_status::HeisenbergExitStatus;
use heisenberg_command_tests::heisenberg_check::HeisenbergCheck;
use heisenberg_fixtures::fixture_workspace::FixtureWorkspace;

#[test]
fn unresolved_associated_item_and_method_call_are_violations() {
    let outcome = HeisenbergCheck {
        workspace: &FixtureWorkspace {
            name: "unresolved_associated_items",
        }
        .path(),
    }
    .run()
    .expect("the arguments must parse");

    assert_eq!(
        str::from_utf8(&outcome.output),
        Ok("UnresolvedMethodCall: 2 violations\n\
            Every name must resolve for the analysis to be complete. Fix the call so its method resolves.\n\
            \n\
            - src/run.rs:5:10: Method call is unresolved\n        \
            Item.absent();\n\
            - src/run.rs:7:10: Method call is unresolved\n        \
            Item.absent().unknown();\n\
            \n\
            UnresolvedPath: 2 violations\n\
            Every name must resolve for the analysis to be complete. Fix the path so it resolves.\n\
            \n\
            - src/run.rs:4:5: Path is unresolved\n        \
            Item::missing();\n\
            - src/run.rs:6:5: Path is unresolved\n        \
            missing_function().unknown();\n\
            \n\
            ------------------------------\n\
            4 errors found\n\
            ------------------------------\n\
            Analysed on x86_64-pc-windows-msvc: 1 crate targets, 3 modules\n\
            Analysed on x86_64-unknown-linux-gnu: 1 crate targets, 3 modules\n\
            Precondition: rustc 1.99.1 compiles the workspace for every declared target\n")
    );
    assert_eq!(outcome.status, HeisenbergExitStatus::Violations);
}
