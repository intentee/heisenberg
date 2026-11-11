pub fn measured() -> u32 {
    1
}

#[cfg(coverage)]
pub fn only_under_coverage() -> u32 {
    2
}
