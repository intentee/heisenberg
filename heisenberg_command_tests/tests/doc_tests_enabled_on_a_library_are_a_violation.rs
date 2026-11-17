use std::str;

use heisenberg_command::heisenberg_exit_status::HeisenbergExitStatus;
use heisenberg_command_tests::fixture_workspace::FixtureWorkspace;
use heisenberg_command_tests::heisenberg_check::HeisenbergCheck;
use heisenberg_command_tests::nextest_presence_evaluation::NextestPresenceEvaluation;

#[test]
fn doc_tests_enabled_on_a_library_are_a_violation() {
    let outcome = HeisenbergCheck {
        workspace: &FixtureWorkspace {
            name: "doc_tests_enabled",
        }
        .path(),
    }
    .run()
    .expect("the arguments must parse");

    assert_eq!(
        str::from_utf8(&outcome.output),
        Ok(["DocTestsEnabled: 1 violation\n\
            A doc test is Rust code that cargo test compiles and runs out of a documentation comment, where it is text that no analysis reads, so a target with doc tests enabled can hold code that escapes every rule. Set doctest = false in the [lib] table of the package, and keep code that must compile in an example or a test, which are analysed.\n\
            \n\
            - Cargo.toml: Target lib has doc tests enabled, which cargo test runs and no analysis reads\n\
            \n\
            ------------------------------\n\
            1 errors found in 1 rule\n\
            ------------------------------\n", NextestPresenceEvaluation::NotEvaluated.host_footer(1, 1).as_str()].concat().as_str())
    );
    assert_eq!(outcome.status, HeisenbergExitStatus::Violations);
}
