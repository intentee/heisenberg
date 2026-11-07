use std::path::PathBuf;

#[derive(Debug)]
pub struct FixtureOutDir {
    pub manifest_path: PathBuf,
    pub package: String,
}
