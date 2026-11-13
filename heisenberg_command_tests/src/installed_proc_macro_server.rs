use std::collections::HashMap;
use std::env;

use ra_ap_paths::AbsPathBuf;
use ra_ap_paths::Utf8PathBuf;
use ra_ap_proc_macro_api::ProcMacroClient;
use ra_ap_project_model::Sysroot;

use crate::constants::SYSROOT_VARIABLE;
use crate::heisenberg_check_error::HeisenbergCheckError;

/// # Errors
///
/// Returns [`HeisenbergCheckError`] when `HEISENBERG_SYSROOT` cannot be read or is not
/// absolute, when the sysroot holds no procedural macro server, or when the server cannot start.
pub fn installed_proc_macro_server() -> Result<ProcMacroClient, HeisenbergCheckError> {
    let no_environment: HashMap<String, Option<String>> = HashMap::new();

    env::var(SYSROOT_VARIABLE)
        .map_err(HeisenbergCheckError::Sysroot)
        .and_then(|sysroot| {
            AbsPathBuf::try_from(Utf8PathBuf::from(sysroot))
                .map_err(HeisenbergCheckError::SysrootNotAbsolute)
        })
        .and_then(|sysroot| {
            Sysroot::new(Some(sysroot.clone()), None)
                .discover_proc_macro_srv()
                .ok_or(HeisenbergCheckError::SysrootWithoutRoot(sysroot))
        })
        .and_then(|discovered| {
            discovered.map_err(|error| HeisenbergCheckError::ProcMacroServerMissing(error.into()))
        })
        .and_then(|server| {
            ProcMacroClient::spawn(&server, no_environment.iter(), None, 1)
                .map_err(HeisenbergCheckError::ProcMacroServerSpawn)
        })
}
