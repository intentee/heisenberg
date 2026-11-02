use std::any::Any;
use std::error::Error;
use std::fmt;
use std::fmt::Display;
use std::fmt::Formatter;

#[derive(Debug)]
pub struct PanicPayload(pub Box<dyn Any + Send>);

impl Display for PanicPayload {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
        let Self(payload) = self;

        match payload.downcast_ref::<&str>() {
            Some(message) => formatter.write_str(message),
            None => match payload.downcast_ref::<String>() {
                Some(message) => formatter.write_str(message),
                None => formatter.write_str("the panic carries no text"),
            },
        }
    }
}

impl Error for PanicPayload {}
