pub fn borrowed_temporary_wildcard(text: &str) {
    let _ = &text.parse::<u8>();
}
