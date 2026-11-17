use owner::owner_error::OwnerError;

#[derive(Debug, thiserror::Error)]
pub enum UserError {
    #[error("the entry {entry} cannot be read")]
    Entry {
        entry: String,
        #[source]
        source: OwnerError,
    },
    #[error(transparent)]
    Owner(#[from] OwnerError),
}
