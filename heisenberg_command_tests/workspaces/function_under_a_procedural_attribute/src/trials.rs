use attribute::trial;

use crate::fixture::Fixture;

#[trial]
fn fixture_is_ready(fixture: &Fixture) -> bool {
    fixture.ready
}
