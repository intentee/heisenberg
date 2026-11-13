use std::error::Error;
use std::iter;
use std::path::Path;
use std::thread;
use std::thread::Builder;

use ra_ap_ide_db::base_db::ProcMacroLoadingError;
use ra_ap_paths::AbsPathBuf;

use heisenberg_analysis_threads::analysis_threads::AnalysisThreads;
use heisenberg_analysis_threads::constants::UNRESERVABLE_STACK_SIZE;
use heisenberg_analysis_threads::joined::joined;
use heisenberg_command_tests::installed_proc_macro_server::installed_proc_macro_server;
use heisenberg_project_model::proc_macro_loading::ProcMacroLoading;
use heisenberg_project_model::project_model_error::ProjectModelError;

fn chain(error: &(dyn Error + 'static)) -> Vec<String> {
    iter::successors(Some(error), |cause| (*cause).source())
        .map(ToString::to_string)
        .collect()
}

#[test]
fn library_that_the_server_cannot_load_fails_its_loading() {
    let server = installed_proc_macro_server()
        .expect("the procedural macro server of the sysroot must start");
    let dylib = AbsPathBuf::assert_utf8(Path::new(env!("CARGO_MANIFEST_DIR")).join("Cargo.toml"));
    let failures: Vec<ProjectModelError> = thread::scope(|scope| {
        joined(vec![
            ProcMacroLoading {
                dylib: &dylib,
                server: &server,
                threads: &AnalysisThreads::new(scope),
            }
            .spawn(),
        ])
        .into_iter()
        .filter_map(Result::err)
        .collect()
    });

    assert!(matches!(
        failures.as_slice(),
        [ProjectModelError::ProcMacroLoad {
            path,
            source: ProcMacroLoadingError::ProcMacroSrvError(_),
        }] if *path == dylib
    ));
}

#[test]
fn library_whose_thread_cannot_start_fails_its_loading() {
    let server = installed_proc_macro_server()
        .expect("the procedural macro server of the sysroot must start");
    let dylib = AbsPathBuf::assert_utf8(Path::new(env!("CARGO_MANIFEST_DIR")).join("Cargo.toml"));

    thread::scope(|scope| {
        let refused = Builder::new()
            .stack_size(UNRESERVABLE_STACK_SIZE)
            .spawn_scoped(scope, || ())
            .err()
            .map(|error| error.to_string());
        let failure = ProcMacroLoading {
            dylib: &dylib,
            server: &server,
            threads: &AnalysisThreads {
                scope,
                stack_size: UNRESERVABLE_STACK_SIZE,
            },
        }
        .spawn()
        .err()
        .map(|error| chain(&error));

        assert_eq!(
            failure,
            refused.map(|reason| vec![
                format!(
                    "the procedural macros of `{dylib}` cannot be loaded on a thread of their own"
                ),
                "the operating system cannot start a thread".to_owned(),
                reason,
            ])
        );
    });
}
