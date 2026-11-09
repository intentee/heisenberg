use std::fs;
use std::str;

use toml::Table;

use heisenberg_command::heisenberg_exit_status::HeisenbergExitStatus;
use heisenberg_command_tests::fixture_workspace::FixtureWorkspace;
use heisenberg_command_tests::heisenberg_check::HeisenbergCheck;

#[test]
fn invalid_nextest_configuration_fails_to_load_the_project() {
    let workspace = FixtureWorkspace {
        name: "nextest_configuration_invalid",
    }
    .path();
    let configuration = workspace.join(".config").join("nextest.toml");
    let parse_error = fs::read_to_string(&configuration)
        .expect("the fixture holds the configuration")
        .parse::<Table>()
        .expect_err("the fixture configuration is not valid TOML");
    let outcome = HeisenbergCheck {
        workspace: &workspace,
    }
    .run()
    .expect("the arguments must parse");

    assert_eq!(
        str::from_utf8(&outcome.errors),
        Ok(format!(
            "error: the project cannot be loaded\n\
             error: the nextest configuration `{}` is not valid\n\
             error: {}\n",
            configuration.display(),
            parse_error.to_string().trim_end()
        )
        .as_str())
    );
    assert_eq!(outcome.status, HeisenbergExitStatus::Failure);
}
