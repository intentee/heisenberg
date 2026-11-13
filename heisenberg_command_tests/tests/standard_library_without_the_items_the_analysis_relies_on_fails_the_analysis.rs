use std::collections::BTreeMap;
use std::str;

use heisenberg_command::heisenberg_exit_status::HeisenbergExitStatus;
use heisenberg_command_tests::check_outcome::CheckOutcome;
use heisenberg_command_tests::constants::HOST_CONFIGURATION;
use heisenberg_command_tests::fixture_workspace::FixtureWorkspace;
use heisenberg_command_tests::heisenberg_check::HeisenbergCheck;

#[test]
fn standard_library_without_the_items_the_analysis_relies_on_fails_the_analysis() {
    let fixture = FixtureWorkspace {
        name: "standard_library_without_required_items",
    }
    .path();
    let outcome = CheckOutcome::of(
        &HeisenbergCheck {
            workspace: &fixture.join("workspace"),
        }
        .command(&fixture.join("sysroot"))
        .expect("the arguments must parse"),
        &BTreeMap::new(),
    )
    .expect("the errors of the run must be reported");

    assert_eq!(
        str::from_utf8(&outcome.errors),
        Ok(format!(
            "error: the module dependencies cannot be extracted for the configuration `{HOST_CONFIGURATION}`\n\
             error: the standard library lacks the items that the analysis relies on: core::mem::drop, core::error::Error, core::str::FromStr, core::convert::Infallible, core::iter::IntoIterator, core::iter::Iterator, core::result::Result, alloc::string::ToString\n"
        )
        .as_str())
    );
    assert_eq!(outcome.status, HeisenbergExitStatus::Failure);
}
