use std::str;

use heisenberg_command::heisenberg_exit_status::HeisenbergExitStatus;
use heisenberg_command_tests::fixture_workspace::FixtureWorkspace;
use heisenberg_command_tests::heisenberg_check::HeisenbergCheck;
use heisenberg_command_tests::nextest_presence_evaluation::NextestPresenceEvaluation;

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
        Ok(["UnresolvedMethodCall: 2 violations\n\
            The code compiles, so a name that rust-analyzer cannot resolve leaves a gap in the analysis. Write the call so that rust-analyzer resolves its method, such as by naming the type of its receiver.\n\
            \n\
            - src/run.rs:5:10: Method call compiles, but rust-analyzer cannot resolve its method\n        \
            Item.absent();\n\
            - src/run.rs:7:10: Method call compiles, but rust-analyzer cannot resolve its method\n        \
            Item.absent().unknown();\n\
            \n\
            UnresolvedPath: 2 violations\n\
            The code compiles, so a name that rust-analyzer cannot resolve leaves a gap in the analysis. Write the path so that rust-analyzer resolves it, such as by importing the item it names.\n\
            \n\
            - src/run.rs:4:5: Path compiles, but rust-analyzer cannot resolve it\n        \
            Item::missing();\n\
            - src/run.rs:6:5: Path compiles, but rust-analyzer cannot resolve it\n        \
            missing_function().unknown();\n\
            \n\
            ------------------------------\n\
            4 errors found\n\
            ------------------------------\n", NextestPresenceEvaluation::NotEvaluated.host_footer(1, 3).as_str()].concat().as_str())
    );
    assert_eq!(outcome.status, HeisenbergExitStatus::Violations);
}
