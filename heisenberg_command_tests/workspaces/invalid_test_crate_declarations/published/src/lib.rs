#[cfg(test)]
const fn sample() -> u8 {
    1
}

pub const fn published() -> u8 {
    4
}

#[cfg(test)]
mod tests {
    #[test]
    fn sample_precedes_published() {
        assert!(super::sample() < super::published());
    }
}
