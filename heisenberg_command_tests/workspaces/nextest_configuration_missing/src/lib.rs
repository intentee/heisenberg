pub struct Library;

#[cfg(test)]
mod tests {
    #[test]
    fn saturating_addition_stops_at_the_maximum() {
        assert_eq!(u8::MAX.saturating_add(1), u8::MAX);
    }
}
