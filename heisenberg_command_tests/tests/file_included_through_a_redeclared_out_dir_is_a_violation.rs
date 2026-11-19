use std::str;

use heisenberg_command::heisenberg_exit_status::HeisenbergExitStatus;
use heisenberg_command_tests::fixture_workspace::FixtureWorkspace;
use heisenberg_command_tests::heisenberg_check::HeisenbergCheck;
use heisenberg_command_tests::nextest_presence_evaluation::NextestPresenceEvaluation;

#[test]
fn file_included_through_a_redeclared_out_dir_is_a_violation() {
    let outcome = HeisenbergCheck {
        workspace: &FixtureWorkspace {
            name: "out_dir_redeclared_by_its_build_script",
        }
        .path(),
    }
    .run()
    .expect("the arguments must parse");

    assert_eq!(
        str::from_utf8(&outcome.output),
        Ok(["BuildScriptEnvironmentVariable: 1 violation\n\
            Cargo hands such a variable to the compiler after the variables it sets itself, so it reaches every tool that runs in place of the compiler: clippy reads its lint arguments and the location of its configuration from that environment, and a build script can switch off the lints the workspace denies. Write the value into a file in OUT_DIR and bring it in with include!, or declare a fixed value in the [env] table of the cargo configuration.\n\
            \n\
            - build.rs: Build script of out_dir_redeclared_by_its_build_script sets environment variables for the compiler\n      \
            sets OUT_DIR\n\
            \n\
            IncludedSourceFile: 1 violation\n\
            An included file is trusted as generated text, so code written by hand escapes the rules about how code is written once it is included. Make the file a module, or have a build script generate it into OUT_DIR.\n\
            \n\
            - src/lib.rs:1:1: Included file handwritten/items.rs is not a build script output\n    \
            include!(concat!(env!(\"OUT_DIR\"), \"/items.rs\"));\n\
            \n\
            ------------------------------\n\
            2 errors found in 2 rules\n\
            ------------------------------\n", NextestPresenceEvaluation::NotEvaluated.host_footer(2, 2).as_str()].concat().as_str())
    );
    assert_eq!(outcome.status, HeisenbergExitStatus::Violations);
}
