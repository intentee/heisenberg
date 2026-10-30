pub mod child;

pub struct Parent;

impl Parent {
    pub fn child() -> child::Child {
        child::Child
    }
}
