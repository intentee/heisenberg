use std::str;

use heisenberg_command::heisenberg_exit_status::HeisenbergExitStatus;
use heisenberg_command_tests::heisenberg_check::HeisenbergCheck;
use heisenberg_fixtures::fixture_workspace::FixtureWorkspace;

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
        Ok("PublicConstantOutsideConstantsModule: 5 violations\n\
            Public constants have a single home, crate::constants. Move the constant there.\n\
            \n\
            - src/constants.rs:12:15: Public constant belongs in crate::constants\n        \
            pub const DEEP: u8 = 5;\n\
            - src/limits.rs:3:11: Public constant belongs in crate::constants\n    \
            pub const MAXIMUM: u8 = 9;\n\
            - src/limits.rs:5:18: Public constant belongs in crate::constants\n    \
            pub(crate) const SHARED: u8 = 4;\n\
            - src/limits.rs:13:19: Public constant belongs in crate::constants\n            \
            pub const INNER: u8 = 3;\n\
            - src/settings.rs:4:15: Public constant belongs in crate::constants\n        \
            pub const DEFAULT: u8 = 6;\n\
            \n\
            ------------------------------\n\
            5 errors found\n\
            ------------------------------\n\
            Analysed on x86_64-pc-windows-msvc: 1 crate targets, 7 modules\n\
            Analysed on x86_64-unknown-linux-gnu: 1 crate targets, 7 modules\n\
            Precondition: rustc 1.99.1 compiles the workspace for every declared target\n")
    );
    assert_eq!(outcome.status, HeisenbergExitStatus::Violations);
}
