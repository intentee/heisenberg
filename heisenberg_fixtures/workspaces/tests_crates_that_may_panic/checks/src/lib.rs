use library::Library;
use shared_tests::parsed_number;

pub fn constructed_library(text: &str) -> Library {
    assert!(parsed_number(text) > 0, "the number must be positive");

    Library
}
