use std::collections::BTreeMap;
use std::str;

use clap::Parser;

use heisenberg_command::heisenberg_command::HeisenbergCommand;
use heisenberg_command::heisenberg_exit_status::HeisenbergExitStatus;
use heisenberg_command_tests::constants::EXPLAIN_SUBCOMMAND;
use heisenberg_command_tests::constants::PROGRAM_NAME;

#[test]
fn explain_names_the_rustfmt_configuration_files() {
    let mut output = Vec::new();
    let mut errors = Vec::new();
    let status = HeisenbergCommand::try_parse_from([
        PROGRAM_NAME,
        EXPLAIN_SUBCOMMAND,
        "RustfmtConfigurationMissing",
    ])
    .expect("the arguments must parse")
    .run(&BTreeMap::new(), &mut output, &mut errors)
    .expect("the errors of the run must be reported");

    assert_eq!(
        str::from_utf8(&output),
        Ok(
            "Rule 3.10 RustfmtConfigurationMissing: A workspace without a rustfmt configuration\n\
        Section 3, Lints and formatting. Scope: workspace. Applies to all code. Evaluated in each analysed configuration.\n\
        \n\
        A workspace root that holds neither rustfmt.toml nor .rustfmt.toml. Rustfmt searches the directory of each file and its parents for a configuration and, when it finds none, reads one from the home or user configuration directory.\n\
        \n\
        Rustfmt configuration files: rustfmt.toml, .rustfmt.toml\n\
        \n\
        Rationale: Without a configuration in the workspace root, the formatting of the workspace depends on files outside it.\n\
        \n\
        Remedy: Add a rustfmt.toml to the workspace root; an empty one selects the defaults.\n\
        \n\
        See also: RustfmtConfigurationAmbiguous, RustfmtConfigurationInvalid, RustfmtConfigurationUnreadable, UnvettedRustfmtSetting\n\
        "
        )
    );
    assert_eq!(errors, b"");
    assert_eq!(status, HeisenbergExitStatus::Explained);
}
