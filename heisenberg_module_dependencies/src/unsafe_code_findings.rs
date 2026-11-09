use heisenberg_module_graph::module_findings::ModuleFindings;
use heisenberg_module_graph::violation::Violation;

#[derive(Default)]
pub struct UnsafeCodeFindings {
    pub integrates_foreign_code: bool,
    pub sites: Vec<Violation>,
}

impl UnsafeCodeFindings {
    pub fn fold(self, ModuleFindings { violations, .. }: &mut ModuleFindings) {
        let Self {
            integrates_foreign_code,
            sites,
        } = self;

        if !integrates_foreign_code {
            violations.extend(sites);
        }
    }

    pub fn merge(
        &mut self,
        Self {
            integrates_foreign_code,
            sites,
        }: Self,
    ) {
        self.integrates_foreign_code |= integrates_foreign_code;
        self.sites.extend(sites);
    }
}
