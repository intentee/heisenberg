use ra_ap_paths::AbsPathBuf;

#[derive(Clone)]
pub struct OutDirFile {
    pub path: AbsPathBuf,
    pub text: String,
}
