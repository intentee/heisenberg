use std::collections::BTreeMap;
use std::str;

use clap::Parser;

use heisenberg_command::heisenberg_command::HeisenbergCommand;
use heisenberg_command::heisenberg_exit_status::HeisenbergExitStatus;
use heisenberg_command_tests::constants::EXPLAIN_SUBCOMMAND;
use heisenberg_command_tests::constants::PROGRAM_NAME;

#[test]
fn explain_names_the_vetted_nextest_settings() {
    let mut output = Vec::new();
    let mut errors = Vec::new();
    let status = HeisenbergCommand::try_parse_from([
        PROGRAM_NAME,
        EXPLAIN_SUBCOMMAND,
        "UnvettedNextestSetting",
    ])
    .expect("the arguments must parse")
    .run(&BTreeMap::new(), &mut output, &mut errors);

    assert_eq!(
        str::from_utf8(&output),
        Ok(
            "UnvettedNextestSetting: A nextest configuration setting that Heisenberg has not vetted\n\
        \n\
        A nextest configuration setting other than the slow-timeout and those that only control concurrency, failure handling and reporting, which can retry, skip or exempt tests, or let a test run past its timeout.\n\
        \n\
        Vetted settings: nextest-version; in a profile: fail-fast, failure-output, final-status-level, global-timeout, inherits, junit, slow-timeout, status-level, success-output, test-threads; in a slow-timeout: on-timeout, period, terminate-after\n\
        \n\
        Every test runs and passes on its first attempt within the limits. Remove the setting, and make the tests that needed it faster or reliable.\n\
        "
        )
    );
    assert_eq!(errors, b"");
    assert_eq!(status, HeisenbergExitStatus::Explained);
}
