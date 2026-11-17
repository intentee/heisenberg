use std::num::ParseIntError;

pub fn accumulated(texts: &[&str]) -> Result<Vec<u8>, Vec<ParseIntError>> {
    let mut numbers = Vec::new();
    let mut failures = Vec::new();

    for text in texts {
        match text.parse::<u8>() {
            Ok(number) => numbers.push(number),
            Err(failure) => failures.push(failure),
        }
    }

    if failures.is_empty() {
        Ok(numbers)
    } else {
        Err(failures)
    }
}
