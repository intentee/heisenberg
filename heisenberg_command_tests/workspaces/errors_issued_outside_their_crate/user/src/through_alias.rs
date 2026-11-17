use owner::parsed::parsed;

use crate::aliased::Aliased;

pub fn through_alias(text: &str) -> Aliased {
    parsed(text)
}
