struct Entry {
    amount: u32,
}

fn total(entries: &[Entry]) -> u32 {
    entries.iter().map(|Entry { amount }| amount).sum()
}

pub struct Ledger {
    entries: Vec<Entry>,
}

impl Ledger {
    pub fn total(&self) -> u32 {
        total(&self.entries)
    }
}

impl Entry {
    fn doubled(&self) -> u32 {
        self.amount * 2
    }
}
