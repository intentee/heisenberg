pub mod converting;
pub mod deferred;
pub mod fixed_step;
pub mod named;
pub mod read;
pub mod received;
pub mod recovered;
pub mod user_error;
pub mod wrapped;

#[cfg(test)]
mod tests {
    use owner::owner_error::OwnerError;

    fn issued() -> Result<u8, OwnerError> {
        Err(OwnerError::Missing)
    }

    #[test]
    fn test_code_issues_the_error_of_another_crate() {
        assert!(issued().is_err());
    }
}
