mod rounding;

pub fn rounded(value: u32) -> u32 {
    rounding::to_ten(value)
}
