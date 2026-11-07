use std::collections::BTreeMap;
use std::str;

use clap::Parser;

use heisenberg_command::heisenberg_command::HeisenbergCommand;
use heisenberg_command::heisenberg_exit_status::HeisenbergExitStatus;

#[test]
fn explain_names_the_required_clippy_settings() {
    let mut output = Vec::new();
    let mut errors = Vec::new();
    let status = HeisenbergCommand::try_parse_from([
        "heisenberg",
        "explain",
        "RequiredClippySettingMissing",
    ])
    .expect("the arguments must parse")
    .run(&BTreeMap::new(), &mut output, &mut errors);

    assert_eq!(
        str::from_utf8(&output),
        Ok(
            "RequiredClippySettingMissing: A clippy configuration without a required setting\n\
        \n\
        A clippy configuration that does not give each required setting the value it requires: true for the test allowances, which are the only way tests may use expect, unwrap and panic, and false for avoid-breaking-exported-api, which clippy otherwise sets to true and so leaves the lints that would change a public signature silent on public items.\n\
        \n\
        Required settings: allow-expect-in-tests = true, allow-panic-in-tests = true, allow-unwrap-in-tests = true, avoid-breaking-exported-api = false\n\
        \n\
        Tests may fail fast with expect, unwrap and panic, and the lints that clippy relaxes for public items by default must hold for them too. Set each required setting to the value it names.\n\
        "
        )
    );
    assert_eq!(errors, b"");
    assert_eq!(status, HeisenbergExitStatus::Explained);
}
