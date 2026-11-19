use std::collections::BTreeMap;
use std::str;

use clap::Parser;

use heisenberg_command::heisenberg_command::HeisenbergCommand;
use heisenberg_command::heisenberg_exit_status::HeisenbergExitStatus;
use heisenberg_command_tests::constants::EXPLAIN_SUBCOMMAND;
use heisenberg_command_tests::constants::PROGRAM_NAME;

#[test]
fn explain_names_the_required_clippy_settings() {
    let mut output = Vec::new();
    let mut errors = Vec::new();
    let status = HeisenbergCommand::try_parse_from([
        PROGRAM_NAME,
        EXPLAIN_SUBCOMMAND,
        "RequiredClippySettingMissing",
    ])
    .expect("the arguments must parse")
    .run(&BTreeMap::new(), &mut output, &mut errors)
    .expect("the errors of the run must be reported");

    assert_eq!(
        str::from_utf8(&output),
        Ok(
            "H027 RequiredClippySettingMissing: A clippy configuration without a required setting\n\
        Section 3, Lints and formatting. Scope: workspace. Applies to all code. Evaluated in each analysed configuration.\n\
        \n\
        A clippy configuration that does not give each required setting the value it requires: true for the test allowances, which are the only way tests may use expect and unwrap, and false for avoid-breaking-exported-api, which clippy otherwise sets to true and so leaves the lints that would change a public signature silent on public items.\n\
        \n\
        Required settings: allow-expect-in-tests = true, allow-unwrap-in-tests = true, avoid-breaking-exported-api = false\n\
        \n\
        Rationale: Tests may fail fast with expect and unwrap, and the lints that clippy relaxes for public items by default must hold for them too.\n\
        \n\
        Remedy: Set each required setting to the value it names.\n\
        \n\
        See also: ClippyConfigurationAmbiguous, ClippyConfigurationMissing, UnvettedClippySetting\n\
        "
        )
    );
    assert_eq!(errors, b"");
    assert_eq!(status, HeisenbergExitStatus::Explained);
}
