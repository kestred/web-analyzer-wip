use crate::syntax_kind::ERROR;
use rowan::SyntaxKind;

#[derive(Debug, Eq, PartialEq)]
pub enum Outcome {
    Ok,
    Err,
}
impl Outcome {
    // TODO: Docs
    pub fn is_ok(&self) -> bool {
        *self == Outcome::Ok
    }

    // TODO: Docs
    pub fn is_err(&self) -> bool {
        *self == Outcome::Ok
    }

    /// Ignores the outcome if it shouldn't cause the current grammar to fail.
    pub fn ignore(self) {}

    // TODO: Docs
    pub fn map(self, kind: SyntaxKind) -> SyntaxKind {
        match self {
            Outcome::Ok => kind,
            Outcome::Err => ERROR,
        }
    }
}
impl From<SyntaxKind> for Outcome {
    fn from(k: SyntaxKind) -> Outcome {
        if k == ERROR {
            Outcome::Err
        } else {
            Outcome::Ok
        }
    }
}

#[macro_export]
/// Like `try!` but for `Outcome`.
macro_rules! parse_ok {
    ($expr:expr) => {
        match $expr {
            Outcome::Ok => (),
            Outcome::Err => return Outcome::Err,
        }
    };
}