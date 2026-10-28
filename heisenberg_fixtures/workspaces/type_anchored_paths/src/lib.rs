pub trait Shutdown {
    fn shutdown(self: Box<Self>) -> u8;
}

pub struct Process;

impl Shutdown for Process {
    fn shutdown(self: Box<Self>) -> u8 {
        0
    }
}

pub fn trait_object() -> u8 {
    <dyn Shutdown>::shutdown(Box::new(Process))
}

pub fn slice(bytes: &[u8]) -> Vec<u8> {
    <[u8]>::to_vec(bytes)
}

pub fn trait_implementation() -> u8 {
    <Process as Shutdown>::shutdown(Box::new(Process))
}
