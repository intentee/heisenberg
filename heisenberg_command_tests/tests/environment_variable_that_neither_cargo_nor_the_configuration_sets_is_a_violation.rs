use std::str;

use heisenberg_command::heisenberg_exit_status::HeisenbergExitStatus;
use heisenberg_command_tests::heisenberg_check::HeisenbergCheck;
use heisenberg_fixtures::fixture_workspace::FixtureWorkspace;

#[test]
fn environment_variable_that_neither_cargo_nor_the_configuration_sets_is_a_violation() {
    let outcome = HeisenbergCheck {
        workspace: &FixtureWorkspace {
            name: "undeclared_environment_variables",
        }
        .path(),
    }
    .run()
    .expect("the arguments must parse");

    assert_eq!(
        str::from_utf8(&outcome.output),
        Ok("UndeclaredEnvironmentVariable: 2 violations\n\
            A compile-time environment variable that a build script or the shell sets is an input Heisenberg cannot see, so the analysis would use a wrong value. Read only variables cargo sets, declare the variable in the [env] table of the cargo configuration, or read it at run time.\n\
            \n\
            - src/constants.rs:3:43: Macro call reads SET_BY_THE_BUILD_SCRIPT, which neither cargo nor the [env] configuration sets\n    \
            pub const SET_BY_THE_BUILD_SCRIPT: &str = env!(\"SET_BY_THE_BUILD_SCRIPT\");\n\
            - src/constants.rs:4:38: Macro call reads UNDECLARED_ANYWHERE, which neither cargo nor the [env] configuration sets\n    \
            pub const UNDECLARED: Option<&str> = option_env!(\"UNDECLARED_ANYWHERE\");\n\
            \n\
            ------------------------------\n\
            2 errors found\n\
            ------------------------------\n\
            Analysed on x86_64-pc-windows-msvc: 4 crate targets, 5 modules\n\
            Analysed on x86_64-unknown-linux-gnu: 4 crate targets, 5 modules\n\
            Precondition: rustc 1.99.1 compiles the workspace for every declared target\n")
    );
    assert_eq!(outcome.status, HeisenbergExitStatus::Violations);
}
