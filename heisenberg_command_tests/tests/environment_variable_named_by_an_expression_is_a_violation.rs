use std::str;

use heisenberg_command::heisenberg_exit_status::HeisenbergExitStatus;
use heisenberg_command_tests::heisenberg_check::HeisenbergCheck;
use heisenberg_fixtures::fixture_workspace::FixtureWorkspace;

#[test]
fn environment_variable_named_by_an_expression_is_a_violation() {
    let outcome = HeisenbergCheck {
        workspace: &FixtureWorkspace {
            name: "computed_environment_variable_name",
        }
        .path(),
    }
    .run()
    .expect("the arguments must parse");

    assert_eq!(
        str::from_utf8(&outcome.output),
        Ok("ComputedEnvironmentVariableName: 1 violation\n\
            Heisenberg accounts for the environment variable a macro call reads only when a string literal names it. Name the variable with a string literal.\n\
            \n\
            - src/constants.rs:1:32: Macro call does not name its environment variable with a readable string literal\n    \
            pub const PACKAGE_NAME: &str = env!(concat!(\"CARGO_PKG_\", \"NAME\"));\n\
            \n\
            ------------------------------\n\
            1 errors found\n\
            ------------------------------\n\
            Analysed on x86_64-pc-windows-msvc: 1 crate targets, 2 modules\n\
            Analysed on x86_64-unknown-linux-gnu: 1 crate targets, 2 modules\n\
            Precondition: rustc 1.99.1 compiles the workspace for every declared target\n")
    );
    assert_eq!(outcome.status, HeisenbergExitStatus::Violations);
}
