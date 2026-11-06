use std::str;

use heisenberg_command::heisenberg_exit_status::HeisenbergExitStatus;
use heisenberg_command_tests::fixture_workspace::FixtureWorkspace;
use heisenberg_command_tests::heisenberg_check::HeisenbergCheck;
use heisenberg_command_tests::host_footer::host_footer;

#[test]
fn name_bound_by_an_unresolved_foreign_import_is_reported_only_at_its_paths() {
    let outcome = HeisenbergCheck {
        workspace: &FixtureWorkspace {
            name: "explicit_import_of_a_build_script_shaped_item",
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
            - src/lib.rs:1:5: Path into shaped::backend is unresolved\n    \
            use shaped::backend::Generated;\n      \
            also unresolved at src/lib.rs:2:5\n      \
            also unresolved at src/lib.rs:12:13\n      \
            also unresolved at src/lib.rs:23:9\n\
            \n\
            ------------------------------\n\
            1 errors found\n\
            ------------------------------\n",
            host_footer(1, 2).as_str()
        ]
        .concat()
        .as_str())
    );
    assert_eq!(outcome.status, HeisenbergExitStatus::Violations);
}
