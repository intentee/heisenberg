use cargo_metadata::Package;
use cargo_metadata::Target;

pub struct LibraryCrate<'metadata> {
    pub package: &'metadata Package,
    pub target: &'metadata Target,
}
