use std::collections::BTreeMap;
use std::collections::HashMap;
use std::thread::available_parallelism;
use std::thread::scope;

use heisenberg_analysis_threads::analysis_threads::AnalysisThreads;
use ra_ap_hir::Crate;
use ra_ap_ide_db::RootDatabase;

pub struct NameResolutionPriming<'priming> {
    pub database: &'priming RootDatabase,
    pub members: &'priming [Crate],
}

impl NameResolutionPriming<'_> {
    #[expect(
        clippy::expect_used,
        reason = "Linux and Windows, the supported hosts, always report their available parallelism"
    )]
    pub fn prime(&self) {
        let workers = available_parallelism()
            .expect("the host reports its available parallelism")
            .get();

        for layer in self.layers().values() {
            scope(|scope| {
                let threads = AnalysisThreads { scope };

                for part in layer.chunks(layer.len().div_ceil(workers)) {
                    let database = self.database.clone();

                    threads.spawn(move || {
                        for krate in part {
                            krate.root_module(&database);
                        }
                    });
                }
            });
        }
    }

    fn layers(&self) -> BTreeMap<usize, Vec<Crate>> {
        let mut depths = HashMap::new();

        for member in self.members {
            self.depth(*member, &mut depths);
        }

        let mut layers: BTreeMap<usize, Vec<Crate>> = BTreeMap::new();

        for (krate, depth) in depths {
            layers.entry(depth).or_default().push(krate);
        }

        layers
    }

    fn depth(&self, krate: Crate, depths: &mut HashMap<Crate, usize>) -> usize {
        if let Some(known) = depths.get(&krate) {
            return *known;
        }

        let depth = krate
            .dependencies(self.database)
            .into_iter()
            .fold(0, |deepest, dependency| {
                deepest.max(self.depth(dependency.krate, depths) + 1)
            });

        depths.insert(krate, depth);

        depth
    }
}
