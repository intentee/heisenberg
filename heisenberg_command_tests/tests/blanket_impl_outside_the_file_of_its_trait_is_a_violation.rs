use std::str;

use heisenberg_command::heisenberg_exit_status::HeisenbergExitStatus;
use heisenberg_command_tests::fixture_workspace::FixtureWorkspace;
use heisenberg_command_tests::heisenberg_check::HeisenbergCheck;
use heisenberg_command_tests::host_footer::host_footer;

#[test]
fn blanket_impl_outside_the_file_of_its_trait_is_a_violation() {
    let outcome = HeisenbergCheck {
        workspace: &FixtureWorkspace {
            name: "blanket_impl_outside_the_trait_file",
        }
        .path(),
    }
    .run()
    .expect("the arguments must parse");

    assert_eq!(
        str::from_utf8(&outcome.output),
        Ok(["MisplacedBlanketImpl: 1 violation\n\
            A blanket impl reaches every type its bounds admit, so it belongs beside its trait. Move the impl into the file that defines the trait.\n\
            \n\
            - src/counted.rs:3:40: Blanket impl belongs in src/counting.rs\n    \
            impl<TIterator: Iterator> Counting for TIterator {\n\
            \n\
            ------------------------------\n\
            1 errors found\n\
            ------------------------------\n", host_footer(1, 3).as_str()].concat().as_str())
    );
    assert_eq!(outcome.status, HeisenbergExitStatus::Violations);
}
