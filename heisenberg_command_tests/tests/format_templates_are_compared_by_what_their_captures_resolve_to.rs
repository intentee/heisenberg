use std::str;

use heisenberg_command::heisenberg_exit_status::HeisenbergExitStatus;
use heisenberg_command_tests::fixture_workspace::FixtureWorkspace;
use heisenberg_command_tests::heisenberg_check::HeisenbergCheck;
use heisenberg_command_tests::nextest_presence_evaluation::NextestPresenceEvaluation;

#[test]
fn format_templates_are_compared_by_what_their_captures_resolve_to() {
    let outcome = HeisenbergCheck {
        workspace: &FixtureWorkspace {
            name: "format_capture_values",
        }
        .path(),
    }
    .run()
    .expect("the arguments must parse");

    assert_eq!(
        str::from_utf8(&outcome.output),
        Ok([
            "DuplicatedFunctionBody: 2 violations\n\
            Duplicated code drifts apart when one copy changes. Keep one function and call it from every place that needs it.\n\
            \n\
            - src/eighth.rs:1:8: Function body is duplicated\n    \
            pub fn eighth() -> String {\n      \
            duplicate at src/seventh.rs:1:8\n\
            - src/fifth.rs:3:8: Function body is duplicated\n    \
            pub fn fifth() -> String {\n      \
            duplicate at src/sixth.rs:3:8\n\
            \n\
            UnresolvedPath: 2 violations\n\
            The code compiles, so a name that rust-analyzer cannot resolve leaves a gap in the analysis. Write the path so that rust-analyzer resolves it, such as by importing the item it names.\n\
            \n\
            - src/fourth.rs:2:13: Path compiles, but rust-analyzer cannot resolve it\n        \
            format!(\"{missing}\")\n\
            - src/third.rs:2:13: Path compiles, but rust-analyzer cannot resolve it\n        \
            format!(\"{missing}\")\n\
            \n\
            ------------------------------\n\
            4 errors found in 2 rules\n\
            ------------------------------\n",
            NextestPresenceEvaluation::NotEvaluated.host_footer(1, 10).as_str()
        ]
        .concat()
        .as_str())
    );
    assert_eq!(outcome.status, HeisenbergExitStatus::Violations);
}
