use ra_ap_hir::FileRangeWrapper;
use ra_ap_syntax::SyntaxToken;
use ra_ap_vfs::FileId;

pub struct WrittenToken {
    pub token: SyntaxToken,
    pub written: FileRangeWrapper<FileId>,
}
