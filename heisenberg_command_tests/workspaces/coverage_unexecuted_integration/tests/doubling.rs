#[test]
fn doubles_two() {
    let doubled = coverage_unexecuted_integration::doubled(2);

    if doubled != 4 {
        panic!("two doubles to {doubled}");
    }
}
