use ra_ap_hir_expand::proc_macro::ProcMacro;
use ra_ap_load_cargo::load_proc_macro;
use ra_ap_paths::AbsPath;
use ra_ap_proc_macro_api::ProcMacroClient;

use heisenberg_analysis_threads::analysis_thread::AnalysisThread;
use heisenberg_analysis_threads::analysis_threads::AnalysisThreads;

use crate::project_model_error::ProjectModelError;

pub struct ProcMacroLoading<'threads, 'scope, 'environment> {
    pub dylib: &'scope AbsPath,
    pub server: &'scope ProcMacroClient,
    pub threads: &'threads AnalysisThreads<'scope, 'environment>,
}

impl<'scope> ProcMacroLoading<'_, 'scope, '_> {
    /// # Errors
    ///
    /// Returns [`ProjectModelError::ProcMacroLoadThread`] when the operating system cannot start
    /// the thread that loads the library; the thread returns
    /// [`ProjectModelError::ProcMacroLoad`] when the server cannot load it.
    pub fn spawn(
        &self,
    ) -> Result<AnalysisThread<'scope, Result<Vec<ProcMacro>, ProjectModelError>>, ProjectModelError>
    {
        let dylib = self.dylib;
        let server = self.server;

        self.threads
            .spawn(move || {
                load_proc_macro(server, dylib, &[]).map_err(|source| {
                    ProjectModelError::ProcMacroLoad {
                        path: dylib.to_path_buf(),
                        source,
                    }
                })
            })
            .map_err(|source| ProjectModelError::ProcMacroLoadThread {
                path: dylib.to_path_buf(),
                source,
            })
    }
}
