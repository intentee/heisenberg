identity::identity! {
    pub fn second() -> u8 {
        let pair = ((1, 2), 3);

        pair.0.1
    }
}

pub struct Nested;
