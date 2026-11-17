use std::str;

use heisenberg_command::heisenberg_exit_status::HeisenbergExitStatus;
use heisenberg_command_tests::fixture_workspace::FixtureWorkspace;
use heisenberg_command_tests::heisenberg_check::HeisenbergCheck;
use heisenberg_command_tests::nextest_presence_evaluation::NextestPresenceEvaluation;

#[cfg(all(target_arch = "x86_64", target_os = "linux", target_env = "gnu"))]
const INCLUSIONS: &str = "- src/shared.rs: Module file belongs to 2 modules\n      \
    crate::second in module_file_analysed_differently_per_target lib\n      \
    crate::third in module_file_analysed_differently_per_target lib, not analysed: the crate already analyses the file as crate::second\n";

#[cfg(all(target_arch = "x86_64", target_os = "linux", target_env = "gnu"))]
const HOST_MODULES: usize = 3;

#[cfg(all(target_arch = "x86_64", target_os = "windows", target_env = "msvc"))]
const INCLUSIONS: &str = "- src/shared.rs: Module file belongs to 3 modules\n      \
    crate::first in module_file_analysed_differently_per_target lib\n      \
    crate::second in module_file_analysed_differently_per_target lib, not analysed: the crate already analyses the file as crate::first\n      \
    crate::third in module_file_analysed_differently_per_target lib, not analysed: the crate already analyses the file as crate::first\n";

#[cfg(all(target_arch = "x86_64", target_os = "windows", target_env = "msvc"))]
const HOST_MODULES: usize = 4;

#[test]
fn module_analysed_differently_per_platform_is_listed_once() {
    let outcome = HeisenbergCheck {
        workspace: &FixtureWorkspace {
            name: "module_file_analysed_differently_per_target",
        }
        .path(),
    }
    .run()
    .expect("the arguments must parse");

    assert_eq!(
        str::from_utf8(&outcome.output),
        Ok(format!(
            "SharedModuleFile: 1 violation\n\
             A file included as several modules has no single owner. Give each module its own file.\n\
             \n\
             {INCLUSIONS}\
             \n\
             ------------------------------\n\
             1 errors found in 1 rule\n\
             ------------------------------\n\
             {}",
            NextestPresenceEvaluation::NotEvaluated.host_footer(1, HOST_MODULES)
        )
        .as_str())
    );
    assert_eq!(outcome.status, HeisenbergExitStatus::Violations);
}
