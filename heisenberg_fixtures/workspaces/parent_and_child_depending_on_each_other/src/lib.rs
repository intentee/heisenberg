pub mod child;

pub struct Parent;

pub fn child() -> child::Child {
    child::Child
}
