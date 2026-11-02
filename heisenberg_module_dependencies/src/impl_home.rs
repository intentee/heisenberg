use ra_ap_vfs::FileId;

pub enum ImplHome {
    Foreign,
    LocalTraitBlanket { file: FileId },
    LocalType { file: FileId },
}
