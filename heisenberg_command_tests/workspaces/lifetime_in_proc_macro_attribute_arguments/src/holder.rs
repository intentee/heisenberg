#[identity::emit(impl<'b> From<&'b str> for Holder { fn from(_name: &'b str) -> Self { Self } })]
pub struct Holder;
