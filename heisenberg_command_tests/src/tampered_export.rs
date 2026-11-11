use std::path::PathBuf;

pub struct TamperedExport<TObserved> {
    pub observed: TObserved,
    pub path: PathBuf,
}
