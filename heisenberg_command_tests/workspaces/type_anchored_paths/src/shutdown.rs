pub trait Shutdown {
    fn shutdown(self: Box<Self>) -> u8;
}
