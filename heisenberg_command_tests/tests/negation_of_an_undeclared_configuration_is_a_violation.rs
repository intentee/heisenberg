use std::path::Path;
use std::str;

use heisenberg_command::heisenberg_exit_status::HeisenbergExitStatus;
use heisenberg_command_tests::full_run_footer::full_run_footer;
use heisenberg_command_tests::heisenberg_check::HeisenbergCheck;
use heisenberg_command_tests::host_only_fixture::HostOnlyFixture;

#[test]
fn negation_of_an_undeclared_configuration_is_a_violation() {
    let outcome = HeisenbergCheck {
        workspace: &HostOnlyFixture {
            directory: Path::new(env!("CARGO_TARGET_TMPDIR")),
            name: "negations_of_undeclared_configurations",
        }
        .path()
        .expect("the host-only fixture must be prepared"),
    }
    .run()
    .expect("the arguments must parse");

    assert_eq!(
        str::from_utf8(&outcome.output),
        Ok(["VacuousNegatedConfiguration: 4 violations\n\
            A negation of a configuration that no declared configuration sets holds wherever Heisenberg analyses the code, so it only changes what a tool outside the declared configurations sees, and hides the code from that tool. Remove the negation.\n\
            \n\
            - src/lib.rs:4:5: Predicate negates rustfmt, which no declared configuration sets\n        \
            #[cfg_attr(not(rustfmt), must_use)]\n\
            - src/lib.rs:9:5: Predicate negates clippy, which no declared configuration sets\n        \
            #[cfg(any(not(target_pointer_width = \"64\"), not(clippy)))]\n\
            - src/lib.rs:12:5: Predicate negates target_os = \"none\", which no declared configuration sets\n        \
            #[cfg(not(target_os = \"none\"))]\n\
            - src/lib.rs:16:9: Predicate negates miri, which no declared configuration sets\n            \
            #[cfg(not(miri))]\n\
            \n\
            ------------------------------\n\
            4 errors found in 1 rule\n\
            ------------------------------\n", full_run_footer(1, 1).as_str()].concat().as_str())
    );
    assert_eq!(outcome.status, HeisenbergExitStatus::Violations);
}
