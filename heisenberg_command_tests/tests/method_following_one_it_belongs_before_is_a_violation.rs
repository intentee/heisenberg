use std::str;

use heisenberg_command::heisenberg_exit_status::HeisenbergExitStatus;
use heisenberg_command_tests::fixture_workspace::FixtureWorkspace;
use heisenberg_command_tests::heisenberg_check::HeisenbergCheck;
use heisenberg_command_tests::host_footer::host_footer;

#[test]
fn method_following_one_it_belongs_before_is_a_violation() {
    let outcome = HeisenbergCheck {
        workspace: &FixtureWorkspace {
            name: "methods_out_of_order",
        }
        .path(),
    }
    .run()
    .expect("the arguments must parse");

    assert_eq!(
        str::from_utf8(&outcome.output),
        Ok(["MethodOutOfOrder: 5 violations\n\
            Constructors come first and wider visibility before narrower, so the interface of a type reads from the top and each function is found by its name. Move the function before the function the message names.\n\
            \n\
            - src/counter.rs:10:12: Method belongs before count: constructors precede public methods\n        \
            pub fn new() -> Self {\n\
            - src/counter.rs:18:19: Method belongs before reset: restricted methods precede private methods\n        \
            pub(super) fn halve(&mut self) {\n\
            - src/counter.rs:22:19: Method belongs before reset: crate methods precede private methods\n        \
            pub(crate) fn bump(&mut self) {\n\
            - src/counter.rs:26:12: Method belongs before count: public methods are sorted by name\n        \
            pub fn add(&mut self, amount: u8) {\n\
            - src/counter.rs:38:8: Method belongs before size_hint: trait methods are sorted by name\n        \
            fn next(&mut self) -> Option<u8> {\n\
            \n\
            ------------------------------\n\
            5 errors found\n\
            ------------------------------\n", host_footer(1, 2).as_str()].concat().as_str())
    );
    assert_eq!(outcome.status, HeisenbergExitStatus::Violations);
}
