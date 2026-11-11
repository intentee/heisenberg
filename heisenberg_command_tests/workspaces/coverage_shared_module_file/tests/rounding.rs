use std::process::Command;

#[test]
fn rounds_down_to_ten() {
    assert_eq!(coverage_shared_module_file::rounded(47), 40);
}

#[test]
fn binary_prints_the_rounded_value() {
    let output = Command::new(env!("CARGO_BIN_EXE_coverage_shared_module_file"))
        .output()
        .unwrap();

    assert_eq!(output.stdout, b"40\n");
}
