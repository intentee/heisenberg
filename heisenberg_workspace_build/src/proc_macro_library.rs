use std::fs;

use cargo_metadata::camino::Utf8PathBuf;
use object::Object;
use object::ObjectKind;

use crate::workspace_build_error::WorkspaceBuildError;

fn is_dynamic_library(contents: &[u8]) -> bool {
    object::File::parse(contents).is_ok_and(|file| file.kind() == ObjectKind::Dynamic)
}

pub fn proc_macro_library(
    proc_macro: &str,
    filenames: &[Utf8PathBuf],
) -> Result<Utf8PathBuf, WorkspaceBuildError> {
    filenames
        .iter()
        .map(|file| {
            fs::metadata(file)
                .and_then(|metadata| metadata.is_file().then(|| fs::read(file)).transpose())
                .map(|contents| {
                    contents
                        .is_some_and(|contents| is_dynamic_library(&contents))
                        .then(|| file.clone())
                })
                .map_err(|source| WorkspaceBuildError::ProcMacroLibraryRead {
                    file: file.clone(),
                    proc_macro: proc_macro.to_owned(),
                    source,
                })
        })
        .collect::<Result<Vec<Option<Utf8PathBuf>>, WorkspaceBuildError>>()
        .and_then(|candidates| {
            <[Utf8PathBuf; 1]>::try_from(
                candidates
                    .into_iter()
                    .flatten()
                    .collect::<Vec<Utf8PathBuf>>(),
            )
            .map(|[library]| library)
            .map_err(|libraries| WorkspaceBuildError::ProcMacroLibraryNotUnique {
                libraries,
                proc_macro: proc_macro.to_owned(),
            })
        })
}

#[cfg(test)]
mod tests {
    use std::slice;

    use cargo_metadata::camino::Utf8PathBuf;

    use super::proc_macro_library;

    #[test]
    fn unreadable_file_of_a_proc_macro_fails_its_library_lookup() {
        let missing = Utf8PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("missing");

        assert_eq!(
            proc_macro_library("injected", slice::from_ref(&missing))
                .err()
                .map(|error| error.to_string()),
            Some(format!(
                "the file `{missing}` that cargo lists for the procedural macro `injected` cannot be read"
            ))
        );
    }

    #[test]
    fn proc_macro_without_a_dynamic_library_fails_its_library_lookup() {
        let manifest = Utf8PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("Cargo.toml");

        assert_eq!(
            proc_macro_library("injected", &[manifest])
                .err()
                .map(|error| error.to_string()),
            Some(
                "the files that cargo lists for the procedural macro `injected` hold 0 dynamic libraries instead of one: []"
                    .to_owned()
            )
        );
    }
}
