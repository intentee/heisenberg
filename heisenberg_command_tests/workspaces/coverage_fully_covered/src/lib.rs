pub fn parity(value: u32) -> &'static str {
    if value % 2 == 0 { "even" } else { "odd" }
}

pub fn largest<TValue: PartialOrd>(first: TValue, second: TValue) -> TValue {
    if first > second { first } else { second }
}
