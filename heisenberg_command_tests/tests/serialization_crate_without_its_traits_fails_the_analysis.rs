use std::str;

use heisenberg_command::heisenberg_exit_status::HeisenbergExitStatus;
use heisenberg_command_tests::constants::HOST_CONFIGURATION;
use heisenberg_command_tests::fixture_workspace::FixtureWorkspace;
use heisenberg_command_tests::heisenberg_check::HeisenbergCheck;

#[test]
fn serialization_crate_without_its_traits_fails_the_analysis() {
    let outcome = HeisenbergCheck {
        workspace: &FixtureWorkspace {
            name: "serialization_crate_without_traits",
        }
        .path(),
    }
    .run()
    .expect("the arguments must parse");

    assert_eq!(
        str::from_utf8(&outcome.errors),
        Ok(format!(
            "error: the module dependencies cannot be extracted for the configuration `{HOST_CONFIGURATION}`\n\
             error: the serialization crates serde_core that the analysis vets lack the traits that the serialization rule relies on: ser::Serialize, de::Deserialize, de::DeserializeOwned\n"
        )
        .as_str())
    );
    assert_eq!(outcome.status, HeisenbergExitStatus::Failure);
}
