use std::path::Path;
use std::str;

use heisenberg_command::heisenberg_exit_status::HeisenbergExitStatus;
use heisenberg_command_tests::full_run_footer::full_run_footer;
use heisenberg_command_tests::heisenberg_check::HeisenbergCheck;
use heisenberg_command_tests::host_only_fixture::HostOnlyFixture;

#[test]
fn attribute_applied_in_no_configuration_is_a_violation() {
    let outcome = HeisenbergCheck {
        workspace: &HostOnlyFixture {
            directory: Path::new(env!("CARGO_TARGET_TMPDIR")),
            name: "attributes_applied_in_no_configuration",
        }
        .path()
        .expect("the host-only fixture must be prepared"),
    }
    .run()
    .expect("the arguments must parse");

    assert_eq!(
        str::from_utf8(&outcome.output),
        Ok(["AttributeAppliedInNoConfiguration: 2 violations\n\
            An attribute that applies in no declared configuration changes nothing that Heisenberg analyses, so it only changes what a tool outside the declared configurations sees. Remove the attribute, or declare a configuration it applies in.\n\
            \n\
            - src/lib.rs:4:7: Conditional attribute applies in no declared configuration\n        \
            #[cfg_attr(rustfmt, rustfmt::skip)]\n\
            - src/lib.rs:7:45: Conditional attribute applies in no declared configuration\n        \
            #[cfg_attr(target_pointer_width = \"64\", cfg_attr(clippy, must_use))]\n\
            \n\
            ------------------------------\n\
            2 errors found in 1 rule\n\
            ------------------------------\n", full_run_footer(1, 1).as_str()]
            .concat()
            .as_str())
    );
    assert_eq!(outcome.status, HeisenbergExitStatus::Violations);
}
