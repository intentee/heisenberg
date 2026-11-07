use std::path::Path;
use std::str;

use heisenberg_command::heisenberg_exit_status::HeisenbergExitStatus;
use heisenberg_command_tests::full_run_footer::full_run_footer;
use heisenberg_command_tests::heisenberg_check::HeisenbergCheck;
use heisenberg_command_tests::host_only_fixture::HostOnlyFixture;

#[cfg(all(target_arch = "x86_64", target_os = "linux", target_env = "gnu"))]
const OTHER_PLATFORM_MODULE: &str = "- src/lib.rs:5:9: Code is active in no declared configuration\n    \
    pub mod windows_only;\n";

#[cfg(all(target_arch = "x86_64", target_os = "linux", target_env = "gnu"))]
const NESTED_PLATFORM_CODE: &str = "";

#[cfg(all(target_arch = "x86_64", target_os = "linux", target_env = "gnu"))]
const HOST_MODULES: usize = 2;

#[cfg(all(target_arch = "x86_64", target_os = "windows", target_env = "msvc"))]
const OTHER_PLATFORM_MODULE: &str = "";

#[cfg(all(target_arch = "x86_64", target_os = "windows", target_env = "msvc"))]
const NESTED_PLATFORM_CODE: &str = "- src/windows_only.rs:5:12: Code is active in no declared configuration\n        \
    pub fn unix_inside_windows() {}\n";

#[cfg(all(target_arch = "x86_64", target_os = "windows", target_env = "msvc"))]
const HOST_MODULES: usize = 3;

#[test]
fn code_active_in_no_declared_configuration_is_a_violation() {
    let outcome = HeisenbergCheck {
        workspace: &HostOnlyFixture {
            directory: Path::new(env!("CARGO_TARGET_TMPDIR")),
            name: "code_active_in_no_configuration",
        }
        .path()
        .expect("the host-only fixture must be prepared"),
    }
    .run()
    .expect("the arguments must parse");

    assert_eq!(
        str::from_utf8(&outcome.output),
        Ok(format!(
            "CodeActiveInNoConfiguration: 6 violations\n\
             Code active in no declared configuration is never built or analysed. Remove it, declare a configuration it is active in, or make it active in a declared configuration.\n\
             \n\
             - src/lib.rs:2:9: Code is active in no declared configuration\n    \
             pub mod mac;\n\
             {OTHER_PLATFORM_MODULE}\
             - src/lib.rs:6:9: Code is active in no declared configuration\n    \
             pub mod configured_out;\n\
             - src/never.rs:4:16: Code is active in no declared configuration\n            \
             pub fn generated_never() {{}}\n\
             - src/never.rs:14:12: Code is active in no declared configuration\n        \
             pub fn unreachable() {{}}\n\
             - src/never.rs:18:9: Code is active in no declared configuration\n            \
             return;\n\
             {NESTED_PLATFORM_CODE}\
             \n\
             ------------------------------\n\
             6 errors found\n\
             ------------------------------\n\
             {footer}",
            footer = full_run_footer(1, HOST_MODULES)
        )
        .as_str())
    );
    assert_eq!(outcome.status, HeisenbergExitStatus::Violations);
}
