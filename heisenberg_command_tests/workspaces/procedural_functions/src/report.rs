pub mod summary;

fn line_count(Report { lines }: &Report) -> u8 {
    *lines
}

pub struct Report {
    pub lines: u8,
}
