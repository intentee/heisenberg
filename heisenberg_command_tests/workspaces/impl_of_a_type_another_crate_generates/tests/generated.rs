generator::generated_type!(Generated);

impl Generated {
    fn describe() -> u8 {
        1
    }
}

#[test]
fn describes() {
    assert_eq!(Generated::describe(), 1);
}
