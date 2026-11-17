use std::str;

use heisenberg_command::heisenberg_exit_status::HeisenbergExitStatus;
use heisenberg_command_tests::fixture_workspace::FixtureWorkspace;
use heisenberg_command_tests::heisenberg_check::HeisenbergCheck;
use heisenberg_command_tests::nextest_presence_evaluation::NextestPresenceEvaluation;

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
        Ok(["PathPackageOutsideWorkspace: 1 violation\n\
            Only workspace members are analysed. A package built from local sources that is not a member is code the project owns and compiles, yet no rule reads it. Make the package a member of the workspace, or depend on a release of it from a registry or on a git revision.\n\
            \n\
            - Cargo.toml: Package shaped v0.1.0 is built from shaped/Cargo.toml, which is not a workspace member, so its code is not analysed\n\
            \n\
            UnresolvedPath: 1 violation\n\
            The code compiles, so a name that rust-analyzer cannot resolve leaves a gap in the analysis. Write the path so that rust-analyzer resolves it, such as by importing the item it names.\n\
            \n\
            - src/lib.rs:1:23: Path into shaped::backend compiles, but rust-analyzer cannot resolve it\n    \
            pub fn generated() -> shaped::backend::Generated {\n      \
            also unresolved at src/lib.rs:2:5\n\
            \n\
            ------------------------------\n\
            2 errors found in 2 rules\n\
            ------------------------------\n",
            NextestPresenceEvaluation::NotEvaluated.host_footer(1, 1).as_str()
        ]
        .concat()
        .as_str())
    );
    assert_eq!(outcome.status, HeisenbergExitStatus::Violations);
}
