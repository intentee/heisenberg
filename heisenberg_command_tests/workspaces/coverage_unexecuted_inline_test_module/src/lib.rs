pub fn doubled(value: u32) -> u32 {
    value * 2
}

#[cfg(test)]
mod tests {
    #[test]
    fn doubles_two() {
        let doubled = super::doubled(2);

        if doubled != 4 {
            panic!("two doubles to {doubled}");
        }
    }
}
