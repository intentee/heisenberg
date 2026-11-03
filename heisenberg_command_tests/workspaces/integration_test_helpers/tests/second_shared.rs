mod support;

use support::fixture_value::FixtureValue;

#[test]
fn second_shared_test_reads_the_fixture() {
    assert!(FixtureValue::new().value > 0);
}
