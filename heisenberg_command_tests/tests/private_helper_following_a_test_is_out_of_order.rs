use std::str;

use heisenberg_command::heisenberg_exit_status::HeisenbergExitStatus;
use heisenberg_command_tests::fixture_workspace::FixtureWorkspace;
use heisenberg_command_tests::heisenberg_check::HeisenbergCheck;
use heisenberg_command_tests::host_footer::host_footer;

#[test]
fn private_helper_following_a_test_is_out_of_order() {
    let outcome = HeisenbergCheck {
        workspace: &FixtureWorkspace {
            name: "tests_among_items",
        }
        .path(),
    }
    .run()
    .expect("the arguments must parse");

    assert_eq!(
        str::from_utf8(&outcome.output),
        Ok(["ItemOutOfOrder: 1 violation\n\
            Every file reads in the same order, from what it declares and imports down to what it exposes. Move the item before the item the message names.\n\
            \n\
            - src/lib.rs:18:12: Item belongs before subtracts: private helpers precede tests\n            \
            fn helper() {}\n\
            \n\
            ------------------------------\n\
            1 errors found\n\
            ------------------------------\n", host_footer(1, 3).as_str()].concat().as_str())
    );
    assert_eq!(outcome.status, HeisenbergExitStatus::Violations);
}
