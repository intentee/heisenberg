use crate::gauge::Gauge;
use crate::settings::Settings;
use crate::tuning::Tuning;

pub struct Usage {
    pub gauge_is_high: bool,
    pub depth: u8,
    pub gain: u8,
}

impl Usage {
    #[must_use]
    pub const fn new() -> Self {
        Self {
            gauge_is_high: Gauge::new().level().is_high(),
            depth: Settings { depth: 0, width: 1 }.deepened().depth,
            gain: Tuning { gain: 0, width: 1 }.amplified().gain,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::constants::TEST_LIMIT;
    use crate::gauge::Gauge;
    use crate::level::Level;
    use crate::marker::Marker;
    use crate::probe::Probe;
    use crate::raw::Raw;
    use crate::reading::Reading;
    use crate::registry::REGISTRY;
    use crate::settings::Settings;
    use crate::tuning::Tuning;

    #[test]
    fn gauge_starts_unobserved() {
        let reading: Reading = Gauge::new().observed;

        assert_eq!(reading, 0);
        assert_eq!(size_of::<Probe>(), 0);
        assert_eq!(size_of::<Marker>(), 0);
        assert_eq!(size_of::<Raw>(), 4);
        assert!(REGISTRY < TEST_LIMIT);
    }

    #[test]
    fn probed_level_is_not_high() {
        assert!(!Level::Probed.is_high());
        assert_eq!(Settings { depth: 0, width: 2 }.deepened().width, 2);
        assert_eq!(Tuning { gain: 0, width: 2 }.amplified().width, 2);
    }
}
