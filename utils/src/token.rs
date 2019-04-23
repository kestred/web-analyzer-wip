use rowan::{SyntaxKind, TextUnit};

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub struct Token {
    /// The kind of token.
    pub kind: SyntaxKind,
    /// The length of the token.
    pub len: TextUnit,
}