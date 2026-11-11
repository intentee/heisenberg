use messages::Message;

#[derive(Message)]
#[message("value {value}")]
pub struct Notice {
    pub value: u8,
}

impl Notice {
    #[must_use]
    pub fn shown(&self) -> String {
        self.to_string()
    }
}
