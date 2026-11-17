fn leading() -> usize {
    1
}

const LABEL: &str = concat!(
    "a text continued over lines\n\
                                                                                \n\
                                                                                \n\
                                                                                leading trailing"
);

fn trailing() -> usize {
    LABEL.len()
}

pub fn entry() -> usize {
    leading().saturating_add(trailing())
}

#[cfg(test)]
mod tests {
    use super::leading;
    use super::trailing;

    #[test]
    fn functions_around_the_label_are_counted() {
        assert_eq!(leading(), 1);
        assert_eq!(trailing(), 46);
    }
}
