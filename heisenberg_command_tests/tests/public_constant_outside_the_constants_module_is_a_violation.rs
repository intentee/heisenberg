use std::str;

use heisenberg_command::heisenberg_exit_status::HeisenbergExitStatus;
use heisenberg_fixtures::fixture_workspace::FixtureWorkspace;
use heisenberg_test_harness::heisenberg_check::HeisenbergCheck;

#[test]
fn public_constant_outside_the_constants_module_is_a_violation() {
    let outcome = HeisenbergCheck {
        workspace: &FixtureWorkspace {
            name: "constant_outside_the_constants_module",
        }
        .path(),
    }
    .run()
    .expect("the arguments must parse");

    assert_eq!(
        str::from_utf8(&outcome.output),
        Ok(
            "ERROR: PublicConstantOutsideConstantsModule - src/constants.rs:12:5 - Public constant belongs in crate::constants\n    \
            pub const DEEP: u8 = 5;\n\
            \n\
            ERROR: PublicConstantOutsideConstantsModule - src/limits.rs:3:1 - Public constant belongs in crate::constants\n\
            pub const MAXIMUM: u8 = 9;\n\
            \n\
            ERROR: PublicConstantOutsideConstantsModule - src/limits.rs:5:1 - Public constant belongs in crate::constants\n\
            pub(crate) const SHARED: u8 = 4;\n\
            \n\
            ERROR: PublicConstantOutsideConstantsModule - src/limits.rs:10:5 - Public constant belongs in crate::constants\n    \
            pub const MINIMUM: u8 = 1;\n\
            \n\
            ERROR: PublicConstantOutsideConstantsModule - src/limits.rs:15:9 - Public constant belongs in crate::constants\n        \
            pub const INNER: u8 = 3;\n\
            \n\
            ERROR: PublicConstantOutsideConstantsModule - src/settings.rs:4:5 - Public constant belongs in crate::constants\n    \
            pub const DEFAULT: u8 = 6;\n\
            \n\
            ------------------------------\n\
            6 errors found\n\
            ------------------------------\n\
            Analysed on x86_64-pc-windows-msvc: 1 crate targets, 7 modules\n\
            Analysed on x86_64-unknown-linux-gnu: 1 crate targets, 7 modules\n\
            Precondition: rustc 1.99.1 compiles the workspace for every supported target\n"
        )
    );
    assert_eq!(outcome.status, HeisenbergExitStatus::Violations);
}
