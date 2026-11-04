use std::collections::BTreeMap;
use std::str;

use clap::Parser;

use heisenberg_command::heisenberg_command::HeisenbergCommand;
use heisenberg_command::heisenberg_exit_status::HeisenbergExitStatus;

#[test]
fn explain_names_the_test_allowances() {
    let mut output = Vec::new();
    let mut errors = Vec::new();
    let status =
        HeisenbergCommand::try_parse_from(["heisenberg", "explain", "ClippyTestAllowanceMissing"])
            .expect("the arguments must parse")
            .run(&BTreeMap::new(), &mut output, &mut errors);

    assert_eq!(
        str::from_utf8(&output),
        Ok(
            "ClippyTestAllowanceMissing: A clippy configuration that does not allow tests to fail fast\n\
        \n\
        A clippy configuration that does not set each test allowance to true, which is the only way tests may use them.\n\
        \n\
        Test allowances: allow-expect-in-tests, allow-panic-in-tests, allow-unwrap-in-tests\n\
        \n\
        Tests may fail fast with expect, unwrap and panic. Set the allowance to true in the clippy configuration.\n\
        "
        )
    );
    assert_eq!(errors, b"");
    assert_eq!(status, HeisenbergExitStatus::Explained);
}
