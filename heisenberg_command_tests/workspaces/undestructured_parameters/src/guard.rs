fn count_of(guard: Guard) -> u8 {
    guard.count
}

fn name_length(guard: Guard) -> usize {
    guard.name.len()
}

pub struct Guard {
    pub count: u8,
    pub name: String,
}

impl Guard {
    #[must_use]
    pub fn measured(count: u8, name: &str) -> usize {
        usize::from(count_of(Self {
            count,
            name: name.to_owned(),
        })) + name_length(Self {
            count,
            name: name.to_owned(),
        })
    }
}

impl Drop for Guard {
    fn drop(&mut self) {}
}
