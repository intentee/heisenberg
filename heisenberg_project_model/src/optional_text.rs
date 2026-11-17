use std::fs;
use std::io;

use ra_ap_paths::Utf8Path;

pub fn optional_text(path: &Utf8Path) -> Result<Option<String>, io::Error> {
    match fs::read_to_string(path) {
        Ok(text) => Ok(Some(text)),
        Err(error) if error.kind() == io::ErrorKind::NotFound => Ok(None),
        Err(error) => Err(error),
    }
}
