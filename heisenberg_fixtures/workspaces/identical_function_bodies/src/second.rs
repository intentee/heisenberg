pub fn summed_length(words: &[String]) -> usize {
    let mut total = 0;

    for word in words {
        total += word.len();
    }

    total
}
