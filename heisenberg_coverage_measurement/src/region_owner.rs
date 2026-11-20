use ra_ap_ide_db::FilePosition;

use heisenberg_module_graph::source_location::SourceLocation;

pub enum RegionOwner {
    Functions(Vec<FilePosition>),
    Item(SourceLocation),
}
