identity::identity! {
    pub struct Borrowed<'a> {
        pub name: &'a str,
    }
}
