use std::path::PathBuf;

#[derive(Clone)]
pub struct OutDirFile {
    pub path: PathBuf,
    pub text: String,
}
