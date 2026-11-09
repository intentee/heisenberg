use std::collections::BTreeMap;
use std::str;

use clap::Parser;

use heisenberg_command::heisenberg_command::HeisenbergCommand;
use heisenberg_command::heisenberg_exit_status::HeisenbergExitStatus;

#[test]
fn explain_names_the_nextest_limits() {
    let mut output = Vec::new();
    let mut errors = Vec::new();
    let status =
        HeisenbergCommand::try_parse_from(["heisenberg", "explain", "NextestTimeoutNotEnforced"])
            .expect("the arguments must parse")
            .run(&BTreeMap::new(), &mut output, &mut errors);

    assert_eq!(
        str::from_utf8(&output),
        Ok(
            "NextestTimeoutNotEnforced: A nextest profile whose slow-timeout exceeds the limits\n\
        \n\
        A nextest profile whose slow-timeout flags a test as slow later than the slow limit, terminates it later than the termination limit or never, or counts a timed out test as passing. The default profile sets a slow-timeout, since the one nextest ships never terminates a test, and a profile that sets none inherits the slow-timeout of its parent.\n\
        \n\
        Limits: slow after at most 10s, terminated after at most 20s\n\
        \n\
        A test that runs longer than the limits shows slow code or a slow test. Make the code and the test faster instead of relaxing the timeout.\n\
        "
        )
    );
    assert_eq!(errors, b"");
    assert_eq!(status, HeisenbergExitStatus::Explained);
}
