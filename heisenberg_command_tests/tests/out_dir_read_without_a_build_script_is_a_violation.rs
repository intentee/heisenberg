use std::str;

use heisenberg_command::heisenberg_exit_status::HeisenbergExitStatus;
use heisenberg_command_tests::fixture_workspace::FixtureWorkspace;
use heisenberg_command_tests::heisenberg_check::HeisenbergCheck;

#[test]
fn out_dir_read_without_a_build_script_is_a_violation() {
    let outcome = HeisenbergCheck {
        workspace: &FixtureWorkspace {
            name: "out_dir_read_without_a_build_script",
        }
        .path(),
    }
    .run()
    .expect("the arguments must parse");

    assert_eq!(
        str::from_utf8(&outcome.output),
        Ok("UndeclaredEnvironmentVariable: 1 violation\n\
            A compile-time environment variable that only the shell sets is an input Heisenberg cannot see, so the analysis would use a wrong value. Read only variables that cargo or a build script sets, declare the variable in the [env] table of the cargo configuration, or read it at run time.\n\
            \n\
            - src/constants.rs:1:41: Macro call reads OUT_DIR, which neither cargo, a build script nor the [env] configuration sets\n    \
            pub const OUT_DIRECTORY: Option<&str> = option_env!(\"OUT_DIR\");\n\
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
