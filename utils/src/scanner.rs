use rowan::TextUnit;
use std::str::Chars;

/// A simple view into the characters of a string.
pub struct Scanner<'s> {
    text: &'s str,
    len: TextUnit,
}

impl<'s> Scanner<'s> {
    /// Creates a new `Scanner` from a string.
    pub fn new(text: &'s str) -> Scanner<'s> {
        Scanner { text, len: 0.into() }
    }

    /// Gets the length of the remaining string.
    pub fn into_len(self) -> TextUnit {
        self.len
    }

    /// Gets the current character, if one exists.
    pub fn current(&self) -> Option<char> {
        self.chars().next()
    }

    /// Gets the nth character from the current.
    /// For example, 0 will return the current character, 1 will return the next, etc.
    pub fn nth(&self, n: u32) -> Option<char> {
        self.chars().nth(n as usize)
    }

    /// Checks whether the current character is `c`.
    pub fn at(&self, c: char) -> bool {
        self.current() == Some(c)
    }

    /// Checks whether the next characters match `s`.
    pub fn at_str(&self, s: &str) -> bool {
        let chars = self.chars();
        chars.as_str().starts_with(s)
    }

    /// Checks whether the current character satisfies the predicate `p`.
    pub fn at_p<P: Fn(char) -> bool>(&self, p: P) -> bool {
        self.current().map(p) == Some(true)
    }

    /// Checks whether the nth character satisfies the predicate `p`.
    pub fn nth_is_p<P: Fn(char) -> bool>(&self, n: u32, p: P) -> bool {
        self.nth(n).map(p) == Some(true)
    }

    /// Moves to the next character.
    pub fn bump(&mut self) -> Option<char> {
        let ch = self.chars().next()?;
        self.len += TextUnit::of_char(ch);
        Some(ch)
    }

    /// Moves to next character until the end of the line or end of file.
    pub fn bump_until_eol(&mut self) {
        loop {
            if self.at('\n') || self.at_str("\r\n") {
                return;
            }
            if self.bump().is_none() {
                break;
            }
        }
    }

    /// Moves to the next character as long as `pred` is satisfied.
    pub fn bump_while<F: Fn(char) -> bool>(&mut self, pred: F) {
        loop {
            match self.current() {
                Some(c) if pred(c) => {
                    self.bump();
                }
                _ => return,
            }
        }
    }

    /// Returns the text up to the current point.
    pub fn current_text(&self) -> &str {
        let len: u32 = self.len.into();
        &self.text[..len as usize]
    }

    /// Returns an iterator over the remaining characters.
    fn chars(&self) -> Chars {
        let len: u32 = self.len.into();
        self.text[len as usize..].chars()
    }

    // TODO: Replace `advance` and `remaining_text` with a safer "child" scanner method

    #[doc(hidden)]
    /// Advances the scanner a given length.
    pub fn advance(&mut self, len: TextUnit) {
        self.len += len;
    }

    #[doc(hidden)]
    /// Returns the remaining text.
    pub fn remaining_text(&self) -> &str {
        let len: u32 = self.len.into();
        &self.text[len as usize..]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_current() {
        let ptr = Scanner::new("test");
        assert_eq!(ptr.current(), Some('t'));
    }

    #[test]
    fn test_nth() {
        let ptr = Scanner::new("test");
        assert_eq!(ptr.nth(0), Some('t'));
        assert_eq!(ptr.nth(1), Some('e'));
        assert_eq!(ptr.nth(2), Some('s'));
        assert_eq!(ptr.nth(3), Some('t'));
        assert_eq!(ptr.nth(4), None);
    }

    #[test]
    fn test_at() {
        let ptr = Scanner::new("test");
        assert!(ptr.at('t'));
        assert!(!ptr.at('a'));
    }

    #[test]
    fn test_at_str() {
        let ptr = Scanner::new("test");
        assert!(ptr.at_str("t"));
        assert!(ptr.at_str("te"));
        assert!(ptr.at_str("test"));
        assert!(!ptr.at_str("tests"));
        assert!(!ptr.at_str("rust"));
    }

    #[test]
    fn test_at_p() {
        let ptr = Scanner::new("test");
        assert!(ptr.at_p(|c| c == 't'));
        assert!(!ptr.at_p(|c| c == 'e'));
    }

    #[test]
    fn test_nth_is_p() {
        let ptr = Scanner::new("test");
        assert!(ptr.nth_is_p(0, |c| c == 't'));
        assert!(!ptr.nth_is_p(1, |c| c == 't'));
        assert!(ptr.nth_is_p(3, |c| c == 't'));
        assert!(!ptr.nth_is_p(150, |c| c == 't'));
    }

    #[test]
    fn test_bump() {
        let mut ptr = Scanner::new("test");
        assert_eq!(ptr.current(), Some('t'));
        ptr.bump();
        assert_eq!(ptr.current(), Some('e'));
        ptr.bump();
        assert_eq!(ptr.current(), Some('s'));
        ptr.bump();
        assert_eq!(ptr.current(), Some('t'));
        ptr.bump();
        assert_eq!(ptr.current(), None);
        ptr.bump();
        assert_eq!(ptr.current(), None);
    }

    #[test]
    fn test_bump_while() {
        let mut ptr = Scanner::new("test");
        assert_eq!(ptr.current(), Some('t'));
        ptr.bump_while(|c| c != 's');
        assert_eq!(ptr.current(), Some('s'));
    }
}