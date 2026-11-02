use std::str;

use heisenberg_command::heisenberg_exit_status::HeisenbergExitStatus;
use heisenberg_fixtures::fixture_workspace::FixtureWorkspace;
use heisenberg_test_harness::heisenberg_check::HeisenbergCheck;

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
        Ok("UnresolvedPath: 3 violations\n\
            Every name must resolve for the analysis to be complete. Fix the path so it resolves.\n\
            \n\
            - src/lib.rs:1:5: Path is unresolved\n    \
            use crate::missing::helper;\n\
            - src/lib.rs:2:5: Path is unresolved\n    \
            use crate::missing::helper_rules;\n\
            - src/lib.rs:10:20: Path is unresolved\n    \
            fn unknown(_value: Unknown) {}\n\
            \n\
            ------------------------------\n\
            3 errors found\n\
            ------------------------------\n\
            Analysed on x86_64-pc-windows-msvc: 1 crate targets, 1 modules\n\
            Analysed on x86_64-unknown-linux-gnu: 1 crate targets, 1 modules\n\
            Precondition: rustc 1.99.1 compiles the workspace for every supported target\n")
    );
    assert_eq!(outcome.status, HeisenbergExitStatus::Violations);
}
