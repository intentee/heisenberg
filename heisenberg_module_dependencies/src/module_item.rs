use ra_ap_syntax::ast;

pub enum ModuleItem<'items> {
    Absent,
    Accepted(&'items ast::Item),
    Misnamed {
        item: &'items ast::Item,
        module: String,
    },
}

impl<'items> ModuleItem<'items> {
    pub const fn item(&self) -> Option<&'items ast::Item> {
        match self {
            Self::Absent => None,
            Self::Accepted(item) | Self::Misnamed { item, .. } => Some(item),
        }
    }
}
