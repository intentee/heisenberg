use std::path::Path;

pub struct WorkspacePath<'root> {
    pub root: &'root Path,
}

impl WorkspacePath<'_> {
    #[expect(
        clippy::expect_used,
        reason = "every analysed source file and member manifest lies inside the workspace root, and rust-analyzer and cargo metadata hold only UTF-8 paths"
    )]
    #[must_use]
    pub fn relative(&self, absolute: &Path) -> String {
        absolute
            .strip_prefix(self.root)
            .expect("every analysed source file and member manifest lies inside the workspace root")
            .components()
            .map(|component| {
                component
                    .as_os_str()
                    .to_str()
                    .expect("rust-analyzer and cargo metadata hold only UTF-8 paths")
            })
            .collect::<Vec<&str>>()
            .join("/")
    }
}
