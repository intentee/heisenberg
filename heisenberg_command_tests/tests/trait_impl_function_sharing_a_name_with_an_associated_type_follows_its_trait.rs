use std::str;

use heisenberg_command::heisenberg_exit_status::HeisenbergExitStatus;
use heisenberg_command_tests::fixture_workspace::FixtureWorkspace;
use heisenberg_command_tests::heisenberg_check::HeisenbergCheck;
use heisenberg_command_tests::nextest_presence_evaluation::NextestPresenceEvaluation;

#[test]
fn trait_impl_function_sharing_a_name_with_an_associated_type_follows_its_trait() {
    let outcome = HeisenbergCheck {
        workspace: &FixtureWorkspace {
            name: "trait_function_sharing_a_name_with_an_associated_type",
        }
        .path(),
    }
    .run()
    .expect("the arguments must parse");

    assert_eq!(
        str::from_utf8(&outcome.output),
        Ok([
            "------------------------------\n\
             No errors found!\n\
             ------------------------------\n",
            NextestPresenceEvaluation::NotEvaluated
                .host_footer(1, 3)
                .as_str()
        ]
        .concat()
        .as_str())
    );
    assert_eq!(outcome.status, HeisenbergExitStatus::Compliant);
}
