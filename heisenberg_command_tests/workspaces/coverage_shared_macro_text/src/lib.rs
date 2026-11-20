macro_rules! threshold {
    ($name:ident, $limit:expr) => {
        pub fn $name(value: u32) -> &'static str {
            if value > $limit { "above" } else { "within" }
        }
    };
}

threshold!(above_ten, 10);
threshold!(above_twenty, 20);
