macro_rules! do_submit {
    () => {
        struct
    };
}

macro_rules! submit {
    () => {
        do_submit!();
    };
}

#[attributes::submitted]
pub fn submitted() {}
