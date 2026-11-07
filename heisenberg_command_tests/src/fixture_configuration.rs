use std::path::PathBuf;

#[derive(Clone, Debug)]
pub struct FixtureConfiguration {
    pub index: usize,
    pub manifest_path: PathBuf,
}
