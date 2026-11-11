#[path = "../shared/clamp.rs"]
mod clamp;

pub fn bounded(value: u32) -> u32 {
    clamp::clamp(value)
}
