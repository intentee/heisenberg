pub struct Second;

impl Second {
    pub fn first() {
        crate::first::uses_second();
    }
}
