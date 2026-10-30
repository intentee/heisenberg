use crate::module_path::ModulePath;

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum ModuleFileAnalysis {
    Analysed,
    AlreadyAnalysedAs(ModulePath),
}
