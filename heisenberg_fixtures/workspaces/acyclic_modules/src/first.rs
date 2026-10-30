pub struct First;

impl First {
    pub fn second() -> crate::second::Second {
        crate::second::Second
    }

    pub fn third() -> crate::third::Third {
        crate::third::Third
    }
}
