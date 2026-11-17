use std::path::Path;
use std::str;

use heisenberg_command::heisenberg_exit_status::HeisenbergExitStatus;
use heisenberg_command_tests::constants::CONFIGURATION_ARGUMENT;
use heisenberg_command_tests::constants::FULL_RUN_FOOTER_END;
use heisenberg_command_tests::constants::HOST_CONFIGURATION;
use heisenberg_command_tests::heisenberg_check::HeisenbergCheck;
use heisenberg_command_tests::host_only_fixture::HostOnlyFixture;

const EXPORTED_ITEMS: &str = "ItemMoreVisibleThanItsUses: 10 violations\n\
            An unpublishable package has every user inside the workspace, so an item exported to crates that never use it declares an interface that does not exist: it hides which items are the crate's surface, and keeps the compiler from reporting the item once nothing in its crate uses it either. Narrow the visibility to pub(crate), or remove the item when its own crate does not use it.\n\
            \n\
            - builder/src/lib.rs:1:14: Function is used by no other crate target, so its visibility reaches further than its uses\n    \
            pub const fn builder() -> u8 {\n\
            - consumer/src/lib.rs:1:14: Function is used by no other crate target, so its visibility reaches further than its uses\n    \
            pub const fn consumer() -> u8 {\n\
            - dependent/src/lib.rs:1:14: Function is used by no other crate target, so its visibility reaches further than its uses\n    \
            pub const fn dependent() -> u8 {\n\
            - enabler/src/lib.rs:1:14: Function is used by no other crate target, so its visibility reaches further than its uses\n    \
            pub const fn enabler() -> u8 {\n\
            - forked/src/lib.rs:1:14: Function is used by no other crate target, so its visibility reaches further than its uses\n    \
            pub const fn forked() -> u8 {\n\
            - harness/src/lib.rs:1:14: Function is used by no other crate target, so its visibility reaches further than its uses\n    \
            pub const fn harness() -> u8 {\n\
            - idle/src/lib.rs:1:14: Function is used by no other crate target, so its visibility reaches further than its uses\n    \
            pub const fn idle() -> u8 {\n\
            - scriptor/src/lib.rs:1:14: Function is used by no other crate target, so its visibility reaches further than its uses\n    \
            pub const fn scriptor() -> u8 {\n\
            - stranger/src/lib.rs:1:14: Function is used by no other crate target, so its visibility reaches further than its uses\n    \
            pub const fn stranger() -> u8 {\n\
            - unlisted/src/lib.rs:1:14: Function is used by no other crate target, so its visibility reaches further than its uses\n    \
            pub const fn unlisted() -> u8 {\n\
            \n\
            ";

