use derives::Parse;

const fn port() -> u16 {
    80
}

#[derive(Parse)]
#[default_with(port)]
pub struct Command;

impl Command {
    pub const fn run(&self) -> u8 {
        1
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn port_is_http() {
        assert_eq!(super::port(), 80);
    }
}
