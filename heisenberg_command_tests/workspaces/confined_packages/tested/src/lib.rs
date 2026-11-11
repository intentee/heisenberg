pub const fn tested() -> u8 {
    9
}

#[cfg(test)]
mod tests {
    #[test]
    fn x_is_five() {
        assert_eq!(x::x(), 5);
    }
}
