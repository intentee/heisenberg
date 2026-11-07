pub mod compile_error;

#[cfg(all(feature = "shared_runtime", feature = "static_runtime"))]
compile_error!("the shared_runtime and static_runtime features are mutually exclusive");

#[cfg(all(feature = "shared_runtime", feature = "static_runtime"))]
std::thread_local! {
    static NEVER_BUILT: u8 = 0;
}
