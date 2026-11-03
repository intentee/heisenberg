use ra_ap_vfs::FileId;

pub enum ImplHome {
    Foreign,
    LocalTraitBlanket { file: FileId },
    LocalTraitForeignType { file: FileId, trait_name: String },
    LocalTraitObject { file: FileId },
    LocalType { file: FileId },
    LocalTypeArgument { file: FileId },
}
