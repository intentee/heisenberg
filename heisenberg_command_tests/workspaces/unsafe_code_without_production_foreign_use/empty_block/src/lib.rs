unsafe extern "C" {}

pub struct Handle;

unsafe impl Send for Handle {}
