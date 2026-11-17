#[derive(Default)]
pub struct Probe {
    #[cfg(test)]
    pub observed: u8,
    pub value: u8,
}

impl Probe {
    pub const fn doubled(&self) -> u8 {
        #[cfg(test)]
        let _observed = self.observed;

        match self.value {
            #[cfg(test)]
            0 => 0,
            value => value * 2,
        }
    }
}
