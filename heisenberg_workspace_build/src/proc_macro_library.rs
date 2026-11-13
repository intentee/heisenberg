use std::fs;

use cargo_metadata::camino::Utf8PathBuf;
use object::Object;
use object::ObjectKind;

use crate::unparsable_file::UnparsableFile;
use crate::workspace_build_error::WorkspaceBuildError;

pub fn proc_macro_library(
    proc_macro: &str,
    filenames: &[Utf8PathBuf],
) -> Result<Utf8PathBuf, WorkspaceBuildError> {
    let mut libraries = Vec::new();
    let mut unparsable = Vec::new();

    for file in filenames {
        let contents = fs::metadata(file)
            .and_then(|metadata| metadata.is_file().then(|| fs::read(file)).transpose())
            .map_err(|source| WorkspaceBuildError::ProcMacroLibraryRead {
                file: file.clone(),
                proc_macro: proc_macro.to_owned(),
                source,
            })?;

        if let Some(contents) = contents {
            match object::File::parse(contents.as_slice()) {
                Ok(parsed) => {
                    if parsed.kind() == ObjectKind::Dynamic {
                        libraries.push(file.clone());
                    }
                }
                Err(source) => unparsable.push(UnparsableFile {
                    file: file.clone(),
                    source,
                }),
            }
        }
    }

    <[Utf8PathBuf; 1]>::try_from(libraries)
        .map(|[library]| library)
        .map_err(|libraries| WorkspaceBuildError::ProcMacroLibraryNotUnique {
            libraries,
            proc_macro: proc_macro.to_owned(),
            unparsable,
        })
}

#[cfg(test)]
mod tests {
    use std::env;
    use std::fs;
    use std::slice;

    use cargo_metadata::camino::Utf8PathBuf;

    use super::proc_macro_library;

    const RELOCATABLE_ELF_HEADER: [u8; 64] = [
        0x7f, b'E', b'L', b'F', 2, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0x3e, 0, 1, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 64, 0, 0,
        0, 0, 0, 64, 0, 0, 0, 0, 0,
    ];

    #[test]
    fn listed_directory_and_relocatable_object_are_not_libraries_of_a_proc_macro() {
        let directory = Utf8PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        let relocatable = Utf8PathBuf::from_path_buf(
            env::temp_dir().join("heisenberg_workspace_build_relocatable_object.o"),
        )
        .expect("the temporary directory must be a UTF-8 path");

        fs::write(&relocatable, RELOCATABLE_ELF_HEADER)
            .expect("the relocatable object must be written");

        assert_eq!(
            proc_macro_library("injected", &[directory, relocatable])
                .err()
                .map(|error| error.to_string()),
            Some(
                "the files that cargo lists for the procedural macro `injected` hold 0 dynamic libraries instead of one: []; the files that are not object files: []"
                    .to_owned()
            )
        );
    }

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
        let expected = fs::read(&manifest)
            .map(|contents| {
                object::File::parse(contents.as_slice()).err().map(|reason| format!(
                    "the files that cargo lists for the procedural macro `injected` hold 0 dynamic libraries instead of one: []; the files that are not object files: [{manifest}: {reason}]"
                ))
            })
            .map_err(|error| error.to_string());

        assert_eq!(
            expected,
            Ok(proc_macro_library("injected", slice::from_ref(&manifest))
                .err()
                .map(|error| error.to_string()))
        );
    }
}
