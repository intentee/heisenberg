mod support;

use support::fixture_value::FixtureValue;

#[test]
fn first_shared_test_reads_the_fixture() {
    assert_eq!(FixtureValue::new().value, 1);
}
