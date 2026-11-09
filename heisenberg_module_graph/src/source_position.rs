use crate::written_offset::WrittenOffset;

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum SourcePosition {
    Expanded {
        call: WrittenOffset,
        call_node: Box<Self>,
        offset: u32,
    },
    Written(WrittenOffset),
}
