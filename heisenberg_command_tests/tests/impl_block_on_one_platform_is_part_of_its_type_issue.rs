use std::str;

use heisenberg_command::heisenberg_exit_status::HeisenbergExitStatus;
use heisenberg_command_tests::fixture_workspace::FixtureWorkspace;
use heisenberg_command_tests::heisenberg_check::HeisenbergCheck;
use heisenberg_command_tests::nextest_presence_evaluation::NextestPresenceEvaluation;

#[cfg(all(target_arch = "x86_64", target_os = "linux", target_env = "gnu"))]
const UNIX_IMPL_BLOCK: &str = "      impl block at tests/recording.rs:4:6\n";

#[cfg(all(target_arch = "x86_64", target_os = "windows", target_env = "msvc"))]
const UNIX_IMPL_BLOCK: &str = "";

#[test]
fn impl_block_on_one_platform_is_part_of_its_type_issue() {
    let outcome = HeisenbergCheck {
        workspace: &FixtureWorkspace {
            name: "impl_block_of_a_flagged_type_on_one_platform",
        }
        .path(),
    }
    .run()
    .expect("the arguments must parse");

    assert_eq!(
        str::from_utf8(&outcome.output),
        Ok(format!(
            "NonTestItemInTestTarget: 1 violation\n\
             A helper used beyond its own test file is a test library, and test libraries never live in a tests directory. Move it into a library crate; when that crate depends on the package under test, move the tests that use it there too, so package dependencies never form a cycle.\n\
             \n\
             - tests/recording.rs:1:12: Integration test target holds an item that is neither a test, a constant, an import nor a private helper used only in its own file\n    \
             pub struct Recorder;\n\
             {UNIX_IMPL_BLOCK}\
             \n\
             ------------------------------\n\
             1 errors found in 1 rule\n\
             ------------------------------\n\
             {}",
            NextestPresenceEvaluation::Evaluated.host_footer(2, 2)
        )
        .as_str())
    );
    assert_eq!(outcome.status, HeisenbergExitStatus::Violations);
}
