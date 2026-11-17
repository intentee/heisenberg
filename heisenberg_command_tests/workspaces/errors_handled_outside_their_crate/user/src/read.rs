use std::fs;
use std::io;
use std::path::Path;

pub fn read(path: &Path) -> Result<String, io::Error> {
    fs::read_to_string(path)
}
