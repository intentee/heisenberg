use std::path::Path;
use std::str;

use heisenberg_command::heisenberg_exit_status::HeisenbergExitStatus;
use heisenberg_command_tests::constants::HOST_TRIPLE;
use heisenberg_command_tests::fixture_workspace::FixtureWorkspace;
use heisenberg_command_tests::heisenberg_check::HeisenbergCheck;

#[test]
fn proc_macro_that_fails_to_compile_fails_to_load_the_project() {
    let outcome = HeisenbergCheck {
        workspace: &FixtureWorkspace {
            name: "proc_macro_that_fails_to_compile",
        }
        .path(),
    }
    .run()
    .expect("the arguments must parse");

    assert_eq!(
        str::from_utf8(&outcome.errors),
        Ok(format!(
            "error: the project cannot be loaded\n\
             error: the build scripts and procedural macros of the workspace cannot be built for {HOST_TRIPLE}\n\
             error: the build scripts and procedural macros cannot be built or run; a package whose build script cannot run on every host can declare its output in [workspace.metadata.heisenberg.out_dirs.<package>] instead:\n\
             error[E0308]: mismatched types\n \
             --> {}:5:22\n  \
             |\n\
             5 |     let _count: u8 = \"text\";\n  \
             |                 --   ^^^^^^ expected `u8`, found `&str`\n  \
             |                 |\n  \
             |                 expected due to this\n\
             \n\
             For more information about this error, try `rustc --explain E0308`.\n\
             error: could not compile `proc_macro_that_fails_to_compile` (lib) due to 1 previous error\n\
             error: 1 job failed\n",
            Path::new("src").join("lib.rs").display()
        )
        .as_str())
    );
    assert_eq!(outcome.status, HeisenbergExitStatus::Failure);
}
