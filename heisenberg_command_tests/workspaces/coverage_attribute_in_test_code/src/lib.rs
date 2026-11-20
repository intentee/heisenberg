pub fn one() -> u32 {
    1
}

#[cfg(test)]
mod tests {
    rust_analyzer_divergent_macros::hidden_from_rust_analyzer! {
        use identity_marker_macros::service as renamed;
    }

    #[renamed]
    fn two() -> u32 {
        2
    }

    #[test]
    fn two_is_two() {
        assert_eq!(two(), 2);
    }
}
