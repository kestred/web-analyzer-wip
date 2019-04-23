use crate::syntax_kind::*;
use javascript_grammar::scan::{
    scan_multibyte_symbol,
    scan_number,
    scan_template_literal,
};
use web_grammars_utils::{Lexer, Scanner, SyntaxKind};
use web_grammars_utils::scan::{
    is_decimal,
    is_ident_prefix_ascii,
    is_ident_suffix_ascii,
    is_whitespace,
    scan_c_comment,
    scan_shebang,
    scan_string,
};


pub struct TypescriptLexer;

impl Lexer for TypescriptLexer {
    fn scan(c: char, s: &mut Scanner) -> SyntaxKind {
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

        if is_ident_prefix_ascii(c) {
            s.bump_while(is_ident_suffix_ascii);
            if let Some(kind) = to_javascript_keyword(s.current_text()) {
                return kind;
            }
            if let Some(kind) = to_typescript_keyword(s.current_text()) {
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
                scan_template_literal::<TypescriptLexer>(s);
                return TEMPLATE_LIT;
            }
            _ => (),
        }

        ERROR
    }
}