fn expected_report() -> String {
    format!(
        "ConfinedPackageReachedOutsideItsUsers: 7 violations\n\
            A confined package is built and used only by its declared users, so nothing else reaches what it provides, even through another member. Remove the dependency that the path names; code that needs the package belongs in one of its users.\n\
            \n\
            - both/Cargo.toml: Package both depends on confined package confined without being one of its users\n      \
            both v0.1.0 -> confined v0.1.0\n\
            - builder/Cargo.toml: Package builder depends on confined package confined without being one of its users\n      \
            builder v0.1.0 -> both v0.1.0 -> wrapper v0.1.0 -> confined v0.1.0\n\
            - builder/Cargo.toml: Package builder depends on confined package x without being one of its users\n      \
            builder v0.1.0 -> scripted v0.1.0 -> x v0.1.0\n\
            - enabler/Cargo.toml: Package enabler depends on confined package confined without being one of its users\n      \
            enabler v0.1.0 -> both v0.1.0 -> confined v0.1.0\n\
            - forked/Cargo.toml: Package forked depends on confined package confined without being one of its users\n      \
            forked v0.1.0 -> wrapper v0.1.0 -> confined v0.1.0\n\
            - stranger/Cargo.toml: Package stranger depends on confined package confined without being one of its users (only on {HOST_CONFIGURATION})\n      \
            stranger v0.1.0 -> user v0.1.0 -> wrapper v0.1.0 -> confined v0.1.0\n\
            - unlisted/Cargo.toml: Package unlisted depends on confined package confined without being one of its users\n      \
            unlisted v0.1.0 -> confined v0.1.0\n\
            \n\
            ConfinedPackageUserListedTwice: 1 violation\n\
            Each user of a confined package is listed once. Remove the repeated entry from confined_packages.\n\
            \n\
            - Cargo.toml: Package user is listed more than once as a user of confined package confined\n\
            \n\
            ConfinedPackageUserNotAMember: 1 violation\n\
            Only a workspace member can use a confined package. Remove the entry from confined_packages, or add the package to the workspace.\n\
            \n\
            - Cargo.toml: Package missing is listed as a user of confined package confined but is not a workspace member\n\
            \n\
            ConfinedPackageUserNotReachingIt: 1 violation\n\
            The users of a confined package are exactly the members that reach it, so the list states where the package is used. Remove the member from the users of the package.\n\
            \n\
            - Cargo.toml: Package idle is listed as a user of confined package confined but depends on it in no declared configuration\n\
            \n\
            {EXPORTED_ITEMS}\
            PackageDependencyCycle: 1 violation\n\
            Package dependencies never form a cycle, dev-dependencies included. When a test library depends on the package whose tests use it, move those tests and the helpers that need the package into a separate tests package that holds them in its library.\n\
            \n\
            - cyclic/Cargo.toml: Packages cyclic v0.1.0, harness v0.1.0 depend on each other through dev-dependencies\n\
            \n\
            PathPackageOutsideWorkspace: 8 violations\n\
            Only workspace members are analysed. A package built from local sources that is not a member is code the project owns and compiles, yet no rule reads it. Make the package a member of the workspace, or depend on a release of it from a registry or on a git revision.\n\
            \n\
            - Cargo.toml: Package confined v0.1.0 is built from vendor/confined/Cargo.toml, which is not a workspace member, so its code is not analysed\n\
            - Cargo.toml: Package d v0.1.0 is built from vendor/d/Cargo.toml, which is not a workspace member, so its code is not analysed\n\
            - Cargo.toml: Package p v0.1.0 is built from vendor/p/Cargo.toml, which is not a workspace member, so its code is not analysed\n\
            - Cargo.toml: Package q v0.1.0 is built from vendor/q/Cargo.toml, which is not a workspace member, so its code is not analysed\n\
            - Cargo.toml: Package scripted v0.1.0 is built from vendor/scripted/Cargo.toml, which is not a workspace member, so its code is not analysed\n\
            - Cargo.toml: Package x v0.1.0 is built from vendor/x/Cargo.toml, which is not a workspace member, so its code is not analysed\n\
            - Cargo.toml: Package y v0.1.0 is built from vendor/y/Cargo.toml, which is not a workspace member, so its code is not analysed\n\
            - Cargo.toml: Package z v0.1.0 is built from vendor/z/Cargo.toml, which is not a workspace member, so its code is not analysed\n\
            \n\
            TestLibraryInProductionPackage: 3 violations\n\
            A library that only tests need belongs with the tests, in a test crate: a production package that declares it mixes the inputs of its tests into its own manifest, and every package that tests the same way repeats it. Move the tests that need the library, with the dev-dependency, into a declared test crate.\n\
            \n\
            - tested/Cargo.toml: Dev-dependency x brings a library from outside the workspace into the tests of a production package\n\
            - tool/Cargo.toml: Dev-dependency d brings a library from outside the workspace into the tests of a production package\n\
            - unlisted/Cargo.toml: Dev-dependency confined brings a library from outside the workspace into the tests of a production package\n\
            \n\
            UnknownConfinedPackage: 1 violation\n\
            A confinement holds only for a package that the workspace resolves. Remove the entry from confined_packages, or correct the package name.\n\
            \n\
            - Cargo.toml: Confined package absent names no package that the workspace resolves\n\
            \n\
            UnusedDependency: 7 violations\n\
            A declared dependency is an input of the build and of every analysis of the package, so one that no code uses states an input that does not exist. Remove the dependency from the manifest.\n\
            \n\
            - both/Cargo.toml: Dependency confined is referred to by no code that the role serves\n\
            - both/Cargo.toml: Dependency wrapper is referred to by no code that the role serves\n\
            - cyclic/Cargo.toml: Dev-dependency harness is referred to by no code that the role serves\n\
            - forked/Cargo.toml: Dependency wrapper is referred to by no code that the role serves\n\
            - stranger/Cargo.toml: Dependency user is referred to by no code that the role serves\n\
            - tested/Cargo.toml: Dev-dependency q is referred to by no code that the role serves\n\
            - tested/Cargo.toml: Dependency x is referred to by no code that the role serves\n\
            \n\
            ------------------------------\n\
            40 errors found in 10 rules\n\
            ------------------------------\n\
            Analysed on {HOST_CONFIGURATION}: 19 crate targets, 22 modules\n\
            Analysed on {HOST_CONFIGURATION}-without-features: 19 crate targets, 22 modules\n\
            {FULL_RUN_FOOTER_END}"
    )
}

#[test]
fn confined_packages_declaration_that_does_not_hold_is_a_violation() {
    let outcome = HeisenbergCheck {
        workspace: &HostOnlyFixture {
            directory: Path::new(env!("CARGO_TARGET_TMPDIR")),
            name: "confined_packages",
        }
        .path()
        .expect("the host-only fixture must be prepared"),
    }
    .run_with_arguments(&[
        CONFIGURATION_ARGUMENT,
        HOST_CONFIGURATION,
        CONFIGURATION_ARGUMENT,
        &format!("{HOST_CONFIGURATION}-without-features"),
    ])
    .expect("the arguments must parse");

    assert_eq!(
        str::from_utf8(&outcome.output),
        Ok(expected_report().as_str())
    );
    assert_eq!(outcome.status, HeisenbergExitStatus::Violations);
}
