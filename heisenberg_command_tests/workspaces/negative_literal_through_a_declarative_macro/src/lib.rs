macro_rules! wrap {
    ($name:ident = $($value:tt)*) => {
        const $name: i32 = $($value)*;
    };
}

#[literal_emitter::single_token]
fn single_token() {}

#[literal_emitter::separate_sign]
fn separate_sign() {}
