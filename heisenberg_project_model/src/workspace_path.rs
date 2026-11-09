use ra_ap_paths::Utf8Path;

use crate::constants::CARGO_MANIFEST_FILE;
use crate::constants::PARENT_DIRECTORY;

pub struct WorkspacePath<'root> {
    pub root: &'root Utf8Path,
}

impl WorkspacePath<'_> {
    #[must_use]
    pub fn manifest(&self) -> String {
        self.relative(&self.root.join(CARGO_MANIFEST_FILE))
    }

    #[must_use]
    pub fn relative(&self, absolute: &Utf8Path) -> String {
        let shared = self
            .root
            .components()
            .zip(absolute.components())
            .take_while(|(root_component, component)| root_component == component)
            .count();

        self.root
            .components()
            .skip(shared)
            .map(|_| PARENT_DIRECTORY)
            .chain(
                absolute
                    .components()
                    .skip(shared)
                    .map(|component| component.as_str()),
            )
            .collect::<Vec<&str>>()
            .join("/")
    }
}
