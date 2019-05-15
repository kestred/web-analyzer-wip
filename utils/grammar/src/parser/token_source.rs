//! The contents of this file are filched nearly* verbatim from rust analyzer.

use crate::Token;
use crate::syntax_kind::EOF;
use rowan::{SyntaxKind, TextRange, TextUnit};

#[derive(Copy, Clone, Debug)]
pub struct TokenInput<'a, 'b> {
    pub text: &'a str,
    pub tokens: &'b [Token]
}

impl<'a, 'b, Slice: AsRef<[Token]>> From<(&'a str, &'b Slice)> for TokenInput<'a, 'b> {
    fn from(from: (&'a str, &'b Slice)) -> TokenInput<'a, 'b> {
        TokenInput {
            text: from.0,
            tokens: from.1.as_ref(),
        }
    }
}

/// `TokenSource` abstracts the source of the tokens parser operates one.
pub trait TokenSource {
    /// What is the current token?
    fn token_kind(&self, pos: usize) -> SyntaxKind;
    /// Is the current token joined to the next one (`> >` vs `>>`).
    fn is_token_joint_to_next(&self, pos: usize) -> bool;
    /// Is the current token on the same line as the next one (`> >` vs `>>`).
    fn is_token_inline_to_next(&self, pos: usize) -> bool;
    /// Is the current token a specified keyword?
    fn at_keyword(&self, pos: usize, kw: &str) -> bool;
}

pub struct TextTokenSource<'t> {
    text: &'t str,
    /// start position of each token(expect whitespace and comment)
    /// ```non-rust
    ///  struct Foo;
    /// ^------^---
    /// |      |  ^-
    /// 0      7  10
    /// ```
    /// (token, start_offset): `[(struct, 0), (Foo, 7), (;, 10)]`
    start_offsets: Vec<TextUnit>,
    /// non-whitespace/comment tokens
    /// ```non-rust
    /// struct Foo {}
    /// ^^^^^^ ^^^ ^^
    /// ```
    /// tokens: `[struct, Foo, {, }]`
    tokens: Vec<Token>,
}

impl<'t> TokenSource for TextTokenSource<'t> {
    fn token_kind(&self, pos: usize) -> SyntaxKind {
        if !(pos < self.tokens.len()) {
            return EOF;
        }
        self.tokens[pos].kind
    }
    fn is_token_joint_to_next(&self, pos: usize) -> bool {
        if !(pos + 1 < self.tokens.len()) {
            return true;
        }
        self.start_offsets[pos] + self.tokens[pos].len == self.start_offsets[pos + 1]
    }
    fn is_token_inline_to_next(&self, pos: usize) -> bool {
        if !(pos + 1 < self.tokens.len()) {
            return true;
        }
        let left = self.start_offsets[pos] + self.tokens[pos].len;
        let right = self.start_offsets[pos + 1];
        let range = TextRange::from_to(left, right);
        self.text[range].chars().all(|c| c != '\n' && c != '\r')
    }
    fn at_keyword(&self, pos: usize, kw: &str) -> bool {
        if !(pos < self.tokens.len()) {
            return false;
        }
        let range = TextRange::offset_len(self.start_offsets[pos], self.tokens[pos].len);

        self.text[range] == *kw
    }
}

impl<'t> TextTokenSource<'t> {
    /// Generate input from tokens (allowing some tokens to be skipped, such as comment and whitespace).
    pub fn extract<'o, F>(raw_input: TokenInput<'t, 'o>, skip: F) -> TextTokenSource<'t>
    where
        F: Fn(SyntaxKind) -> bool
    {
        let mut tokens = Vec::new();
        let mut start_offsets = Vec::new();
        let mut len = 0.into();
        for &token in raw_input.tokens.iter() {
            if !skip(token.kind) {
                tokens.push(token);
                start_offsets.push(len);
            }
            len += token.len;
        }

        TextTokenSource {
            text: raw_input.text,
            start_offsets,
            tokens
        }
    }
}