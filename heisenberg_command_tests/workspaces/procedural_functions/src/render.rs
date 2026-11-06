use crate::report::Report;

pub fn render(Report { lines }: &Report) -> u8 {
    lines * 2
}
