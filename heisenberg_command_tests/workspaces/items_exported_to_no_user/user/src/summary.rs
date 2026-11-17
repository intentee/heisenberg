use library::constants::SHARED;
use library::detailed::detailed;
use library::report::Report;
use open::offered::offered;

#[must_use]
pub fn summary() -> u8 {
    Report {
        lines: SHARED,
        pages: offered(),
    }
    .tripled()
    .saturating_add(detailed().depth)
}
