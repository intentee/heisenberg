use std::str;

use heisenberg_command::heisenberg_exit_status::HeisenbergExitStatus;
use heisenberg_command_tests::heisenberg_check::HeisenbergCheck;
use heisenberg_fixtures::fixture_workspace::FixtureWorkspace;

#[test]
fn unstable_feature_in_the_cargo_configuration_fails_to_load_the_project() {
    let workspace = FixtureWorkspace {
        name: "unstable_feature_in_the_cargo_configuration",
    }
    .path();
    let manifest = workspace.join("Cargo.toml");
    let outcome = HeisenbergCheck {
        workspace: &workspace,
    }
    .run()
    .expect("the arguments must parse");

    assert_eq!(
        str::from_utf8(&outcome.errors),
        Ok(format!(
            "error: the project cannot be loaded\n\
             error: the cargo workspace `{0}` cannot be loaded\n\
             error: failed to parse manifest at `{0}`\n\
             error: `artifact = …` requires `-Z bindeps` (tool)\n",
            manifest.display()
        )
        .as_str())
    );
    assert_eq!(outcome.status, HeisenbergExitStatus::Failure);
}
