use std::str;

use heisenberg_command::heisenberg_exit_status::HeisenbergExitStatus;
use heisenberg_command_tests::fixture_workspace::FixtureWorkspace;
use heisenberg_command_tests::heisenberg_check::HeisenbergCheck;
use heisenberg_command_tests::nextest_presence_evaluation::NextestPresenceEvaluation;

#[test]
fn rustfmt_configuration_missing_is_a_violation() {
    let outcome = HeisenbergCheck {
        workspace: &FixtureWorkspace {
            name: "rustfmt_configuration_missing",
        }
        .path(),
    }
    .run()
    .expect("the arguments must parse");

    assert_eq!(
        str::from_utf8(&outcome.output),
        Ok(["RustfmtConfigurationMissing: 1 violation\n\
            Without a configuration in the workspace root, the formatting of the workspace depends on files outside it. Add a rustfmt.toml to the workspace root; an empty one selects the defaults.\n\
            \n\
            - Cargo.toml: Workspace root holds neither rustfmt.toml nor .rustfmt.toml\n\
            \n\
            ------------------------------\n\
            1 errors found in 1 rule\n\
            ------------------------------\n",
            NextestPresenceEvaluation::NotEvaluated
                .host_footer(1, 1)
                .as_str()
        ]
        .concat()
        .as_str())
    );
    assert_eq!(outcome.status, HeisenbergExitStatus::Violations);
}
