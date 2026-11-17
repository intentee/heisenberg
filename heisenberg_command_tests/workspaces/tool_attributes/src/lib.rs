pub struct Excused;

impl Excused {
    #[clippy::cognitive_complexity = "100"]
    pub const fn complex() {}

    #[cfg_attr(target_pointer_width = "64", rustfmt::skip)]
    pub const fn conditionally_unformatted() {}

    #[clippy::msrv = "1.70.0"]
    pub const fn old() {}

    #[rustfmt::skip]
    pub const fn unformatted() {}
}
