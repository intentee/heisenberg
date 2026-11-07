macro_rules! anonymous_constant {
    () => {
        const _: () = ();
    };
}

macro_rules! exported_type {
    () => {
        pub struct Exported;
    };
}

macro_rules! local_helper {
    () => {
        fn local_helper() {}
    };
}

anonymous_constant!();
exported_type!();
local_helper!();

impl Exported {
    fn helped() {
        local_helper();
    }
}
