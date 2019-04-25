use crate::syntax_kind::*;
use web_grammar_utils::{Scanner, SyntaxKind};

pub fn is_html_tag_prefix(c: char) -> bool {
    (c >= 'a' && c <= 'z') || (c >= 'A' && c <= 'Z')
}

pub fn is_html_tag_suffix(c: char) -> bool {
    (c >= 'a' && c <= 'z') || (c >= 'A' && c <= 'Z') || c == '-'
}

/// Assumes preceding `<`
pub fn scan_html_comment(s: &mut Scanner, nested: bool) -> bool {
    if s.at_str("!--") {
        s.bump();
        s.bump();
        s.bump();
        let mut depth: u32 = 1;
        while depth > 0 {
            if s.at_str("-->") {
                depth -= 1;
                s.bump();
                s.bump();
                s.bump();
            } else if nested && s.at_str("<!--") {
                depth += 1;
                s.bump();
                s.bump();
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

pub fn scan_html_symbol(c: char, s: &mut Scanner) -> Option<SyntaxKind> {
    let kind = match c {
        '=' => EQ,
        '<' => {
            match s.current() {
                Some('!') => {
                    s.bump();
                    L_ANGLE_BANG
                }
                Some('/') => {
                    s.bump();
                    L_ANGLE_SLASH
                }
                _ => L_ANGLE,
            }
        }
        '>' => {
            match s.current() {
                Some('/') => {
                    s.bump();
                    R_ANGLE_SLASH
                }
                _ => R_ANGLE,
            }
        }
        _ => return None,
    };
    Some(kind)
}