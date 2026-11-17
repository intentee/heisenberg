pub mod boxed {
    pub struct Box<TValue> {
        pub value: TValue,
    }
}

pub mod string {
    pub trait ToString {}
}

pub mod vec {
    pub struct Vec<TValue> {
        pub value: TValue,
    }
}
