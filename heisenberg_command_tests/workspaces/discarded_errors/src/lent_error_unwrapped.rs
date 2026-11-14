pub fn lent_error_unwrapped(text: &str) {
    let _ = text.parse::<u8>().as_ref().unwrap_err();
}
