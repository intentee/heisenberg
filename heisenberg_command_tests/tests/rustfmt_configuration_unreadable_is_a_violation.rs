use std::fs;
use std::str;

use heisenberg_command::heisenberg_exit_status::HeisenbergExitStatus;
use heisenberg_command_tests::fixture_workspace::FixtureWorkspace;
use heisenberg_command_tests::heisenberg_check::HeisenbergCheck;
use heisenberg_command_tests::nextest_presence_evaluation::NextestPresenceEvaluation;

#[test]
fn rustfmt_configuration_unreadable_is_a_violation() {
    let workspace = FixtureWorkspace {
        name: "rustfmt_configuration_unreadable",
    }
    .path();
    let read_error = fs::read_to_string(workspace.join("rustfmt.toml"))
        .expect_err("the fixture configuration is a directory");
    let outcome = HeisenbergCheck {
        workspace: &workspace,
    }
    .run()
    .expect("the arguments must parse");

    assert_eq!(
        str::from_utf8(&outcome.output),
        Ok(format!(
            "RustfmtConfigurationUnreadable: 1 violation\n\
             A configuration that cannot be read cannot be vetted, and what rustfmt makes of it is not a declared input. Replace it with a configuration file or remove it.\n\
             \n\
             - rustfmt.toml: Rustfmt configuration cannot be read: {}\n\
             \n\
             ------------------------------\n\
             1 errors found in 1 rule\n\
             ------------------------------\n\
             {}",
            read_error.kind(),
            NextestPresenceEvaluation::NotEvaluated.host_footer(1, 1)
        )
        .as_str())
    );
    assert_eq!(outcome.status, HeisenbergExitStatus::Violations);
}
