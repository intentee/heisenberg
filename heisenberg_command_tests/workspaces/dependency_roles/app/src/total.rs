use provider::provided::provided;

#[must_use]
pub const fn total() -> u8 {
    provided()
}

#[cfg(test)]
mod tests {
    use outside_checker::checked::checked;

    use helper::helped::helped;
    use tested_with::tested::tested;

    use super::total;

    #[test]
    fn total_is_one() {
        assert_eq!(total().saturating_add(helped()), 6);
        assert_eq!(checked().saturating_add(tested()), 14);
    }
}
