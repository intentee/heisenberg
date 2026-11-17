extern crate alloc;

fn alloc() -> usize {
    1
}

#[cfg(test)]
mod tests {
    use super::alloc;

    #[test]
    fn alloc_returns_one() {
        assert_eq!(alloc(), 1);
    }
}
