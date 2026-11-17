use std::str;

use heisenberg_command::heisenberg_exit_status::HeisenbergExitStatus;
use heisenberg_command_tests::fixture_workspace::FixtureWorkspace;
use heisenberg_command_tests::heisenberg_check::HeisenbergCheck;
use heisenberg_command_tests::nextest_presence_evaluation::NextestPresenceEvaluation;

#[test]
fn rustfmt_configuration_ambiguous_is_a_violation() {
    let outcome = HeisenbergCheck {
        workspace: &FixtureWorkspace {
            name: "rustfmt_configuration_ambiguous",
        }
        .path(),
    }
    .run()
    .expect("the arguments must parse");

    assert_eq!(
        str::from_utf8(&outcome.output),
        Ok(["RustfmtConfigurationAmbiguous: 1 violation\n\
            One configuration decides how the code of a directory is formatted. Keep one of the two files.\n\
            \n\
            - rustfmt.toml: Directory holds both rustfmt.toml and .rustfmt.toml\n\
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
