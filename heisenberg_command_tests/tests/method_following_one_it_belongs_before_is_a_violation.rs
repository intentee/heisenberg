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
        Ok(["MethodOutOfOrder: 12 violations\n\
            Constructors come first and wider visibility before narrower, so the interface of a type reads from the top and each function is found by its name, and a trait impl reads in the order of its trait. Move the function before the function the message names.\n\
            \n\
            - src/counter.rs:12:12: Method belongs before count: public constructors precede public methods\n        \
            pub fn new() -> Self {\n\
            - src/counter.rs:20:19: Method belongs before reset: restricted methods precede private methods\n        \
            pub(super) fn halve(&mut self) {\n\
            - src/counter.rs:24:19: Method belongs before reset: crate methods precede private methods\n        \
            pub(crate) fn bump(&mut self) {\n\
            - src/counter.rs:28:12: Method belongs before count: public methods are sorted by name\n        \
            pub fn add(&mut self, amount: u8) {\n\
            - src/counter.rs:32:12: Method belongs before reset: public methods precede private methods\n        \
            pub fn reversed(&self) -> Self {\n\
            - src/counter.rs:38:19: Method belongs before count: crate constructors precede public methods\n        \
            pub(crate) fn empty() -> Self {\n\
            - src/counter.rs:42:12: Method belongs before count: public constructors precede public methods\n        \
            pub fn with_count(self, count: u8) -> Self {\n\
            - src/counter.rs:46:19: Method belongs before count: restricted constructors precede public methods\n        \
            pub(super) fn third() -> Self {\n\
            - src/counter.rs:50:8: Method belongs before count: private constructors precede public methods\n        \
            fn fourth() -> Self {\n\
            - src/counter.rs:62:8: Method belongs before size_hint: Iterator declares it earlier\n        \
            fn next(&mut self) -> Option<u8> {\n\
            - src/tally.rs:4:8: Method belongs before total: public constructors precede public methods\n        \
            fn create() -> Self;\n\
            - src/tally.rs:6:8: Method belongs before total: public methods are sorted by name\n        \
            fn average(&self) -> u8;\n\
            \n\
            ------------------------------\n\
            12 errors found\n\
            ------------------------------\n", host_footer(1, 3).as_str()].concat().as_str())
    );
    assert_eq!(outcome.status, HeisenbergExitStatus::Violations);
}
