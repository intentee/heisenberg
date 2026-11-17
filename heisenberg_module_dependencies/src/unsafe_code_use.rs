use heisenberg_module_graph::violation::Violation;

#[derive(Default)]
pub struct UnsafeCodeUse {
    pub integrates_foreign_code: bool,
    pub sites: Vec<Violation>,
}

impl UnsafeCodeUse {
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
