use owner::converted::converted;

use crate::user_error::UserError;

pub fn converting(text: &str) -> Result<u8, UserError> {
    converted(text)
}
