use std::str;

use heisenberg_command::heisenberg_exit_status::HeisenbergExitStatus;
use heisenberg_command_tests::fixture_workspace::FixtureWorkspace;
use heisenberg_command_tests::heisenberg_check::HeisenbergCheck;
use heisenberg_command_tests::host_footer::host_footer;

#[test]
fn field_shorthand_is_compared_by_the_value_it_reads() {
    let outcome = HeisenbergCheck {
        workspace: &FixtureWorkspace {
            name: "field_shorthand_values",
        }
        .path(),
    }
    .run()
    .expect("the arguments must parse");

    assert_eq!(
        str::from_utf8(&outcome.output),
        Ok([
            "DuplicatedFunctionBody: 1 violation\n\
            Duplicated code drifts apart when one copy changes. Keep one function and call it from every place that needs it.\n\
            \n\
            - src/fifth.rs:4:14: Function body is duplicated\n    \
            pub const fn fifth() -> Declaration {\n      \
            duplicate at src/sixth.rs:4:14\n\
            \n\
            UnresolvedPath: 2 violations\n\
            The code compiles, so a name that rust-analyzer cannot resolve leaves a gap in the analysis. Write the path so that rust-analyzer resolves it, such as by importing the item it names.\n\
            \n\
            - src/eighth.rs:4:19: Path compiles, but rust-analyzer cannot resolve it\n        \
            Declaration { command }\n\
            - src/seventh.rs:4:19: Path compiles, but rust-analyzer cannot resolve it\n        \
            Declaration { command }\n\
            \n\
            ------------------------------\n\
            3 errors found\n\
            ------------------------------\n",
            host_footer(1, 9).as_str()
        ]
        .concat()
        .as_str())
    );
    assert_eq!(outcome.status, HeisenbergExitStatus::Violations);
}
