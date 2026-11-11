use ra_ap_hir::attach_db;
use ra_ap_ide_db::RootDatabase;
use rayon::iter::IntoParallelRefIterator;
use rayon::iter::ParallelIterator;

pub fn attached_work<TItem: Sync, TOutput: Send>(
    database: &RootDatabase,
    items: &[TItem],
    work: impl Fn(&RootDatabase, &TItem) -> TOutput + Send + Sync,
) -> Vec<TOutput> {
    items
        .par_iter()
        .map_with(database.clone(), |item_database, item| {
            let item_database = &*item_database;

            attach_db(item_database, || work(item_database, item))
        })
        .collect()
}
