macro_rules! generated_type {
    () => {
        struct Generated;
    };
}

generated_type!();

#[test]
fn generates() {}

impl Generated {
    fn generated() {}
}
