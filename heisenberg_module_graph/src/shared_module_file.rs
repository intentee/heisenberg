use crate::module_file_analysis::ModuleFileAnalysis;
use crate::module_path::ModulePath;

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct SharedModuleFile {
    pub file: String,
    pub module: ModulePath,
    pub analysis: ModuleFileAnalysis,
}
