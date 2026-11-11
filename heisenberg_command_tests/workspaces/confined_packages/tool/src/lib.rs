pub const fn tool() -> u8 {
    16
}

#[cfg(test)]
mod tests {
    #[test]
    fn d_is_fifteen() {
        assert_eq!(d::d(), 15);
    }
}
