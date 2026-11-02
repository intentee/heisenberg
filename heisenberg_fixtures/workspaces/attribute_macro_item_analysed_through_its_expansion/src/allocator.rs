use std::alloc::System;

#[global_allocator]
pub static ALLOCATOR: System = System;

pub fn second() {}
