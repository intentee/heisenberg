macro_rules! generated_type {
    () => {
        struct Generated;
    };
}

generated_type!();

impl Generated {
    fn generated() {}
}

#[test]
fn generates() {}
