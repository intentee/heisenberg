use ra_ap_syntax::TextSize;
use ra_ap_vfs::FileId;

#[derive(Clone, Copy)]
pub struct TokenSite {
    pub file: FileId,
    pub offset: TextSize,
}
