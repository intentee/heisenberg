use std::fmt;
use std::fmt::Display;
use std::fmt::Formatter;

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum UsedItem {
    Constant,
    Enum,
    Field,
    Function,
    Static,
    Struct,
    TypeAlias,
    Union,
    Variant,
}

impl Display for UsedItem {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
        formatter.write_str(match self {
            Self::Constant => "Constant is used",
            Self::Enum => "Enum is used",
            Self::Field => "Field is read",
            Self::Function => "Function is used",
            Self::Static => "Static is used",
            Self::Struct => "Struct is used",
            Self::TypeAlias => "Type alias is used",
            Self::Union => "Union is used",
            Self::Variant => "Variant is constructed",
        })
    }
}
