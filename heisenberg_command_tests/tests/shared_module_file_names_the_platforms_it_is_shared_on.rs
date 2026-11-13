use std::str;

use heisenberg_command::heisenberg_exit_status::HeisenbergExitStatus;
use heisenberg_command_tests::fixture_workspace::FixtureWorkspace;
use heisenberg_command_tests::heisenberg_check::HeisenbergCheck;
use heisenberg_command_tests::nextest_presence_evaluation::NextestPresenceEvaluation;

#[cfg(all(target_arch = "x86_64", target_os = "linux", target_env = "gnu"))]
const WINDOWS_SHARED_FILES: &str = "";

#[cfg(all(target_arch = "x86_64", target_os = "linux", target_env = "gnu"))]
const SHARED_FILES: &str = "1 violation";

#[cfg(all(target_arch = "x86_64", target_os = "linux", target_env = "gnu"))]
const ERRORS: usize = 1;

#[cfg(all(target_arch = "x86_64", target_os = "linux", target_env = "gnu"))]
const HOST_MODULES: usize = 6;

#[cfg(all(target_arch = "x86_64", target_os = "windows", target_env = "msvc"))]
const WINDOWS_SHARED_FILES: &str = "- tests/gated/mod.rs: Module file belongs to 2 modules\n      \
    crate::gated in module_file_shared_on_some_platforms example \"demo\"\n      \
    crate::gated in module_file_shared_on_some_platforms test \"first\"\n\
    - tests/platform/mod.rs: Module file belongs to 2 modules\n      \
    crate::platform in module_file_shared_on_some_platforms example \"demo\"\n      \
    crate::platform in module_file_shared_on_some_platforms test \"first\"\n";

#[cfg(all(target_arch = "x86_64", target_os = "windows", target_env = "msvc"))]
const SHARED_FILES: &str = "3 violations";

#[cfg(all(target_arch = "x86_64", target_os = "windows", target_env = "msvc"))]
const ERRORS: usize = 3;

#[cfg(all(target_arch = "x86_64", target_os = "windows", target_env = "msvc"))]
const HOST_MODULES: usize = 9;

#[test]
fn shared_module_file_names_the_platforms_it_is_shared_on() {
    let outcome = HeisenbergCheck {
        workspace: &FixtureWorkspace {
            name: "module_file_shared_on_some_platforms",
        }
        .path(),
    }
    .run()
    .expect("the arguments must parse");

    assert_eq!(
        str::from_utf8(&outcome.output),
        Ok(format!(
            "SharedModuleFile: {SHARED_FILES}\n\
             A file included as several modules has no single owner. Give each module its own file.\n\
             \n\
             {WINDOWS_SHARED_FILES}\
             - tests/support/mod.rs: Module file belongs to 2 modules\n      \
             crate::support in module_file_shared_on_some_platforms example \"demo\"\n      \
             crate::support in module_file_shared_on_some_platforms test \"first\"\n\
             \n\
             ------------------------------\n\
             {ERRORS} errors found\n\
             ------------------------------\n\
             {}",
            NextestPresenceEvaluation::Evaluated.host_footer(3, HOST_MODULES)
        )
        .as_str())
    );
    assert_eq!(outcome.status, HeisenbergExitStatus::Violations);
}
