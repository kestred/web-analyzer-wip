use crate::syntax_kind::*;
use crate::scan::{
    scan_multibyte_symbol,
    scan_number,
    scan_template_literal,
};
use web_grammars_utils::{Lexer, Scanner, SyntaxKind};
use web_grammars_utils::lexer::ResetableLexer;
use web_grammars_utils::scan::{
    is_decimal,
    is_ascii_ident_prefix,
    is_ascii_ident_suffix,
    is_whitespace,
    scan_c_comment,
    scan_shebang,
    scan_string,
};

pub struct JavascriptLexer {
    prev_tokens: [Option<SyntaxKind>; 3]
}

impl JavascriptLexer {
    pub fn new() -> JavascriptLexer {
        JavascriptLexer { prev_tokens: [None, None, None] }
    }

    fn scan_next(&self, c: char, s: &mut Scanner) -> SyntaxKind {
        if is_whitespace(c) {
            s.bump_while(is_whitespace);
            return WHITESPACE;
        }

        match c {
            '#' => {
                if scan_shebang(s) {
                    return SHEBANG;
                }
            }
            '/' => {
                if scan_c_comment(s, false) {
                    return COMMENT;
                }
            }
            _ => (),
        }

        if is_ascii_ident_prefix(c) {
            s.bump_while(is_ascii_ident_suffix);
            if let Some(kind) = to_javascript_keyword(s.current_text()) {
                return kind;
            }
            return IDENT;
        }

        if is_decimal(c) {
            scan_number(c, s);
            return NUMBER_LIT;
        }

        // One-byte symbols/operations/punctuation.
        if let Some(kind) = to_javascript_symbol(c) {
            return kind;
        }

        // Multi-byte symbols/operations/punctuation.
        if let Some(kind) = scan_multibyte_symbol(c, s) {
            return kind;
        }

        match c {
            '\'' | '"' => {
                scan_string(c, s);
                return STRING_LIT;
            }
            '`' => {
                return scan_template_literal(s, JavascriptLexer::new());
            }
            _ => (),
        }

        ERROR
    }
}

impl Lexer for JavascriptLexer {
    fn scan(&mut self, c: char, s: &mut Scanner) -> SyntaxKind {
        let kind = self.scan_next(c, s);
        self.prev_tokens = [Some(kind), self.prev_tokens[0], self.prev_tokens[1]];
        return kind;
    }
}

impl ResetableLexer for JavascriptLexer {
    fn reset(&mut self) {
        self.prev_tokens = [None, None, None];
    }
}