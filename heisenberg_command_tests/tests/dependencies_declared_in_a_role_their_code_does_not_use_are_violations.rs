use std::path::Path;
use std::str;

use heisenberg_command::heisenberg_exit_status::HeisenbergExitStatus;
use heisenberg_command_tests::full_run_footer::full_run_footer;
use heisenberg_command_tests::heisenberg_check::HeisenbergCheck;
use heisenberg_command_tests::host_only_fixture::HostOnlyFixture;

#[test]
fn dependencies_declared_in_a_role_their_code_does_not_use_are_violations() {
    let outcome = HeisenbergCheck {
        workspace: &HostOnlyFixture {
            directory: Path::new(env!("CARGO_TARGET_TMPDIR")),
            name: "dependency_roles",
        }
        .path()
        .expect("the host-only fixture must be prepared"),
    }
    .run()
    .expect("the arguments must parse");

    assert_eq!(
        str::from_utf8(&outcome.output),
        Ok(["TestDependencyInProduction: 1 violation\n\
            A dependency declared for production is built into every user of the package, so one that only tests need states a production input that production does not have. Declare the dependency in dev-dependencies, or move the tests that need it into a test crate.\n\
            \n\
            - app/Cargo.toml: Dependency tested_with is referred to only by test, benchmark or example code\n\
            \n\
            TestLibraryInProductionPackage: 1 violation\n\
            A library that only tests need belongs with the tests, in a test crate: a production package that declares it mixes the inputs of its tests into its own manifest, and every package that tests the same way repeats it. Move the tests that need the library, with the dev-dependency, into a declared test crate.\n\
            \n\
            - app/Cargo.toml: Dev-dependency outside_checker brings a library from outside the workspace into the tests of a production package\n\
            \n\
            UnusedDependency: 3 violations\n\
            A declared dependency is an input of the build and of every analysis of the package, so one that no code uses states an input that does not exist. Remove the dependency from the manifest.\n\
            \n\
            - app/Cargo.toml: Dev-dependency idle is referred to by no code that the role serves\n\
            - app/Cargo.toml: Dependency spare is referred to by no code that the role serves\n\
            - app/Cargo.toml: Build-dependency unbuilt is referred to by no code that the role serves\n\
            \n\
            ------------------------------\n\
            5 errors found in 3 rules\n\
            ------------------------------\n", full_run_footer(11, 21).as_str()]
            .concat()
            .as_str())
    );
    assert_eq!(outcome.status, HeisenbergExitStatus::Violations);
}
