use heisenberg_module_graph::crate_module_graph::CrateModuleGraph;
use heisenberg_module_graph::violation::Violation;

#[derive(Default)]
pub struct UnsafeCodeFindings {
    pub integrates_foreign_code: bool,
    pub sites: Vec<Violation>,
}

impl UnsafeCodeFindings {
    pub fn fold(self, CrateModuleGraph { violations, .. }: &mut CrateModuleGraph) {
        let Self {
            integrates_foreign_code,
            sites,
        } = self;

        if !integrates_foreign_code {
            violations.extend(sites);
        }
    }
}
