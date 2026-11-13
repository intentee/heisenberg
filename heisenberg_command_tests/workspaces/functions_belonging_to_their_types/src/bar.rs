use std::num::TryFromIntError;

use crate::foo::Foo;

pub struct Bar {
    pub value: u8,
}

impl Bar {
    pub fn acquired(Foo { value }: &Foo) -> Option<Self> {
        value.checked_sub(1).map(|value| Self { value })
    }

    pub async fn fetched(foo: &Foo) -> Result<Self, TryFromIntError> {
        Self::parsed(foo)
    }

    pub async fn loaded(Foo { value }: &Foo) -> Self {
        Self { value: *value }
    }

    pub const fn new(Foo { value }: &Foo) -> Self {
        Self { value: *value }
    }

    pub fn parsed(Foo { value }: &Foo) -> Result<Self, TryFromIntError> {
        u8::try_from(u16::from(*value).saturating_add(1)).map(|value| Self { value })
    }
}

impl From<&Foo> for Bar {
    fn from(foo: &Foo) -> Self {
        Self {
            value: foo.doubled(),
        }
    }
}
