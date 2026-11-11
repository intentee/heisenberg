pub struct Meter;

#[automatically_derived]
impl Clone for Meter {
    fn clone(&self) -> Self {
        Self
    }
}

#[cfg(not(coverage))]
pub fn uninstrumented() -> u32 {
    4
}

pub fn instrumented() -> u32 {
    5
}
