use outside::absent_macro;
use outside::absent_method;
use outside::absent_value;

pub fn absent() {
    absent_value!();
    absent_method!();
    absent_macro!();
}
