use ra_ap_ide_db::search::FileReferenceNode;

pub struct FoundReference {
    pub name: FileReferenceNode,
    pub writes: bool,
}
