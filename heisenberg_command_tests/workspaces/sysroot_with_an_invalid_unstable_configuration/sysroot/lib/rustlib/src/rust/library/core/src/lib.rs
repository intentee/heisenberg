pub mod convert {
    pub enum Infallible {}
}

pub mod error {
    pub trait Error {}
}

pub mod fmt {
    pub trait Display {}
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

pub mod option {
    pub enum Option<TValue> {
        None,
        Some(TValue),
    }
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
