use library::report::Report;

pub fn rendered(Report { lines }: &Report) -> u8 {
    lines * 2
}
