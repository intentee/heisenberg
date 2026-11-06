use std::str;

use heisenberg_command::heisenberg_exit_status::HeisenbergExitStatus;
use heisenberg_command_tests::fixture_workspace::FixtureWorkspace;
use heisenberg_command_tests::heisenberg_check::HeisenbergCheck;
use heisenberg_command_tests::host_footer::host_footer;

const CYCLE_ON_EVERY_TARGET: &str = "- src/first.rs:4:24: Parts of crate depend on each other\n        \
    pub fn second() -> crate::second::Second {\n      \
    crate::first -> crate::second:\n        \
    crate::first uses crate::second at src/first.rs:4:24\n        \
    crate::first uses crate::second at src/first.rs:5:9\n      \
    crate::second -> crate::first:\n        \
    crate::second uses crate::first at src/second.rs:4:23\n        \
    crate::second uses crate::first at src/second.rs:5:9\n";

#[cfg(all(target_arch = "x86_64", target_os = "linux", target_env = "gnu"))]
const HOST_CYCLES: &str = "ModuleCycle: 1 violation\n";

#[cfg(all(target_arch = "x86_64", target_os = "linux", target_env = "gnu"))]
const HOST_ONLY_CYCLE: &str = "";

#[cfg(all(target_arch = "x86_64", target_os = "linux", target_env = "gnu"))]
const HOST_ERRORS: &str = "1 errors found\n";

#[cfg(all(target_arch = "x86_64", target_os = "linux", target_env = "gnu"))]
const HOST_MODULES: usize = 3;

#[cfg(all(target_arch = "x86_64", target_os = "windows", target_env = "msvc"))]
const HOST_CYCLES: &str = "ModuleCycle: 2 violations\n";

#[cfg(all(target_arch = "x86_64", target_os = "windows", target_env = "msvc"))]
const HOST_ONLY_CYCLE: &str = "- src/fourth.rs:4:23: Parts of crate depend on each other\n        \
    pub fn third() -> crate::third::Third {\n      \
    crate::fourth -> crate::third:\n        \
    crate::fourth uses crate::third at src/fourth.rs:4:23\n        \
    crate::fourth uses crate::third at src/fourth.rs:5:9\n      \
    crate::third -> crate::fourth:\n        \
    crate::third uses crate::fourth at src/third.rs:4:24\n        \
    crate::third uses crate::fourth at src/third.rs:5:9\n";

#[cfg(all(target_arch = "x86_64", target_os = "windows", target_env = "msvc"))]
const HOST_ERRORS: &str = "2 errors found\n";

#[cfg(all(target_arch = "x86_64", target_os = "windows", target_env = "msvc"))]
const HOST_MODULES: usize = 5;

#[test]
fn default_run_analyses_the_host_target() {
    let outcome = HeisenbergCheck {
        workspace: &FixtureWorkspace {
            name: "cycles_on_every_target_and_on_one",
        }
        .path(),
    }
    .run()
    .expect("the arguments must parse");

    assert_eq!(
        str::from_utf8(&outcome.output),
        Ok(format!(
            "{HOST_CYCLES}\
             Modules that depend on each other cannot be understood or changed in isolation. Break the cycle so the modules form layers.\n\
             \n\
             {CYCLE_ON_EVERY_TARGET}\
             {HOST_ONLY_CYCLE}\
             \n\
             ------------------------------\n\
             {HOST_ERRORS}\
             ------------------------------\n\
             {}",
            host_footer(1, HOST_MODULES)
        )
        .as_str())
    );
    assert_eq!(outcome.status, HeisenbergExitStatus::Violations);
}
