#[test]
fn binary_and_temporary_directory_are_named_by_cargo() {
    assert_ne!(
        env!("CARGO_BIN_EXE_undeclared_environment_variables"),
        env!("CARGO_TARGET_TMPDIR")
    );
}
