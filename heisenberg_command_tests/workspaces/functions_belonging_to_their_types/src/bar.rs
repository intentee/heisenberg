use crate::foo::Foo;

pub struct Bar {
    pub value: u8,
}

impl Bar {
    pub fn acquired(Foo { value }: &Foo) -> Option<Self> {
        value.checked_sub(1).map(|value| Self { value })
    }

    pub async fn fetched(foo: &Foo) -> Result<Self, u8> {
        Self::parsed(foo)
    }

    pub async fn loaded(Foo { value }: &Foo) -> Self {
        Self { value: *value }
    }

    pub const fn new(Foo { value }: &Foo) -> Self {
        Self { value: *value }
    }

    pub fn parsed(Foo { value }: &Foo) -> Result<Self, u8> {
        value
            .checked_add(1)
            .map(|value| Self { value })
            .ok_or(*value)
    }
}

impl From<&Foo> for Bar {
    fn from(foo: &Foo) -> Self {
        Self {
            value: foo.doubled(),
        }
    }
}
