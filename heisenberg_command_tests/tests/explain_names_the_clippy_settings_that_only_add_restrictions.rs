use std::collections::BTreeMap;
use std::str;

use clap::Parser;

use heisenberg_command::heisenberg_command::HeisenbergCommand;
use heisenberg_command::heisenberg_exit_status::HeisenbergExitStatus;
use heisenberg_command_tests::constants::EXPLAIN_SUBCOMMAND;
use heisenberg_command_tests::constants::PROGRAM_NAME;

#[test]
fn explain_names_the_clippy_settings_that_only_add_restrictions() {
    let mut output = Vec::new();
    let mut errors = Vec::new();
    let status = HeisenbergCommand::try_parse_from([
        PROGRAM_NAME,
        EXPLAIN_SUBCOMMAND,
        "UnvettedClippySetting",
    ])
    .expect("the arguments must parse")
    .run(&BTreeMap::new(), &mut output, &mut errors);

    assert_eq!(
        str::from_utf8(&output),
        Ok(
            "UnvettedClippySetting: A clippy configuration setting that Heisenberg has not vetted\n\
        \n\
        A clippy configuration setting other than a required one and those that only add restrictions, which can silence or relax the lints that Heisenberg requires.\n\
        \n\
        Settings that only add restrictions: absolute-paths-allowed-crates, absolute-paths-max-segments, disallowed-macros, disallowed-methods, disallowed-types\n\
        \n\
        A clippy setting can silence or relax the lints that Heisenberg requires, so a clippy configuration holds only the required settings and settings that only add restrictions. Remove the setting.\n\
        "
        )
    );
    assert_eq!(errors, b"");
    assert_eq!(status, HeisenbergExitStatus::Explained);
}
