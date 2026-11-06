use clap::error::ErrorKind;

use heisenberg_command_tests::fixture_workspace::FixtureWorkspace;
use heisenberg_command_tests::heisenberg_check::HeisenbergCheck;
use heisenberg_command_tests::heisenberg_check_error::HeisenbergCheckError;

#[test]
fn all_features_cannot_be_combined_with_a_feature_selection() {
    let Err(HeisenbergCheckError::Arguments(error)) = (HeisenbergCheck {
        workspace: &FixtureWorkspace {
            name: "feature_gated_cycle",
        }
        .path(),
    })
    .run_with_arguments(&["--all-features", "--features", "cyclic"]) else {
        panic!("--all-features must be rejected together with --features");
    };

    assert_eq!(error.kind(), ErrorKind::ArgumentConflict);
    assert_eq!(error.exit_code(), 2);
}
