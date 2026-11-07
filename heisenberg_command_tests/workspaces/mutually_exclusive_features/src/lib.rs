#[cfg(feature = "static_runtime")]
pub mod linked;
#[cfg(feature = "shared_runtime")]
pub mod shared;
