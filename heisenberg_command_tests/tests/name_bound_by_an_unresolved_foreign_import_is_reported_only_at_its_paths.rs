use std::str;

use heisenberg_command::heisenberg_exit_status::HeisenbergExitStatus;
use heisenberg_command_tests::fixture_workspace::FixtureWorkspace;
use heisenberg_command_tests::heisenberg_check::HeisenbergCheck;
use heisenberg_command_tests::nextest_presence_evaluation::NextestPresenceEvaluation;

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
        Ok(["ImportInsideBlock: 1 violation\n\
            The imports of a file are read in one place, at its top, and a name means the same thing throughout the file. Move the import to the top of the module.\n\
            \n\
            - src/lib.rs:12:9: Import is written inside a block\n            \
            use shaped::backend::Generated as Local;\n\
            \n\
            UnresolvedPath: 1 violation\n\
            The code compiles, so a name that rust-analyzer cannot resolve leaves a gap in the analysis. Write the path so that rust-analyzer resolves it, such as by importing the item it names.\n\
            \n\
            - src/lib.rs:1:5: Path into shaped::backend compiles, but rust-analyzer cannot resolve it\n    \
            use shaped::backend::Generated;\n      \
            also unresolved at src/lib.rs:2:5\n      \
            also unresolved at src/lib.rs:12:13\n      \
            also unresolved at src/lib.rs:23:9\n\
            \n\
            ------------------------------\n\
            2 errors found in 2 rules\n\
            ------------------------------\n",
            NextestPresenceEvaluation::NotEvaluated.host_footer(1, 2).as_str()
        ]
        .concat()
        .as_str())
    );
    assert_eq!(outcome.status, HeisenbergExitStatus::Violations);
}
