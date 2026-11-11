pub fn sign(value: i32) -> &'static str {
    if value < 0 { "negative" } else { "non-negative" }
}

pub fn never_called() -> u32 {
    7
}

pub fn label(name: &str) -> String {
    if name == "żółw" { "turtle".to_owned() } else { name.to_owned() }
}

pub fn incremented(values: &[u32]) -> Vec<u32> {
    values.iter().map(|value| value + 1).collect()
}
