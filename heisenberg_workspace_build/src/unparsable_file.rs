use std::fmt;
use std::fmt::Display;
use std::fmt::Formatter;

use cargo_metadata::camino::Utf8PathBuf;
use object::Error;

#[derive(Debug)]
pub struct UnparsableFile {
    pub file: Utf8PathBuf,
    pub source: Error,
}

impl Display for UnparsableFile {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
        write!(formatter, "{}: {}", self.file, self.source)
    }
}
