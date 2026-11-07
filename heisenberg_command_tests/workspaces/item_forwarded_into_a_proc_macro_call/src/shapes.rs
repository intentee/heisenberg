macro_rules! forwarded {
    ($name:ident = $length:expr) => {
        identity::identity! {
            pub struct $name(pub [u8; $length]);
        }
    };
}

forwarded! { Second = 2 }

pub struct Shapes;
