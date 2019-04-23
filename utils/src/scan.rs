use crate::scanner::Scanner;

pub fn is_whitespace(c: char) -> bool {
    c.is_whitespace()
}

pub fn is_decimal(c: char) -> bool {
    '0' <= c && c <= '9'
}

pub fn is_ident_prefix_ascii(c: char) -> bool {
    (c >= 'a' && c <= 'z')
        || (c >= 'A' && c <= 'Z')
        || c == '_'
}

pub fn is_ident_suffix_ascii(c: char) -> bool {
    (c >= 'a' && c <= 'z')
        || (c >= 'A' && c <= 'Z')
        || (c >= '0' && c <= '9')
        || c == '_'
}

/// Assumes preceding `#`
pub fn scan_shebang(s: &mut Scanner) -> bool {
    if s.at_str("!/") {
        s.bump();
        s.bump();
        s.bump_until_eol();
        true
    } else {
        false
    }
}

/// Assumes preceding `/`
pub fn scan_c_comment(s: &mut Scanner, nested: bool) -> bool {
    if s.at('/') {
        s.bump_until_eol();
        true
    } else {
        scan_c_block_comment(s, nested)
    }
}

/// Assumes preceding `/`
pub fn scan_c_line_comment(s: &mut Scanner) -> bool {
    if s.at('/') {
        s.bump_until_eol();
        true
    } else {
        false
    }
}

/// Assumes preceding `/`
pub fn scan_c_block_comment(s: &mut Scanner, nested: bool) -> bool {
    if s.at('*') {
        s.bump();
        let mut depth: u32 = 1;
        while depth > 0 {
            if s.at_str("*/") {
                depth -= 1;
                s.bump();
                s.bump();
            } else if nested && s.at_str("/*") {
                depth += 1;
                s.bump();
                s.bump();
            } else if s.bump().is_none() {
                break;
            }
        }
        true
    } else {
        false
    }
}

/// Assumes preceding open delimiter matching `delim`
pub fn scan_string(delim: char, s: &mut Scanner) {
    while let Some(c) = s.current() {
        match c {
            '\\' => {
                s.bump();
                if s.at('\\') || s.at(delim) {
                    s.bump();
                }
            }
            _ if c == delim => {
              s.bump();
              return;
            }
            _ => {
              s.bump();
            }
        }
    }
}