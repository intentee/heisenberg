use std::str;

use heisenberg_command::heisenberg_exit_status::HeisenbergExitStatus;
use heisenberg_command_tests::fixture_workspace::FixtureWorkspace;
use heisenberg_command_tests::heisenberg_check::HeisenbergCheck;
use heisenberg_command_tests::host_footer::host_footer;

#[test]
fn tuple_in_an_item_signature_is_a_violation() {
    let outcome = HeisenbergCheck {
        workspace: &FixtureWorkspace {
            name: "tuples_in_signatures",
        }
        .path(),
    }
    .run()
    .expect("the arguments must parse");

    assert_eq!(
        str::from_utf8(&outcome.output),
        Ok(["TupleInSignature: 7 violations\n\
            A tuple names none of its values, so its readers must remember what each position holds. Declare a struct with named fields instead.\n\
            \n\
            - src/constants.rs:1:19: Tuple type in a signature\n    \
            pub const ORIGIN: (u8, u8) = (0, 0);\n\
            - src/counter.rs:4:17: Tuple type in a signature\n        \
            type Item = (u8, u8);\n\
            - src/measure.rs:2:31: Tuple type in a signature\n        \
            fn measure(&self, bounds: (u8, u8)) -> u8;\n\
            - src/pair.rs:1:17: Tuple type in a signature\n    \
            pub type Pair = (u8, u8);\n\
            - src/pairs.rs:1:23: Tuple type in a signature\n    \
            pub fn pairs() -> Vec<(u8, u8)> {\n\
            - src/span.rs:2:17: Tuple type in a signature\n        \
            pub bounds: (u8, u8),\n\
            - src/span.rs:5:11: Tuple type in a signature\n    \
            impl From<(u8, u8)> for Span {\n\
            \n\
            ------------------------------\n\
            7 errors found\n\
            ------------------------------\n", host_footer(1, 7).as_str()].concat().as_str())
    );
    assert_eq!(outcome.status, HeisenbergExitStatus::Violations);
}
