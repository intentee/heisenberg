use std::str;

use heisenberg_command::heisenberg_exit_status::HeisenbergExitStatus;
use heisenberg_command_tests::fixture_workspace::FixtureWorkspace;
use heisenberg_command_tests::heisenberg_check::HeisenbergCheck;
use heisenberg_command_tests::nextest_presence_evaluation::NextestPresenceEvaluation;

#[test]
fn unresolved_names_in_dependency_macro_output_are_violations() {
    let outcome = HeisenbergCheck {
        workspace: &FixtureWorkspace {
            name: "unresolved_names_in_dependency_macro_output",
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
            - Cargo.toml: Package outside v0.1.0 is built from outside/Cargo.toml, which is not a workspace member, so its code is not analysed\n\
            \n\
            UnresolvedMacroCall: 1 violation\n\
            The code compiles, so a name that rust-analyzer cannot resolve leaves a gap in the analysis. Define or import the macro so that rust-analyzer resolves it.\n\
            \n\
            - src/lib.rs:8:5: Macro call compiles, but rust-analyzer cannot resolve its macro\n        \
            absent_macro!();\n\
            \n\
            UnresolvedMethodCall: 1 violation\n\
            The code compiles, so a name that rust-analyzer cannot resolve leaves a gap in the analysis. Write the call so that rust-analyzer resolves its method, such as by naming the type of its receiver.\n\
            \n\
            - src/lib.rs:7:5: Method call compiles, but rust-analyzer cannot resolve its method\n        \
            absent_method!();\n\
            \n\
            UnresolvedPath: 1 violation\n\
            The code compiles, so a name that rust-analyzer cannot resolve leaves a gap in the analysis. Write the path so that rust-analyzer resolves it, such as by importing the item it names.\n\
            \n\
            - src/lib.rs:6:5: Path into outside compiles, but rust-analyzer cannot resolve it\n        \
            absent_value!();\n\
            \n\
            ------------------------------\n\
            4 errors found in 4 rules\n\
            ------------------------------\n", NextestPresenceEvaluation::NotEvaluated.host_footer(1, 1).as_str()].concat().as_str())
    );
    assert_eq!(outcome.status, HeisenbergExitStatus::Violations);
}
