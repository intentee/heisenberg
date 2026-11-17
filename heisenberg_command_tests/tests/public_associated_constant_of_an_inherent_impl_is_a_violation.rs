use std::str;

use heisenberg_command::heisenberg_exit_status::HeisenbergExitStatus;
use heisenberg_command_tests::fixture_workspace::FixtureWorkspace;
use heisenberg_command_tests::heisenberg_check::HeisenbergCheck;
use heisenberg_command_tests::nextest_presence_evaluation::NextestPresenceEvaluation;

#[test]
fn public_associated_constant_of_an_inherent_impl_is_a_violation() {
    let outcome = HeisenbergCheck {
        workspace: &FixtureWorkspace {
            name: "public_associated_constants",
        }
        .path(),
    }
    .run()
    .expect("the arguments must parse");

    assert_eq!(
        str::from_utf8(&outcome.output),
        Ok(["PublicAssociatedConstant: 2 violations\n\
            Public constants have a single home, crate::constants. Move the constant there as a free constant.\n\
            \n\
            - src/constants.rs:4:15: Public associated constant belongs in crate::constants as a free constant\n        \
            pub const CEILING: u8 = 10;\n\
            - src/limits.rs:4:15: Public associated constant belongs in crate::constants as a free constant\n        \
            pub const MINIMUM: u8 = 1;\n\
            \n\
            ------------------------------\n\
            2 errors found in 1 rule\n\
            ------------------------------\n", NextestPresenceEvaluation::NotEvaluated.host_footer(1, 3).as_str()].concat().as_str())
    );
    assert_eq!(outcome.status, HeisenbergExitStatus::Violations);
}
