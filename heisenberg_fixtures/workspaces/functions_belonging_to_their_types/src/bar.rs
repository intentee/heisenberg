use crate::foo::Foo;

pub struct Bar {
    pub value: u8,
}

impl Bar {
    pub const fn new(foo: &Foo) -> Self {
        Self { value: foo.value }
    }

    pub fn parsed(foo: &Foo) -> Result<Self, u8> {
        foo.value
            .checked_add(1)
            .map(|value| Self { value })
            .ok_or(foo.value)
    }
}

impl From<&Foo> for Bar {
    fn from(foo: &Foo) -> Self {
        Self {
            value: foo.doubled(),
        }
    }
}
