use std::str;

use heisenberg_command::heisenberg_exit_status::HeisenbergExitStatus;
use heisenberg_command_tests::heisenberg_check::HeisenbergCheck;
use heisenberg_fixtures::fixture_workspace::FixtureWorkspace;

#[test]
fn path_into_a_build_script_shaped_dependency_module_is_unresolved() {
    let outcome = HeisenbergCheck {
        workspace: &FixtureWorkspace {
            name: "foreign_prefix_path_into_a_build_script_shaped_dependency",
        }
        .path(),
    }
    .run()
    .expect("the arguments must parse");

    assert_eq!(
        str::from_utf8(&outcome.output),
        Ok("UnresolvedPath: 1 violation\n\
            Every name must resolve for the analysis to be complete. Fix the path so it resolves.\n\
            \n\
            - src/lib.rs:1:23: Path into shaped::backend is unresolved\n    \
            pub fn generated() -> shaped::backend::Generated {\n      \
            also unresolved at src/lib.rs:2:5\n\
            \n\
            ------------------------------\n\
            1 errors found\n\
            ------------------------------\n\
            Analysed on x86_64-pc-windows-msvc: 1 crate targets, 1 modules\n\
            Analysed on x86_64-unknown-linux-gnu: 1 crate targets, 1 modules\n\
            Precondition: rustc 1.99.1 compiles the workspace for every declared target\n")
    );
    assert_eq!(outcome.status, HeisenbergExitStatus::Violations);
}
