use std::process::Command;

#[test]
fn parity_tells_even_from_odd() {
    assert_eq!(coverage_fully_covered::parity(2), "even");
    assert_eq!(coverage_fully_covered::parity(3), "odd");
}

#[test]
fn largest_picks_either_argument() {
    assert_eq!(coverage_fully_covered::largest(3, 2), 3);
    assert_eq!(coverage_fully_covered::largest(2, 3), 3);
}

#[test]
fn binary_prints_the_parity_of_its_argument() {
    let output = Command::new(env!("CARGO_BIN_EXE_coverage_fully_covered"))
        .arg("4")
        .output()
        .unwrap();

    assert_eq!(output.stdout, b"even\n");
}
