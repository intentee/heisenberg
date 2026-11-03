use crate::foo::Foo;

pub fn sum(foos: &[Foo]) -> u8 {
    foos.iter().map(|foo| foo.value).sum()
}
