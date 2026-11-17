use maker::absent_method;
use maker::absent_value;

pub fn absent() {
    absent_value!();
    absent_method!(0_u8);
}
