use std::str;

use heisenberg_command::heisenberg_exit_status::HeisenbergExitStatus;
use heisenberg_command_tests::fixture_workspace::FixtureWorkspace;
use heisenberg_command_tests::heisenberg_check::HeisenbergCheck;
use heisenberg_command_tests::nextest_presence_evaluation::NextestPresenceEvaluation;

#[cfg(all(target_arch = "x86_64", target_os = "linux", target_env = "gnu"))]
const PLATFORM_IMPORT: &str = "- src/report.rs:7:5: Item belongs before Report: imports from vendor crates precede public items\n    \
    use std::os::unix::ffi::OsStrExt;\n";

#[cfg(all(target_arch = "x86_64", target_os = "windows", target_env = "msvc"))]
const PLATFORM_IMPORT: &str = "- src/report.rs:4:5: Item belongs before Report: imports from vendor crates precede public items\n    \
    use std::os::windows::ffi::OsStrExt;\n";

#[test]
fn import_out_of_order_on_one_platform_is_reported_for_that_platform() {
    let outcome = HeisenbergCheck {
        workspace: &FixtureWorkspace {
            name: "import_out_of_order_on_windows",
        }
        .path(),
    }
    .run()
    .expect("the arguments must parse");

    assert_eq!(
        str::from_utf8(&outcome.output),
        Ok(format!(
            "ItemOutOfOrder: 1 violation\n\
             Every file reads in the same order, from what it declares and imports down to what it exposes. Move the item before the item the message names.\n\
             \n\
             {PLATFORM_IMPORT}\
             \n\
             ------------------------------\n\
             1 errors found\n\
             ------------------------------\n\
             {}",
            NextestPresenceEvaluation::NotEvaluated.host_footer(1, 2)
        )
        .as_str())
    );
    assert_eq!(outcome.status, HeisenbergExitStatus::Violations);
}
