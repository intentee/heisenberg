#![expect(
    non_camel_case_types,
    reason = "the generated bindings keep the names of the C library"
)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
