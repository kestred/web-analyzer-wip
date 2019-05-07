use crate::syntax_kind::*;
use grammar_utils::{Scanner, SyntaxKind};
use grammar_utils::lexer::ResetableLexer;
use grammar_utils::scan::{is_decimal, scan_string};

pub fn is_javascript_ident_prefix(c: char) -> bool {
    (c >= 'a' && c <= 'z')
        || (c >= 'A' && c <= 'Z')
        || c == '_'
        || c == '$'
}

pub fn is_javascript_ident_suffix(c: char) -> bool {
    (c >= 'a' && c <= 'z')
        || (c >= 'A' && c <= 'Z')
        || (c >= '0' && c <= '9')
        || c == '_'
}

/// Assumes preceding back tick
pub fn scan_template_literal(s: &mut Scanner, mut lexer: impl ResetableLexer) -> SyntaxKind {
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
                    while s.current().is_some() {
                        let mut child = Scanner::new(s.remaining_text());
                        let c = child.bump().unwrap();
                        match lexer.scan(c, &mut child) {
                            k if k == EOF => break,
                            k if k == R_CURLY => break,
                            k if k == TOMBSTONE => return TOMBSTONE,
                            k if k == ERROR => return ERROR,
                            _ => (),
                        }
                        s.advance(child.into_len());
                        lexer.reset()
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
    TEMPLATE_LITERAL
}

/// Assumes preceding `/`
pub fn scan_regexp_literal(s: &mut Scanner, prev_tokens: [Option<SyntaxKind>; 3]) -> bool {
    let next_chars = [Some('/'), s.current(), s.nth(1)];
    if !is_regexp_start(next_chars, prev_tokens) {
        return false;
    }

    scan_string('/', s);
    s.bump_while(is_regexp_flag);
    true
}

fn is_regexp_flag(c: char) -> bool {
    match c {
        'g' | 'i' | 'm' | 's' | 'u' | 'y' => true,
        _ => false,
    }
}

fn is_regexp_start(next: [Option<char>; 3], prev: [Option<SyntaxKind>; 3]) -> bool {
    let _1st = match prev[0] {
        Some(p) => p,
        None => return true,
    };

    if is_javascript_punct(_1st) {
        if _1st == INCREMENT || _1st == DECREMENT {
            if let Some(_2nd) = prev[1] {
                let next_chars = [None, next[0], next[1]];
                let prev_tokens = [Some(_2nd), prev[2], None];
                return is_regexp_start(next_chars, prev_tokens);
            }
            return true;
        } else if _1st == R_CURLY {
            // N.B. assume regexp follows curly, because the alternative never
            //      makes sense semantically even if it is valid syntactically.
            return true;
        } else if _1st == R_PAREN {
            // FIXME: This part certainly can parse valid code incorrectly.
            // TODO: Maybe conservatively, always return `false` in this case
            //       and handle the weird symbol sequences during parsing?
            // HACK: Use some heuristics to best guess whether its a regexp
            return match (next[0], next[1], next[2]) {
                // Looks like a pattern
                (Some('/'), Some('^'), _) => true,
                (Some('/'), Some('['), _) => true,
                (Some('/'), Some('\\'), _) => true,

                // Looks like division
                (Some('/'), Some('\n'), _) => false,
                (Some('/'), Some('('), _) => false,
                (Some('/'), Some(' '), _) => false,
                (Some('/'), Some(k), _) if is_decimal(k) => false,

                // Fallback to assuming regexp after a parenthesis
                _ => true,
            };
        } else if _1st == R_BRACK {
            // N.B. a regular expression can never follow a `]`
            return false;
        } else {
            // We are confident that this is actually a regular expression!
            return true;
        }
    }
    match _1st {
        kw if kw == NEW_KW => true,
        kw if kw == DELETE_KW => true,
        kw if kw == VOID_KW => true,
        kw if kw == TYPEOF_KW => true,
        kw if kw == INSTANCEOF_KW => true,
        kw if kw == IN_KW => true,
        kw if kw == DO_KW => true,
        kw if kw == RETURN_KW => true,
        kw if kw == CASE_KW => true,
        kw if kw == THROW_KW => true,
        kw if kw == ELSE_KW => true,
        _ => false,
    }
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
                    BANG_EQEQ
                }
                (Some('='), _) => {
                    s.bump();
                    BANG_EQ
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
                    LT_EQ
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
                    GT_EQ
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