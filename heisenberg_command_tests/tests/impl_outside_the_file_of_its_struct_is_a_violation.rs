use std::str;

use heisenberg_command::heisenberg_exit_status::HeisenbergExitStatus;
use heisenberg_command_tests::fixture_workspace::FixtureWorkspace;
use heisenberg_command_tests::heisenberg_check::HeisenbergCheck;
use heisenberg_command_tests::nextest_presence_evaluation::NextestPresenceEvaluation;

#[test]
fn impl_outside_the_file_of_its_struct_is_a_violation() {
    let outcome = HeisenbergCheck {
        workspace: &FixtureWorkspace {
            name: "misplaced_impl",
        }
        .path(),
    }
    .run()
    .expect("the arguments must parse");

    assert_eq!(
        str::from_utf8(&outcome.output),
        Ok(["MisplacedImpl: 2 violations\n\
            All behavior of a type is read in one place, the file that defines it. Move the impl block into that file.\n\
            \n\
            - src/second.rs:4:6: Impl block belongs in src/first.rs\n    \
            impl First {\n\
            - src/second.rs:10:19: Impl block belongs in src/first.rs\n    \
            impl Greeting for First {\n\
            \n\
            ------------------------------\n\
            2 errors found in 1 rule\n\
            ------------------------------\n", NextestPresenceEvaluation::NotEvaluated.host_footer(1, 4).as_str()].concat().as_str())
    );
    assert_eq!(outcome.status, HeisenbergExitStatus::Violations);
}
