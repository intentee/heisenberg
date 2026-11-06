use crate::foo::Foo;

pub fn sum(foos: &[Foo]) -> u8 {
    foos.iter().map(|Foo { value }| value).sum()
}
