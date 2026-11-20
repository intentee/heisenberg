#[test]
fn doubles_two() {
    let doubled = super::doubled(2);

    if doubled != 4 {
        panic!("two doubles to {doubled}");
    }
}
