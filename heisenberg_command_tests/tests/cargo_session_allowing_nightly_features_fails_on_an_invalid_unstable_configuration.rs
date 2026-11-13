use std::error::Error;

use cargo::util::data_structures::HashMap;

use heisenberg_command_tests::fixture_workspace::FixtureWorkspace;
use heisenberg_workspace_build::cargo_session::CargoSession;
use heisenberg_workspace_build::configured_context::ConfiguredContext;
use heisenberg_workspace_build::workspace_build_error::WorkspaceBuildError;

#[test]
fn cargo_session_allowing_nightly_features_fails_on_an_invalid_unstable_configuration() {
    let fixture = FixtureWorkspace {
        name: "sysroot_with_an_invalid_unstable_configuration",
    }
    .path();
    let library = fixture
        .join("sysroot")
        .join("lib")
        .join("rustlib")
        .join("src")
        .join("rust")
        .join("library");
    let failure = CargoSession::new(&ConfiguredContext {
        cargo_home: &fixture,
        environment: &HashMap::default(),
        nightly_features_allowed: true,
        workspace_directory: &library,
    })
    .err();

    assert!(matches!(
        &failure,
        Some(WorkspaceBuildError::CargoConfiguration(context))
            if matches!(&**context, WorkspaceBuildError::CargoContext(_))
    ));
    assert_eq!(
        failure.map(|error| {
            [
                error.to_string(),
                error.source().map_or_else(String::new, ToString::to_string),
            ]
        }),
        Some([
            "the cargo configuration of the build cannot be read".to_owned(),
            format!(
                "error in {}: `unstable.mtime-on-use` expected true/false, but found a string",
                library.join(".cargo").join("config.toml").display()
            ),
        ])
    );
}
