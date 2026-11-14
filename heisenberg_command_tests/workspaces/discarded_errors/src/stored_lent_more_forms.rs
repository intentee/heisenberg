#[must_use]
pub fn stored_lent_more_forms(text: &str) -> u8 {
    let parsed = text.parse::<u8>();
    let matched = match parsed.as_ref() {
        Ok(value) => *value,
        Err(_) => 0,
    };
    let fallback = *parsed.as_ref().unwrap_or_else(|_| &0);
    let (_, nested) = (parsed.as_ref(), 1);
    let counted = u8::from((&parsed).into_iter().count() == 1);
    let parenthesized = (parsed.as_ref()).ok().copied().unwrap_or(0);

    drop(parsed.as_ref());

    match parsed {
        Ok(value) => value
            .saturating_add(matched)
            .saturating_add(fallback)
            .saturating_add(nested)
            .saturating_add(counted)
            .saturating_add(parenthesized),
        Err(error) => u8::from(error.to_string().is_empty()),
    }
}
