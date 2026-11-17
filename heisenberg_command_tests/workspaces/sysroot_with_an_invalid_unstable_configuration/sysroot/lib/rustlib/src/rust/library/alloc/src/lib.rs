pub mod boxed {
    pub struct Box<TValue> {
        pub value: TValue,
    }
}

pub mod string {
    pub trait ToString {}
}
