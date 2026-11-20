use std::str;

use heisenberg_command::heisenberg_exit_status::HeisenbergExitStatus;
use heisenberg_command_tests::fixture_workspace::FixtureWorkspace;
use heisenberg_command_tests::heisenberg_check::HeisenbergCheck;
use heisenberg_command_tests::nextest_presence_evaluation::NextestPresenceEvaluation;

#[test]
fn rust_version_inherited_from_the_workspace_is_reported_at_the_workspace() {
    let outcome = HeisenbergCheck {
        workspace: &FixtureWorkspace {
            name: "rust_version_inherited_below_supported",
        }
        .path(),
    }
    .run()
    .expect("the arguments must parse");

    assert_eq!(
        str::from_utf8(&outcome.output),
        Ok(["RustVersionBelowSupported: 1 violation\n\
            The workspace is analysed and linted as the supported Rust compiles it, so a lower rust-version only relaxes the lints that Heisenberg requires. Remove rust-version or set it to the supported Rust version.\n\
            \n\
            - Cargo.toml: Workspace declares rust-version 1.85, which is older than the supported Rust 1.100.0\n\
            \n\
            ------------------------------\n\
            1 errors found in 1 rule\n\
            ------------------------------\n", NextestPresenceEvaluation::NotEvaluated.host_footer(1, 1).as_str()].concat().as_str())
    );
    assert_eq!(outcome.status, HeisenbergExitStatus::Violations);
}
