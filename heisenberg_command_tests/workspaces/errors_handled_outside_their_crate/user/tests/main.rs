use owner::owner_error::OwnerError;
use owner::parsed::parsed;

fn forwarded(text: &str) -> Result<u8, OwnerError> {
    parsed(text)
}

#[test]
fn integration_test_forwards_the_error_of_another_crate() {
    assert!(matches!(forwarded("x"), Err(OwnerError::Unparsable { .. })));
}
