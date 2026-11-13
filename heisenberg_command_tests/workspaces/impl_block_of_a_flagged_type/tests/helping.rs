impl helpers::Helpers {
    fn help() {}
}

#[test]
fn helps() { assert!(!"helps".is_empty()); }

mod helpers {
    #[derive(Debug)]
    pub struct Helpers;
}
