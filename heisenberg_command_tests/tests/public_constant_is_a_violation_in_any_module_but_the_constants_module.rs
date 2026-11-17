use std::str;

use heisenberg_command::heisenberg_exit_status::HeisenbergExitStatus;
use heisenberg_command_tests::fixture_workspace::FixtureWorkspace;
use heisenberg_command_tests::heisenberg_check::HeisenbergCheck;
use heisenberg_command_tests::nextest_presence_evaluation::NextestPresenceEvaluation;

#[test]
fn public_constant_is_a_violation_in_any_module_but_the_constants_module() {
    let outcome = HeisenbergCheck {
        workspace: &FixtureWorkspace {
            name: "public_constants_in_their_own_modules",
        }
        .path(),
    }
    .run()
    .expect("the arguments must parse");

    assert_eq!(
        str::from_utf8(&outcome.output),
        Ok([
            "PublicConstantOutsideConstantsModule: 5 violations\n\
            Public constants have a single home, crate::constants. Move the constant there.\n\
            \n\
            - src/fixture_directory.rs:1:11: Public constant belongs in crate::constants\n    \
            pub const FIXTURE_DIRECTORY: &str = env!(\"CARGO_MANIFEST_DIR\");\n\
            - src/group_type.rs:1:11: Public constant belongs in crate::constants\n    \
            pub const GROUP_TYPE_GROUP: &str = \"_tb_group\";\n\
            - src/group_type.rs:2:11: Public constant belongs in crate::constants\n    \
            pub const GROUP_TYPE_KEY: &str = \"_tb_type\";\n\
            - src/group_type.rs:3:11: Public constant belongs in crate::constants\n    \
            pub const GROUP_TYPE_LAYER: &str = \"_tb_layer\";\n\
            - src/threshold.rs:1:11: Public constant belongs in crate::constants\n    \
            pub const ALPHA_MASK_THRESHOLD: f32 = 0.5;\n\
            \n\
            ------------------------------\n\
            5 errors found in 1 rule\n\
            ------------------------------\n",
            NextestPresenceEvaluation::NotEvaluated
                .host_footer(1, 4)
                .as_str()
        ]
        .concat()
        .as_str())
    );
    assert_eq!(outcome.status, HeisenbergExitStatus::Violations);
}
