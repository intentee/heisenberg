use std::str;

use heisenberg_command::heisenberg_exit_status::HeisenbergExitStatus;
use heisenberg_command_tests::fixture_workspace::FixtureWorkspace;
use heisenberg_command_tests::heisenberg_check::HeisenbergCheck;
use heisenberg_command_tests::host_footer::host_footer;

#[test]
fn unresolved_paths_into_a_foreign_module_are_one_issue_per_module() {
    let outcome = HeisenbergCheck {
        workspace: &FixtureWorkspace {
            name: "references_into_a_foreign_module_that_does_not_resolve",
        }
        .path(),
    }
    .run()
    .expect("the arguments must parse");

    assert_eq!(
        str::from_utf8(&outcome.output),
        Ok([
            "UnresolvedPath: 1 violation\n\
            Every name must resolve for the analysis to be complete. Fix the path so it resolves.\n\
            \n\
            - src/lib.rs:2:18: Path into outside is unresolved\n        \
            pub binding: outside::Binding,\n\
            \n\
            ------------------------------\n\
            1 errors found\n\
            ------------------------------\n",
            host_footer(1, 1).as_str()
        ]
        .concat()
        .as_str())
    );
    assert_eq!(outcome.status, HeisenbergExitStatus::Violations);
}
