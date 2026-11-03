use std::str;

use heisenberg_command::heisenberg_exit_status::HeisenbergExitStatus;
use heisenberg_command_tests::fixture_workspace::FixtureWorkspace;
use heisenberg_command_tests::heisenberg_check::HeisenbergCheck;

#[test]
fn conversion_from_a_local_type_belongs_with_that_type() {
    let outcome = HeisenbergCheck {
        workspace: &FixtureWorkspace {
            name: "conversions_from_local_types",
        }
        .path(),
    }
    .run()
    .expect("the arguments must parse");

    assert_eq!(
        str::from_utf8(&outcome.output),
        Ok("MisplacedImpl: 1 violation\n\
            All behavior of a type is read in one place, the file that defines it. Move the impl block into that file.\n\
            \n\
            - src/conversions.rs:3:31: Impl block belongs in src/sequence_length.rs\n    \
            impl From<SequenceLength> for u64 {\n\
            \n\
            ------------------------------\n\
            1 errors found\n\
            ------------------------------\n\
            Analysed on x86_64-pc-windows-msvc: 1 crate targets, 4 modules\n\
            Analysed on x86_64-unknown-linux-gnu: 1 crate targets, 4 modules\n\
            Precondition: rustc 1.99.1 compiles the workspace for every declared target\n")
    );
    assert_eq!(outcome.status, HeisenbergExitStatus::Violations);
}
