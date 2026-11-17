use std::str;

use heisenberg_command::heisenberg_exit_status::HeisenbergExitStatus;
use heisenberg_command_tests::fixture_workspace::FixtureWorkspace;
use heisenberg_command_tests::heisenberg_check::HeisenbergCheck;
use heisenberg_command_tests::nextest_presence_evaluation::NextestPresenceEvaluation;

#[test]
fn private_function_its_attribute_expansion_calls_is_file_local() {
    let outcome = HeisenbergCheck {
        workspace: &FixtureWorkspace {
            name: "function_under_a_procedural_attribute",
        }
        .path(),
    }
    .run()
    .expect("the arguments must parse");

    assert_eq!(
        str::from_utf8(&outcome.output),
        Ok(["TestOutsideTestModule: 1 violation\n\
            A test outside a test module is compiled into the same scope as production code, so nothing separates the code that runs in production from the code that tests it. Move the test into the #[cfg(test)] mod tests of its file.\n\
            \n\
            - src/trials.rs:5:3: Test is written outside a test module\n    \
            #[trial]\n\
            \n\
            ------------------------------\n\
            1 errors found in 1 rule\n\
            ------------------------------\n",
            NextestPresenceEvaluation::Evaluated.host_footer(2, 4)
                .as_str()
        ]
        .concat()
        .as_str())
    );
    assert_eq!(outcome.status, HeisenbergExitStatus::Violations);
}
