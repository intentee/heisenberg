#![feature(lang_items)]

pub mod convert {
    pub enum Infallible {}
}

pub mod error {
    pub trait Error {}
}

pub mod iter {
    pub mod traits {
        pub mod collect {
            pub trait IntoIterator {}
        }

        pub mod iterator {
            pub trait Iterator {}
        }
    }
}

pub mod marker {
    #[lang = "destruct"]
    pub trait Destruct {}
}

pub mod mem {
    pub fn drop<TValue>(_value: TValue) {}
}

pub mod str {
    pub trait FromStr {}
}

pub mod result {
    pub enum Result<TValue, TError> {
        Ok(TValue),
        Err(TError),
    }
}
