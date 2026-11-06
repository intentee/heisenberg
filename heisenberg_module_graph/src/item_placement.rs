use std::fmt;
use std::fmt::Display;
use std::fmt::Formatter;

use crate::item_group::ItemGroup;

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct ItemPlacement {
    pub group: ItemGroup,
    pub preceding: String,
    pub preceding_group: ItemGroup,
}

impl Display for ItemPlacement {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
        let Self {
            group,
            preceding,
            preceding_group,
        } = self;

        write!(
            formatter,
            "Item belongs before {preceding}: {group} precede {preceding_group}"
        )
    }
}
