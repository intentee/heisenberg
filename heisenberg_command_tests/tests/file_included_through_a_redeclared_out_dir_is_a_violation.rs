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
        Ok(["IncludedSourceFile: 1 violation\n\
            An included file is trusted as generated text, so code written by hand escapes the rules about how code is written once it is included. Make the file a module, or have a build script generate it into OUT_DIR.\n\
            \n\
            - src/lib.rs:1:1: Included file handwritten/items.rs is not a build script output\n    \
            include!(concat!(env!(\"OUT_DIR\"), \"/items.rs\"));\n\
            \n\
            ------------------------------\n\
            1 errors found in 1 rule\n\
            ------------------------------\n", NextestPresenceEvaluation::NotEvaluated.host_footer(2, 2).as_str()].concat().as_str())
    );
    assert_eq!(outcome.status, HeisenbergExitStatus::Violations);
}
