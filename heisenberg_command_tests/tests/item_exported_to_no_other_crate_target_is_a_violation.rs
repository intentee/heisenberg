use std::path::Path;
use std::str;

use heisenberg_command::heisenberg_exit_status::HeisenbergExitStatus;
use heisenberg_command_tests::full_run_footer::full_run_footer;
use heisenberg_command_tests::heisenberg_check::HeisenbergCheck;
use heisenberg_command_tests::host_only_fixture::HostOnlyFixture;

#[test]
fn item_exported_to_no_other_crate_target_is_a_violation() {
    let outcome = HeisenbergCheck {
        workspace: &HostOnlyFixture {
            directory: Path::new(env!("CARGO_TARGET_TMPDIR")),
            name: "items_exported_to_no_user",
        }
        .path()
        .expect("the host-only fixture must be prepared"),
    }
    .run()
    .expect("the arguments must parse");

    assert_eq!(
        str::from_utf8(&outcome.output),
        Ok(["ItemMoreVisibleThanItsUses: 4 violations\n\
            An unpublishable package has every user inside the workspace, so an item exported to crates that never use it declares an interface that does not exist: it hides which items are the crate's surface, and keeps the compiler from reporting the item once nothing in its crate uses it either. Narrow the visibility to pub(crate), or remove the item when its own crate does not use it.\n\
            \n\
            - library/src/constants.rs:1:11: Constant is used by no other crate target, so its visibility reaches further than its uses\n    \
            pub const LIMIT: u8 = 3;\n\
            - library/src/idle.rs:4:14: Function is used by no other crate target, so its visibility reaches further than its uses\n    \
            pub const fn idle() -> u8 {\n\
            - library/src/report.rs:8:18: Function is used by no other crate target, so its visibility reaches further than its uses\n        \
            pub const fn doubled(&self) -> u8 {\n\
            - user/src/summary.rs:7:8: Function is used by no other crate target, so its visibility reaches further than its uses\n    \
            pub fn summary() -> u8 {\n\
            \n\
            ------------------------------\n\
            4 errors found in 1 rule\n\
            ------------------------------\n", full_run_footer(3, 10).as_str()]
            .concat()
            .as_str())
    );
    assert_eq!(outcome.status, HeisenbergExitStatus::Violations);
}
