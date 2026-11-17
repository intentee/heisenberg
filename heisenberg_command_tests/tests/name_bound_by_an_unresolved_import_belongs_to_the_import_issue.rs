use std::str;

use heisenberg_command::heisenberg_exit_status::HeisenbergExitStatus;
use heisenberg_command_tests::fixture_workspace::FixtureWorkspace;
use heisenberg_command_tests::heisenberg_check::HeisenbergCheck;
use heisenberg_command_tests::nextest_presence_evaluation::NextestPresenceEvaluation;

#[test]
fn name_bound_by_an_unresolved_import_belongs_to_the_import_issue() {
    let outcome = HeisenbergCheck {
        workspace: &FixtureWorkspace {
            name: "names_bound_by_an_unresolved_import",
        }
        .path(),
    }
    .run()
    .expect("the arguments must parse");

    assert_eq!(
        str::from_utf8(&outcome.output),
        Ok([
            "UnresolvedPath: 3 violations\n\
            The code compiles, so a name that rust-analyzer cannot resolve leaves a gap in the analysis. Write the path so that rust-analyzer resolves it, such as by importing the item it names.\n\
            \n\
            - src/lib.rs:1:5: Path compiles, but rust-analyzer cannot resolve it\n    \
            use crate::missing::helper;\n\
            - src/lib.rs:2:5: Path compiles, but rust-analyzer cannot resolve it\n    \
            use crate::missing::helper_rules;\n\
            - src/lib.rs:6:20: Path compiles, but rust-analyzer cannot resolve it\n    \
            fn unknown(_value: Unknown) {}\n\
            \n\
            ------------------------------\n\
            3 errors found in 1 rule\n\
            ------------------------------\n",
            NextestPresenceEvaluation::NotEvaluated.host_footer(1, 1).as_str()
        ]
        .concat()
        .as_str())
    );
    assert_eq!(outcome.status, HeisenbergExitStatus::Violations);
}
