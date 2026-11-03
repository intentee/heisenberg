use crate::shutdown::Shutdown;

pub struct Process;

impl Process {
    pub fn slice(bytes: &[u8]) -> Vec<u8> {
        <[u8]>::to_vec(bytes)
    }

    pub fn trait_implementation() -> u8 {
        <Process as Shutdown>::shutdown(Box::new(Process))
    }

    pub fn trait_object() -> u8 {
        <dyn Shutdown>::shutdown(Box::new(Process))
    }
}

impl Shutdown for Process {
    fn shutdown(self: Box<Self>) -> u8 {
        0
    }
}
