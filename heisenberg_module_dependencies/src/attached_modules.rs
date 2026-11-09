use ra_ap_hir::Module;
use ra_ap_hir::attach_db;
use ra_ap_ide_db::RootDatabase;
use rayon::iter::IntoParallelRefIterator;
use rayon::iter::ParallelIterator;

pub fn attached_modules<TOutput: Send>(
    database: &RootDatabase,
    modules: &[Module],
    work: impl Fn(&RootDatabase, Module) -> TOutput + Send + Sync,
) -> Vec<TOutput> {
    modules
        .par_iter()
        .map_with(database.clone(), |module_database, module| {
            let module_database = &*module_database;

            attach_db(module_database, || work(module_database, *module))
        })
        .collect()
}
