use ra_ap_vfs::FileId;

pub enum ImplHome {
    Foreign,
    LocalTraitBlanket { file: FileId },
    LocalTraitObject { file: FileId },
    LocalTraitOnForeignType { trait_name: String },
    LocalType { file: FileId },
    LocalTypeArgument { file: FileId },
}
