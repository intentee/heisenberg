fn incremented_value(foo: &Foo) -> u8 {
    foo.value + 1
}

pub struct Foo {
    pub value: u8,
}

impl Foo {
    pub const fn doubled(&self) -> u8 {
        self.value * 2
    }

    pub fn incremented(&self) -> u8 {
        incremented_value(self)
    }
}
