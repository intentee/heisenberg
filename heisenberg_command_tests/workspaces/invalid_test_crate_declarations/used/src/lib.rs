#[cfg(test)]
const fn sample() -> u8 {
    2
}

pub const fn used() -> u8 {
    5
}

#[cfg(test)]
mod tests {
    #[test]
    fn sample_precedes_used() {
        assert!(super::sample() < super::used());
    }
}
