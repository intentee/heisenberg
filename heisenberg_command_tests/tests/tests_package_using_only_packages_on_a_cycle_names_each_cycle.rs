use std::path::Path;
use std::str;

use heisenberg_command::heisenberg_exit_status::HeisenbergExitStatus;
use heisenberg_command_tests::full_run_footer::full_run_footer;
use heisenberg_command_tests::heisenberg_check::HeisenbergCheck;
use heisenberg_command_tests::host_only_fixture::HostOnlyFixture;

#[test]
fn tests_package_using_only_packages_on_a_cycle_names_each_cycle() {
    let outcome = HeisenbergCheck {
        workspace: &HostOnlyFixture {
            directory: Path::new(env!("CARGO_TARGET_TMPDIR")),
            name: "tests_package_closing_a_cycle",
        }
        .path()
        .expect("the host-only fixture must be prepared"),
    }
    .run()
    .expect("the arguments must parse");

    assert_eq!(
        str::from_utf8(&outcome.output),
        Ok(["PackageDependencyCycle: 1 violation\n\
            Package dependencies never form a cycle, dev-dependencies included. When a test library depends on the package whose tests use it, move those tests and the helpers that need the package into a separate tests package that holds them in its library.\n\
            \n\
            - harness/Cargo.toml: Packages harness v0.1.0, library v0.1.0 depend on each other through dev-dependencies\n\
            \n\
            TestsOnlyPackageWithoutTestLibraries: 1 violation\n\
            Tests belong to the package whose behaviour they verify, in its tests directory. A separate tests package is justified only by test libraries of its own: shared helpers, which never live in a tests directory, and helpers that need the package under test, which that package cannot dev-depend on without a dependency cycle. Heisenberg cannot tell from the code which package the tests verify, so it lists every workspace member the tests use, either as a package that could take them or with the dependency cycle that moving them there would close; the right owner is the package they verify. If a test library they use depends on that package, the test library is entangled with the code it helps to test: move the helpers that need the package into this package's library, or make the test library independent of it.\n\
            \n\
            - library_tests/Cargo.toml: Package library_tests holds only tests and no test library of its own, so the package its tests verify must own them\n      \
            cannot move to harness: harness v0.1.0 -> library v0.1.0 -> harness v0.1.0\n      \
            cannot move to library: library v0.1.0 -> harness v0.1.0 -> library v0.1.0\n\
            \n\
            ------------------------------\n\
            2 errors found in 2 rules\n\
            ------------------------------\n", full_run_footer(4, 4).as_str()].concat().as_str())
    );
    assert_eq!(outcome.status, HeisenbergExitStatus::Violations);
}
