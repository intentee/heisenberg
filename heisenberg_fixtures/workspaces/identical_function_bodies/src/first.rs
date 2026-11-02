pub fn total_length(names: &[String]) -> usize {
    let mut length = 0;

    for name in names {
        length += name.len();
    }

    length
}
