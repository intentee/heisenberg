pub fn dropped(text: &str) {
    drop(text.parse::<u8>());
}
