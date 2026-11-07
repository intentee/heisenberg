use std::fmt;
use std::fmt::Display;
use std::fmt::Formatter;

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum ItemGroup {
    ModuleDeclaration,
    VendorImport,
    WorkspaceImport,
    LocalImport,
    PrivateHelper,
    PrivateType,
    PublicItem,
    Test,
    InlineModule,
}

impl Display for ItemGroup {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
        formatter.write_str(match self {
            Self::ModuleDeclaration => "module declarations",
            Self::VendorImport => "imports from vendor crates",
            Self::WorkspaceImport => "imports from workspace members",
            Self::LocalImport => "imports from this crate",
            Self::PrivateHelper => "private helpers",
            Self::PrivateType => "private types",
            Self::PublicItem => "public items",
            Self::Test => "tests",
            Self::InlineModule => "inline modules",
        })
    }
}
