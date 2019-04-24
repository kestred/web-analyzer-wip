use rowan::{TextRange, TextUnit};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Location {
    Offset(TextUnit),
    Range(TextRange),
}

impl Location {
    pub fn offset(&self) -> TextUnit {
        match self {
            Location::Offset(offset) => *offset,
            Location::Range(range) => range.start(),
        }
    }
}

impl Into<Location> for TextUnit {
    fn into(self) -> Location {
        Location::Offset(self)
    }
}

impl Into<Location> for TextRange {
    fn into(self) -> Location {
        Location::Range(self)
    }
}