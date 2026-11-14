use std::cell::RefCell;
use std::num::ParseIntError;

pub struct FieldHolder {
    pub parsed: Result<u8, ParseIntError>,
}

impl FieldHolder {
    fn parsed_from(text: &str) -> Self {
        Self {
            parsed: text.parse::<u8>(),
        }
    }

    #[must_use]
    pub fn borrowed_check(&self) -> bool {
        self.itself().parsed.is_ok()
    }

    #[must_use]
    pub fn guarded_check(cell: &RefCell<Self>) -> bool {
        cell.borrow().parsed.is_ok()
    }

    #[must_use]
    pub const fn itself(&self) -> &Self {
        self
    }

    #[must_use]
    pub fn owned_check(text: &str) -> bool {
        Self::parsed_from(text).parsed.is_ok()
    }

    #[must_use]
    pub fn parenthesized_check(&self) -> bool {
        (*self).parsed.is_ok()
    }
}
