pub fn collected(texts: &[&str]) -> Result<usize, Vec<String>> {
    let empty: Vec<String> = texts
        .iter()
        .filter(|text| text.is_empty())
        .map(|text| (*text).to_owned())
        .collect();

    if empty.is_empty() {
        Ok(texts.len())
    } else {
        Err(empty)
    }
}
