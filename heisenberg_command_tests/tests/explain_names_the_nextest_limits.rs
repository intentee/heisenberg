use std::collections::BTreeMap;
use std::str;

use clap::Parser;

use heisenberg_command::heisenberg_command::HeisenbergCommand;
use heisenberg_command::heisenberg_exit_status::HeisenbergExitStatus;
use heisenberg_command_tests::constants::EXPLAIN_SUBCOMMAND;
use heisenberg_command_tests::constants::PROGRAM_NAME;

#[test]
fn explain_names_the_nextest_limits() {
    let mut output = Vec::new();
    let mut errors = Vec::new();
    let status = HeisenbergCommand::try_parse_from([
        PROGRAM_NAME,
        EXPLAIN_SUBCOMMAND,
        "NextestTimeoutNotEnforced",
    ])
    .expect("the arguments must parse")
    .run(&BTreeMap::new(), &mut output, &mut errors)
    .expect("the errors of the run must be reported");

    assert_eq!(
        str::from_utf8(&output),
        Ok(
            "Rule 8.5 NextestTimeoutNotEnforced: A nextest profile whose slow-timeout exceeds the limits\n\
        Section 8, Tests. Scope: workspace. Applies to all code. Evaluated in each analysed configuration.\n\
        \n\
        A nextest profile whose slow-timeout flags a test as slow later than the slow limit, terminates it later than the termination limit or never, or counts a timed out test as passing. The default profile sets a slow-timeout, since the one nextest ships never terminates a test, and a profile that sets none inherits the slow-timeout of its parent.\n\
        \n\
        Limits: slow after at most 10s, terminated after at most 20s\n\
        \n\
        Rationale: A test that runs longer than the limits shows slow code or a slow test.\n\
        \n\
        Remedy: Make the code and the test faster instead of relaxing the timeout.\n\
        \n\
        See also: NextestConfigurationMissing, NextestOverride, UnneededNextestConfiguration, UnvettedNextestSetting\n\
        "
        )
    );
    assert_eq!(errors, b"");
    assert_eq!(status, HeisenbergExitStatus::Explained);
}
