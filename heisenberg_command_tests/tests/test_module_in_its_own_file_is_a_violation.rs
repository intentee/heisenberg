use std::str;

use heisenberg_command::heisenberg_exit_status::HeisenbergExitStatus;
use heisenberg_command_tests::fixture_workspace::FixtureWorkspace;
use heisenberg_command_tests::heisenberg_check::HeisenbergCheck;
use heisenberg_command_tests::nextest_presence_evaluation::NextestPresenceEvaluation;

#[test]
fn test_module_in_its_own_file_is_a_violation() {
    let outcome = HeisenbergCheck {
        workspace: &FixtureWorkspace {
            name: "test_modules_in_their_own_files",
        }
        .path(),
    }
    .run()
    .expect("the arguments must parse");

    assert_eq!(
        str::from_utf8(&outcome.output),
        Ok(["TestModuleInItsOwnFile: 2 violations\n\
            The unit tests of a file are part of that file, and code that several tests share is a test library, which belongs in a test crate. Write the tests in an inline #[cfg(test)] mod tests of the file they test, and move shared test code to a test crate.\n\
            \n\
            - src/lib.rs:2:5: Test module is declared in a file of its own\n    \
            mod sample;\n\
            - src/lib.rs:4:5: Test module is declared in a file of its own\n    \
            mod tests;\n\
            \n\
            ------------------------------\n\
            2 errors found in 1 rule\n\
            ------------------------------\n", NextestPresenceEvaluation::Evaluated.host_footer(1, 3).as_str()].concat().as_str())
    );
    assert_eq!(outcome.status, HeisenbergExitStatus::Violations);
}
