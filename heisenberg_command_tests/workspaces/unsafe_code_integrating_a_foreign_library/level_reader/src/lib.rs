#[must_use]
pub fn level() -> i32 {
    unsafe { foreign_sys::heisenberg_fixture_level::HEISENBERG_FIXTURE_LEVEL }
}
