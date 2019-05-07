use crate::location::Location;
use rowan::TextUnit;

#[derive(Clone, Debug)]
pub struct SyntaxError {
    pub message: String,
    pub location: Location,
}

impl SyntaxError {
    pub fn new<M: Into<String>>(message: M, location: Location) -> SyntaxError {
        SyntaxError {
            message: message.into(),
            location,
        }
    }

    pub fn offset(&self) -> TextUnit {
        self.location.offset()
    }
}
