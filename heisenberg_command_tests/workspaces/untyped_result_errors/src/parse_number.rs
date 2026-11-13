pub fn parse_number(text: &str) -> Result<u8, String> {
    text.parse::<u8>().map_err(|error| error.to_string())
}
