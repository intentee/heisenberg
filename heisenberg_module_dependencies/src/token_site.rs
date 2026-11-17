use ra_ap_syntax::TextSize;
use ra_ap_vfs::FileId;

pub struct TokenSite {
    pub file: FileId,
    pub offset: TextSize,
}
