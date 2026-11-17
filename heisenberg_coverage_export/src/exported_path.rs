use ra_ap_paths::AbsPathBuf;

use heisenberg_absolute_path::absolute_path::AbsolutePath;

pub fn exported_path(AbsolutePath { path }: AbsolutePath) -> AbsPathBuf {
    path.normalize()
}
