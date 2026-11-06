use std::str;

use heisenberg_command::heisenberg_exit_status::HeisenbergExitStatus;
use heisenberg_command_tests::fixture_workspace::FixtureWorkspace;
use heisenberg_command_tests::heisenberg_check::HeisenbergCheck;
use heisenberg_command_tests::host_footer::host_footer;

#[cfg(all(target_arch = "x86_64", target_os = "linux", target_env = "gnu"))]
const WINDOWS_ONLY_CYCLE: &str = "";

#[cfg(all(target_arch = "x86_64", target_os = "linux", target_env = "gnu"))]
const ERRORS: &str = "No errors found!\n";

#[cfg(all(target_arch = "x86_64", target_os = "linux", target_env = "gnu"))]
const STATUS: HeisenbergExitStatus = HeisenbergExitStatus::Compliant;

#[cfg(all(target_arch = "x86_64", target_os = "windows", target_env = "msvc"))]
const WINDOWS_ONLY_CYCLE: &str = "ModuleCycle: 1 violation\n\
            Modules that depend on each other cannot be understood or changed in isolation. Break the cycle so the modules form layers.\n\
            \n\
            - src/first.rs:2:5: Parts of crate depend on each other\n    \
            use crate::second::Second;\n      \
            crate::first -> crate::second:\n        \
            crate::first uses crate::second at src/first.rs:2:5\n      \
            crate::second -> crate::first:\n        \
            crate::second uses crate::first at src/second.rs:1:5\n        \
            crate::second uses crate::first at src/second.rs:4:16\n\
            \n\
";

#[cfg(all(target_arch = "x86_64", target_os = "windows", target_env = "msvc"))]
const ERRORS: &str = "1 errors found\n";

#[cfg(all(target_arch = "x86_64", target_os = "windows", target_env = "msvc"))]
const STATUS: HeisenbergExitStatus = HeisenbergExitStatus::Violations;

#[test]
fn import_disabled_by_cfg_creates_no_module_dependency() {
    let outcome = HeisenbergCheck {
        workspace: &FixtureWorkspace {
            name: "cfg_disabled_import_of_a_sibling",
        }
        .path(),
    }
    .run()
    .expect("the arguments must parse");

    assert_eq!(
        str::from_utf8(&outcome.output),
        Ok(format!(
            "{WINDOWS_ONLY_CYCLE}\
             ------------------------------\n\
             {ERRORS}\
             ------------------------------\n\
             {}",
            host_footer(1, 3)
        )
        .as_str())
    );
    assert_eq!(outcome.status, STATUS);
}
