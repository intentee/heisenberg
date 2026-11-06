use std::str;

use heisenberg_command::heisenberg_exit_status::HeisenbergExitStatus;
use heisenberg_command_tests::fixture_workspace::FixtureWorkspace;
use heisenberg_command_tests::heisenberg_check::HeisenbergCheck;
use heisenberg_command_tests::host_footer::host_footer;

#[test]
fn unresolved_import_takes_no_place_in_the_item_order() {
    let outcome = HeisenbergCheck {
        workspace: &FixtureWorkspace {
            name: "unresolved_import_after_an_item",
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
            - src/lib.rs:3:5: Path is unresolved\n    \
            use missing::Thing;\n\
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
