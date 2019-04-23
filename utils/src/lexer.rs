use crate::scanner::Scanner;
use rowan::{SyntaxKind, TextUnit};
use std::convert::{TryFrom, TryInto};

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub struct Token {
    /// The kind of token.
    pub kind: SyntaxKind,
    /// The length of the token.
    pub len: TextUnit,
}

pub trait Lexer {
  /// Break a string up into its component tokens
  fn tokenize(&mut self, text: &str) -> Vec<Token> {
      let mut text = text;
      let mut acc = Vec::new();
      while let Ok(more) = text.try_into() {
          let token = self.next(more);
          acc.push(token);
          let len: u32 = token.len.into();
          text = &more.0[len as usize..];
      }
      acc
  }

  /// Get the next token from a string.
  fn next(&mut self, text: NonEmptyStr) -> Token {
      let mut ptr = Scanner::new(text.0);
      let c = ptr.bump().unwrap();
      let kind = self.scan(c, &mut ptr);
      let len = ptr.into_len();
      Token { kind, len }
  }

  fn scan(&mut self, c: char, s: &mut Scanner) -> SyntaxKind;
}

pub trait ResetableLexer: Lexer {
    fn reset(&mut self);
}

#[derive(Copy, Clone)]
pub struct NonEmptyStr<'a>(&'a str);

impl<'a> TryFrom<&'a str> for NonEmptyStr<'a> {
    type Error = ();

    fn try_from(s: &'a str) -> Result<NonEmptyStr, Self::Error> {
        if s.is_empty() {
            Err(())
        } else {
            Ok(NonEmptyStr(s))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_non_empty_str() {
        let some: Result<NonEmptyStr, _> = "anything".try_into();
        assert!(some.is_ok());
        let none: Result<NonEmptyStr, _> = "".try_into();
        assert!(none.is_err());
    }
}