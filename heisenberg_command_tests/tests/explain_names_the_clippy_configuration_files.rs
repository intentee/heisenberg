use std::collections::BTreeMap;
use std::str;

use clap::Parser;

use heisenberg_command::heisenberg_command::HeisenbergCommand;
use heisenberg_command::heisenberg_exit_status::HeisenbergExitStatus;
use heisenberg_command_tests::constants::EXPLAIN_SUBCOMMAND;
use heisenberg_command_tests::constants::PROGRAM_NAME;

#[test]
fn explain_names_the_clippy_configuration_files() {
    let mut output = Vec::new();
    let mut errors = Vec::new();
    let status = HeisenbergCommand::try_parse_from([
        PROGRAM_NAME,
        EXPLAIN_SUBCOMMAND,
        "ClippyConfigurationMissing",
    ])
    .expect("the arguments must parse")
    .run(&BTreeMap::new(), &mut output, &mut errors)
    .expect("the errors of the run must be reported");

    assert_eq!(
        str::from_utf8(&output),
        Ok(
            "Rule 3.2 ClippyConfigurationMissing: A workspace member without a clippy configuration\n\
        Section 3, Lints and formatting. Scope: workspace. Applies to all code. Evaluated in each analysed configuration.\n\
        \n\
        A workspace member with no clippy configuration file in its directory or an ancestor within the workspace.\n\
        \n\
        Clippy configuration files: clippy.toml, .clippy.toml\n\
        \n\
        Rationale: Tests may fail fast while production code may not, and clippy needs a configuration to tell them apart.\n\
        \n\
        Remedy: Add a clippy.toml with the test allowances.\n\
        \n\
        See also: ClippyConfigurationAmbiguous, RequiredClippySettingMissing, UnvettedClippySetting\n\
        "
        )
    );
    assert_eq!(errors, b"");
    assert_eq!(status, HeisenbergExitStatus::Explained);
}
