use heisenberg_module_graph::module_findings::ModuleFindings;

use crate::unsafe_code_use::UnsafeCodeUse;

#[derive(Default)]
pub struct UnsafeCodeFindings {
    pub production: UnsafeCodeUse,
    pub test: UnsafeCodeUse,
}

impl UnsafeCodeFindings {
    pub fn fold(self, ModuleFindings { violations, .. }: &mut ModuleFindings) {
        let Self { production, test } = self;

        if !production.integrates_foreign_code {
            violations.extend(production.sites);

            if !test.integrates_foreign_code {
                violations.extend(test.sites);
            }
        }
    }

    pub fn merge(&mut self, Self { production, test }: Self) {
        self.production.merge(production);
        self.test.merge(test);
    }
}
