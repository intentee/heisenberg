use std::str;

use heisenberg_command::heisenberg_exit_status::HeisenbergExitStatus;
use heisenberg_command_tests::fixture_workspace::FixtureWorkspace;
use heisenberg_command_tests::heisenberg_check::HeisenbergCheck;
use heisenberg_command_tests::host_footer::host_footer;

#[test]
fn type_parameter_without_its_prefix_is_a_violation() {
    let outcome = HeisenbergCheck {
        workspace: &FixtureWorkspace {
            name: "unprefixed_type_parameters",
        }
        .path(),
    }
    .run()
    .expect("the arguments must parse");

    assert_eq!(
        str::from_utf8(&outcome.output),
        Ok(["UnprefixedTypeParameter: 2 violations\n\
            The T prefix tells type parameters apart from concrete types. Rename the parameter to T followed by a capitalized word, such as TNode.\n\
            \n\
            - src/pair.rs:1:17: Type parameter name is not T followed by a capitalized word\n    \
            pub struct Pair<Node, Tx> {\n\
            - src/pair.rs:1:23: Type parameter name is not T followed by a capitalized word\n    \
            pub struct Pair<Node, Tx> {\n\
            \n\
            ------------------------------\n\
            2 errors found\n\
            ------------------------------\n", host_footer(1, 2).as_str()].concat().as_str())
    );
    assert_eq!(outcome.status, HeisenbergExitStatus::Violations);
}
