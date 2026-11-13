use std::str;

use heisenberg_command::heisenberg_exit_status::HeisenbergExitStatus;
use heisenberg_command_tests::fixture_workspace::FixtureWorkspace;
use heisenberg_command_tests::heisenberg_check::HeisenbergCheck;
use heisenberg_command_tests::nextest_presence_evaluation::NextestPresenceEvaluation;

#[test]
fn test_that_only_checks_a_serialization_round_trip_is_a_violation() {
    let outcome = HeisenbergCheck {
        workspace: &FixtureWorkspace {
            name: "serialization_round_trip_tests",
        }
        .path(),
    }
    .run()
    .expect("the arguments must parse");

    assert_eq!(
        str::from_utf8(&outcome.output),
        Ok([
            "SerializationRoundTripTest: 4 violations\n\
            A round trip only shows that the serialization library is consistent with itself, which its own tests already cover, and it still passes when the encoded form is wrong. Test the behaviour that depends on the encoded form, such as the exact data it produces or the value a fixed input decodes to.\n\
            \n\
            - src/round_trips.rs:11:4: Test only compares values with what their serialization round trip decodes\n    \
            fn json_round_trip_of_numbers() {\n\
            - src/round_trips.rs:20:4: Test only compares values with what their serialization round trip decodes\n    \
            fn text_round_trip_of_a_number() {\n\
            - src/round_trips.rs:29:4: Test only compares values with what their serialization round trip decodes\n    \
            fn json_round_trip_through_bytes() {\n\
            - src/round_trips.rs:56:4: Test only compares values with what their serialization round trip decodes\n    \
            fn text_round_trip_through_from_str() -> Result<(), ParseIntError> {\n\
            \n\
            ------------------------------\n\
            4 errors found\n\
            ------------------------------\n",
            NextestPresenceEvaluation::Evaluated.host_footer(1, 2).as_str()
        ]
        .concat()
        .as_str())
    );
    assert_eq!(outcome.status, HeisenbergExitStatus::Violations);
}
