use crate::syntax_kind::*;
use web_grammars_utils::{Lexer, Scanner, SyntaxKind};

/// Assumes preceding back tick
pub fn scan_template_literal<L: Lexer>(s: &mut Scanner) -> SyntaxKind {
    while let Some(c) = s.current() {
        match c {
            '\\' => {
                s.bump();
                if s.at('\\') || s.at('`') {
                    s.bump();
                }
            }
            '$' => {
                s.bump();
                if s.at('{') {
                    s.bump();

                    // Scan using lexer until we find a matching R_BRACE
                    while let Some(c) = s.current() {
                        #![allow(unreachable_patterns)] // TODO: Figure out why rust is confused
                        match L::scan(c, s) { // TODO: Test this to make sure it works
                            R_CURLY | EOF => break,
                            TOMBSTONE => return TOMBSTONE,
                            ERROR => return ERROR,
                            _ => (),
                        }
                    }
                }
            }
            '`' => {
                s.bump();
                break;
            }
            _ => {
                s.bump();
            }
        }
    }
    TEMPLATE_LIT
}

pub fn scan_multibyte_symbol(c: char, s: &mut Scanner) -> Option<SyntaxKind> {
    let kind = match c {
        // Multi-byte tokens.
        '.' => {
            match (s.current(), s.nth(1)) {
                (Some('.'), Some('.')) => {
                    s.bump();
                    s.bump();
                    DOTDOTDOT
                }
                (Some('.'), _) => {
                    s.bump();
                    DOTDOT
                }
                _ => DOT,
            }
        }
        ':' => {
            match s.current() {
                Some(':') => {
                    s.bump();
                    COLONCOLON
                }
                _ => COLON,
            }
        }
        '=' => {
            match (s.current(), s.nth(1)) {
                (Some('='), Some('=')) => {
                    s.bump();
                    s.bump();
                    EQEQEQ
                }
                (Some('='), _) => {
                    s.bump();
                    EQEQ
                }
                (Some('>'), _) => {
                    s.bump();
                    FAT_ARROW
                }
                _ => EQ,
            }
        }
        '!' => {
            match (s.current(), s.nth(1)) {
                (Some('='), Some('=')) => {
                    s.bump();
                    s.bump();
                    BANGEQEQ
                }
                (Some('='), _) => {
                    s.bump();
                    BANGEQ
                }
                _ => BANG,
            }
        }
        '-' => {
            match s.current() {
                Some('-') => {
                    s.bump();
                    DECREMENT
                }
                Some('=') => {
                    s.bump();
                    MINUS_EQ
                }
                _ => MINUS,
            }
        }
        '+' => {
            match s.current() {
                Some('+') => {
                    s.bump();
                    INCREMENT
                }
                Some('=') => {
                    s.bump();
                    PLUS_EQ
                }
                _ => PLUS,
            }
        }
        '*' => {
            match s.current() {
                Some('=') => {
                    s.bump();
                    STAR_EQ
                }
                _ => STAR,
            }
        }
        '/' => {
            match s.current() {
                Some('=') => {
                    s.bump();
                    SLASH_EQ
                }
                _ => SLASH,
            }
        }
        '%' => {
            match s.current() {
                Some('=') => {
                    s.bump();
                    PERCENT_EQ
                }
                _ => PERCENT,
            }
        }
        '^' => {
            match s.current() {
                Some('=') => {
                    s.bump();
                    CARET_EQ
                }
                _ => CARET,
            }
        }
        '&' => {
            match s.current() {
                Some('&') => {
                    s.bump();
                    AND
                }
                Some('=') => {
                    s.bump();
                    AMPERSAND_EQ
                }
                _ => AMPERSAND,
            }
        }
        '|' => {
            match s.current() {
                Some('|') => {
                    s.bump();
                    OR
                }
                Some('=') => {
                    s.bump();
                    PIPE_EQ
                }
                _ => PIPE,
            }
        }
        '<' => {
            match (s.current(), s.nth(1)) {
                (Some('<'), Some('=')) => {
                    s.bump();
                    SHL_EQ
                }
                (Some('<'), _) => {
                    s.bump();
                    SHL
                }
                (Some('='), _) => {
                    s.bump();
                    LTEQ
                }
                _ => L_ANGLE,
            }
        }
        '>' => {
            match (s.current(), s.nth(1), s.nth(2)) {
                (Some('>'), Some('>'), Some('=')) => {
                    s.bump();
                    SHU_EQ
                }
                (Some('>'), Some('>'), _) => {
                    s.bump();
                    SHU
                }
                (Some('>'), Some('='), _) => {
                    s.bump();
                    SHR_EQ
                }
                (Some('>'), _, _) => {
                    s.bump();
                    SHR
                }
                (Some('='), _, _) => {
                    s.bump();
                    GTEQ
                }
                _ => R_ANGLE,
            }
        }

        _ => return None,
    };
    Some(kind)
}

pub fn scan_number(c: char, s: &mut Scanner) {
    // FIXME: What follows below is "rust-style" numbers; instead, the javascript spec should be implemented

    if c == '0' {
        match s.current().unwrap_or('\0') {
            'b' | 'o' => {
                s.bump();
                scan_digits(s, false);
            }
            'x' => {
                s.bump();
                scan_digits(s, true);
            }
            '0'...'9' | '_' | '.' | 'e' | 'E' => {
                scan_digits(s, true);
            }
            _ => return,
        }
    } else {
        scan_digits(s, false);
    }

    if s.at('.') {
        s.bump();
        scan_digits(s, false);
        scan_float_exponent(s);
        return;
    }

    // It might be a float if it has an exponent
    if s.at('e') || s.at('E') {
        scan_float_exponent(s);
        return;
    }
}

fn scan_digits(s: &mut Scanner, allow_hex: bool) {
    while let Some(c) = s.current() {
        match c {
            '_' | '0'...'9' => {
                s.bump();
            }
            'a'...'f' | 'A'...'F' if allow_hex => {
                s.bump();
            }
            _ => return,
        }
    }
}

fn scan_float_exponent(s: &mut Scanner) {
    if s.at('e') || s.at('E') {
        s.bump();
        if s.at('-') || s.at('+') {
            s.bump();
        }
        scan_digits(s, false);
    }
}