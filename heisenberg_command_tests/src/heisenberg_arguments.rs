use std::ffi::OsStr;
use std::ffi::OsString;
use std::path::Path;

use crate::constants::CARGO_HOME_ARGUMENT;
use crate::constants::SYSROOT_ARGUMENT;
use crate::constants::WORKSPACE_ARGUMENT;

pub struct HeisenbergArguments<'arguments> {
    pub cargo_home: &'arguments OsStr,
    pub subcommand: &'arguments str,
    pub sysroot: &'arguments Path,
    pub workspace: &'arguments Path,
}

impl HeisenbergArguments<'_> {
    #[must_use]
    pub fn arguments(&self) -> [OsString; 7] {
        [
            OsString::from(self.subcommand),
            OsString::from(CARGO_HOME_ARGUMENT),
            self.cargo_home.to_owned(),
            OsString::from(SYSROOT_ARGUMENT),
            self.sysroot.as_os_str().to_owned(),
            OsString::from(WORKSPACE_ARGUMENT),
            self.workspace.as_os_str().to_owned(),
        ]
    }
}
