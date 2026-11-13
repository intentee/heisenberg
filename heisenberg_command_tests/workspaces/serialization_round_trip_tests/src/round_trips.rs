use std::num::ParseIntError;
use std::str::FromStr;

const EXPECTED_COUNT: usize = 3;

fn count(numbers: &[u8]) -> usize {
    numbers.len()
}

#[test]
fn json_round_trip_of_numbers() {
    let numbers = vec![1, 2, 3];
    let json = serde_json::to_string(&numbers).unwrap();
    let decoded: Vec<u8> = serde_json::from_str(&json).unwrap();

    assert_eq!(numbers, decoded);
}

#[test]
fn text_round_trip_of_a_number() {
    let number: u8 = 42;
    let text = number.to_string();
    let parsed: u8 = text.parse().unwrap();

    assert_eq!(parsed, number);
}

#[test]
fn json_round_trip_through_bytes() {
    let words = vec!["first".to_owned()];
    let encoded = serde_json::to_vec(&words).expect("the words must encode");
    let decoded: Vec<String> =
        serde_json::from_slice(encoded.as_slice()).expect("the words must decode");

    assert_eq!(decoded, words);
}

#[test]
fn json_encoding_of_numbers() {
    let numbers = vec![1, 2, 3];
    let json = serde_json::to_string(&numbers).unwrap();
    let decoded: Vec<u8> = serde_json::from_str(&json).unwrap();

    assert_eq!(numbers, decoded);
    assert_eq!(json, "[1,2,3]");
}

#[test]
fn json_decoding_of_a_literal() {
    let decoded: Vec<u8> = serde_json::from_str("[1]").unwrap();

    assert_eq!(decoded, vec![1]);
}

#[test]
fn text_round_trip_through_from_str() -> Result<(), ParseIntError> {
    let number: u8 = 7;
    let text = (number).to_string();
    let parsed = u8::from_str(&text)?;

    debug_assert_eq!(number, parsed);

    Ok(())
}

#[test]
fn json_length_of_numbers() {
    let numbers: Vec<u8> = vec![1, 2, 3];
    let json = serde_json::to_string(&numbers).unwrap();

    assert_eq!(json.len(), 7);
    assert_eq!(Some(EXPECTED_COUNT), Some(numbers.len()));
    assert_eq!(numbers.len(), count(&numbers));
    assert_eq!(json.to_string().len(), 7);
}

#[test]
fn parsing_of_written_digits() {
    let digits = String::from("7");
    let parsed: u8 = digits.parse().unwrap();

    assert_eq!(parsed, 7);
}

#[test]
fn text_decoded_twice() {
    let number: u8 = 7;
    let text = number.to_string();
    let copy: String = text.parse().unwrap();
    let reparsed: u8 = copy.parse().unwrap();
    let rewritten = copy.to_string();

    assert_eq!(reparsed, 7);
    assert_eq!(rewritten.len(), 1);
}
