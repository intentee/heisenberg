#[test]
fn binary_path_is_known() {
    let _binary = env!("CARGO_BIN_EXE_undeclared_environment_variables");
    let _temporary = env!("CARGO_TARGET_TMPDIR");
}
