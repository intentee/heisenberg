pub struct Shared;

unsafe impl Send for Shared {}

#[cfg(test)]
mod tests {
    unsafe extern "C" {
        fn abs(input: i32) -> i32;
    }

    #[test]
    fn absolute_value_comes_from_the_foreign_library() {
        assert_eq!(unsafe { abs(-1) }, 1);
    }
}
