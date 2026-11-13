use std::str;

use heisenberg_command::heisenberg_exit_status::HeisenbergExitStatus;
use heisenberg_command_tests::fixture_workspace::FixtureWorkspace;
use heisenberg_command_tests::heisenberg_check::HeisenbergCheck;
use heisenberg_command_tests::nextest_presence_evaluation::NextestPresenceEvaluation;

#[test]
fn path_in_an_included_file_closes_a_cycle_at_the_include() {
    let outcome = HeisenbergCheck {
        workspace: &FixtureWorkspace {
            name: "cycle_through_an_included_file",
        }
        .path(),
    }
    .run()
    .expect("the arguments must parse");

    assert_eq!(
        str::from_utf8(&outcome.output),
        Ok(["IncludedSourceFile: 1 violation\n\
            An included file is trusted as generated text, so code written by hand escapes every rule once it is included. Make the file a module, or have a build script generate it into OUT_DIR.\n\
            \n\
            - src/first.rs:1:1: Included file snippets/second.rs is not a build script output\n    \
            include!(\"../snippets/second.rs\");\n\
            \n\
            ModuleCycle: 1 violation\n\
            Modules that depend on each other cannot be understood or changed in isolation. Break the cycle so the modules form layers.\n\
            \n\
            - src/first.rs:1:1: Parts of crate depend on each other\n    \
            include!(\"../snippets/second.rs\");\n      \
            crate::first -> crate::second:\n        \
            crate::first uses crate::second at src/first.rs:1:1\n      \
            crate::second -> crate::first:\n        \
            crate::second uses crate::first at src/second.rs:4:23\n        \
            crate::second uses crate::first at src/second.rs:5:9\n\
            \n\
            ------------------------------\n\
            2 errors found\n\
            ------------------------------\n", NextestPresenceEvaluation::NotEvaluated.host_footer(1, 3).as_str()].concat().as_str())
    );
    assert_eq!(outcome.status, HeisenbergExitStatus::Violations);
}
