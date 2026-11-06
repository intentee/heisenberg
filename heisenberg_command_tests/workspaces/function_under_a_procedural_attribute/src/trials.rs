use attribute::trial;

use crate::fixture::Fixture;

#[trial]
fn fixture_is_ready(Fixture { ready }: &Fixture) -> bool {
    *ready
}
