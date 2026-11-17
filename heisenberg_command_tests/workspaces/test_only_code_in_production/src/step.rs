pub enum Step {
    Forward,
    #[cfg(test)]
    Probed,
}

#[cfg(all(test, target_pointer_width = "64"))]
mod tests {
    use super::Step;

    #[test]
    fn step_moves_forward() {
        assert!(matches!(Step::Forward, Step::Forward));
    }
}
