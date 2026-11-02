use std::iter::repeat_n;

use ra_ap_paths::Utf8Component;
use ra_ap_paths::Utf8Path;

use crate::constants::PARENT_DIRECTORY;

pub struct WorkspacePath<'root> {
    pub root: &'root Utf8Path,
}

impl WorkspacePath<'_> {
    #[must_use]
    pub fn relative(&self, absolute: &Utf8Path) -> String {
        let root: Vec<Utf8Component<'_>> = self.root.components().collect();
        let shared = root
            .iter()
            .zip(absolute.components())
            .take_while(|(root_component, component)| **root_component == *component)
            .count();

        repeat_n(PARENT_DIRECTORY, root.len() - shared)
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
