pub const fn unlisted() -> u8 {
    4
}

#[cfg(test)]
mod tests {
    #[test]
    fn confined_value_is_one() {
        assert_eq!(confined::confined(), 1);
    }
}
