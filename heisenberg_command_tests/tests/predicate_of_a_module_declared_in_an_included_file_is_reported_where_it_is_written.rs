use std::str;

use heisenberg_command::heisenberg_exit_status::HeisenbergExitStatus;
use heisenberg_command_tests::fixture_workspace::FixtureWorkspace;
use heisenberg_command_tests::heisenberg_check::HeisenbergCheck;
use heisenberg_command_tests::nextest_presence_evaluation::NextestPresenceEvaluation;

#[test]
fn predicate_of_a_module_declared_in_an_included_file_is_reported_where_it_is_written() {
    let outcome = HeisenbergCheck {
        workspace: &FixtureWorkspace {
            name: "module_declared_in_an_included_file",
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
            - src/lib.rs:1:1: Included file src/declarations.rs is not a build script output\n    \
            include!(\"declarations.rs\");\n\
            \n\
            NegatedConfiguration: 1 violation\n\
            Code that exists only without a feature, outside tests or outside debug builds escapes testing and makes features non-additive. Express the condition positively.\n\
            \n\
            - src/limits.rs:1:1: Configuration predicate negates a feature, test or debug_assertions\n    \
            #![cfg_attr(not(test), allow(dead_code))]\n\
            \n\
            ------------------------------\n\
            2 errors found in 2 rules\n\
            ------------------------------\n", NextestPresenceEvaluation::NotEvaluated.host_footer(1, 2).as_str()].concat().as_str())
    );
    assert_eq!(outcome.status, HeisenbergExitStatus::Violations);
}
