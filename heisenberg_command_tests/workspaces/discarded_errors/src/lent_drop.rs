pub fn lent_drop(text: &str) {
    drop(text.parse::<u8>().as_ref());
}
