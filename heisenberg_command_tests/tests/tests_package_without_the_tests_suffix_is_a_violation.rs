use std::str;

use heisenberg_command::heisenberg_exit_status::HeisenbergExitStatus;
use heisenberg_command_tests::heisenberg_check::HeisenbergCheck;
use heisenberg_fixtures::fixture_workspace::FixtureWorkspace;

#[test]
fn tests_package_without_the_tests_suffix_is_a_violation() {
    let outcome = HeisenbergCheck {
        workspace: &FixtureWorkspace {
            name: "tests_package_without_the_tests_suffix",
        }
        .path(),
    }
    .run()
    .expect("the arguments must parse");

    assert_eq!(
        str::from_utf8(&outcome.output),
        Ok("TestsOnlyPackageWithoutTestLibraries: 1 violation\n\
            Tests belong to the package whose behaviour they verify, in its tests directory. A separate tests package is justified only by test libraries of its own: shared helpers, which never live in a tests directory, and helpers that need the package under test, which that package cannot dev-depend on without a dependency cycle. Heisenberg cannot tell from the code which package the tests verify, so the packages listed could take the tests without a cycle, but the right owner is the package they verify. If a test library they use depends on that package, the test library is entangled with the code it helps to test: move the helpers that need the package into this package's library, or make the test library independent of it.\n\
            \n\
            - library_checks/Cargo.toml: Package library_checks holds only tests and no test library of its own, so the package its tests verify must own them\n      \
            could move without a dependency cycle to library\n\
            \n\
            TestsPackageWithoutTestsSuffix: 1 violation\n\
            A package that holds tests says so in its name, so people and tools tell test crates from production crates. Rename the package to end in _tests.\n\
            \n\
            - library_checks/Cargo.toml: Package library_checks holds only tests but its name does not end in _tests\n\
            \n\
            ------------------------------\n\
            2 errors found\n\
            ------------------------------\n\
            Analysed on x86_64-pc-windows-msvc: 2 crate targets, 2 modules\n\
            Analysed on x86_64-unknown-linux-gnu: 2 crate targets, 2 modules\n\
            Precondition: rustc 1.99.1 compiles the workspace for every declared target\n")
    );
    assert_eq!(outcome.status, HeisenbergExitStatus::Violations);
}
