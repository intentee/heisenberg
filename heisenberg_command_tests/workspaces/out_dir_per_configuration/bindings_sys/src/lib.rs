#[cfg(accelerated_backend)]
pub mod accelerated;
#[cfg(declared_backend)]
pub mod backend;
pub mod constants;

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
