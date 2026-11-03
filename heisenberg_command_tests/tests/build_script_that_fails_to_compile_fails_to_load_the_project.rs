use std::str;

use heisenberg_command::heisenberg_exit_status::HeisenbergExitStatus;
use heisenberg_command_tests::fixture_workspace::FixtureWorkspace;
use heisenberg_command_tests::heisenberg_check::HeisenbergCheck;

#[test]
fn build_script_that_fails_to_compile_fails_to_load_the_project() {
    let outcome = HeisenbergCheck {
        workspace: &FixtureWorkspace {
            name: "build_script_that_fails_to_compile",
        }
        .path(),
    }
    .run()
    .expect("the arguments must parse");

    assert_eq!(
        str::from_utf8(&outcome.errors),
        Ok("error: the project cannot be loaded\n\
            error: the build scripts and procedural macros of the workspace cannot be built for x86_64-pc-windows-msvc\n\
            error: the build scripts and procedural macros cannot be built or run:\n\
            error[E0308]: mismatched types\n \
            --> build.rs:2:22\n  \
            |\n\
            2 |     let _count: u8 = \"text\";\n  \
            |                 --   ^^^^^^ expected `u8`, found `&str`\n  \
            |                 |\n  \
            |                 expected due to this\n\
            \n\
            For more information about this error, try `rustc --explain E0308`.\n\
            error: could not compile `build_script_that_fails_to_compile` (build script) due to 1 previous error\n\
            error: 1 job failed\n")
    );
    assert_eq!(outcome.status, HeisenbergExitStatus::Failure);
}
