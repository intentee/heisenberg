fn incremented_value(Foo { value }: &Foo) -> u8 {
    value + 1
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
