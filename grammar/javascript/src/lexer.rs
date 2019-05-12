use crate::syntax_kind::*;
use crate::scan::{
    is_javascript_ident_prefix,
    is_javascript_ident_suffix,
    scan_multibyte_symbol,
    scan_number,
    scan_regexp_literal,
    scan_template_literal,
};
use code_grammar::{Lexer, Scanner, SyntaxKind};
use code_grammar::lexer::ResetableLexer;
use code_grammar::scan::{
    is_decimal,
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
                if scan_regexp_literal(s, self.prev_tokens) {
                    return REGEXP_LITERAL;
                }
            }
            _ => (),
        }

        if is_javascript_ident_prefix(c) {
            s.bump_while(is_javascript_ident_suffix);
            if let Some(kind) = to_javascript_keyword(s.current_text()) {
                return kind;
            }
            return IDENT;
        }

        if is_decimal(c) {
            scan_number(c, s);
            return NUMBER_LITERAL;
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
                return STRING_LITERAL;
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
        if kind != WHITESPACE && kind != COMMENT && kind != ERROR {
            self.prev_tokens = [Some(kind), self.prev_tokens[0], self.prev_tokens[1]];
        }
        return kind;
    }
}

impl ResetableLexer for JavascriptLexer {
    fn reset(&mut self) {
        self.prev_tokens = [None, None, None];
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_scan_regexp() {
        let example = r#"
let foo = /abc/.test(3);
let bar = 12 / 3.5;
if (/^[A-Za-z_]+$/g.test(x)) return true;
"#;

        let tokens = JavascriptLexer::new()
            .tokenize(example)
            .into_iter()
            .map(|t| t.kind)
            .filter(|k| *k != WHITESPACE)
            .collect::<Vec<_>>();

        let expect = vec![
            // let foo = /abc/.test(3);
            LET_KW, IDENT, EQ, REGEXP_LITERAL, DOT, IDENT, L_PAREN, NUMBER_LITERAL, R_PAREN, SEMI,

            // let bar = 12 / 3.5;
            LET_KW, IDENT, EQ, NUMBER_LITERAL, SLASH, NUMBER_LITERAL, SEMI,

            // if (/^[A-Za-z_]+\/$/g.test(x)) return true;
            IF_KW, L_PAREN, REGEXP_LITERAL, DOT, IDENT, L_PAREN, IDENT, R_PAREN, R_PAREN, RETURN_KW, TRUE_KW, SEMI
        ];
        assert_eq!(tokens, expect);
    }

    #[test]
    fn test_scan_template() {
        let example = r#"
let foo = `${bar + 3} and ${ "hello" + `_${baz}_` } in \`myfile.txt\`` + `1` + '2';
"#;

        let tokens = JavascriptLexer::new()
            .tokenize(example)
            .into_iter()
            .map(|t| t.kind)
            .filter(|k| *k != WHITESPACE)
            .collect::<Vec<_>>();

        let expect = vec![LET_KW, IDENT, EQ, TEMPLATE_LITERAL, PLUS, TEMPLATE_LITERAL, PLUS, STRING_LITERAL, SEMI];
        assert_eq!(tokens, expect);
    }

    #[test]
    fn test_scan_sample1() {
        let example = crate::samples::SAMPLE_1;
        let tokens = JavascriptLexer::new()
            .tokenize(example)
            .into_iter()
            .map(|t| t.kind)
            .collect::<Vec<_>>();
        let expect = &[
            WHITESPACE, VAR_KW, WHITESPACE, IDENT, WHITESPACE, EQ, // var rows =
            WHITESPACE, IDENT, L_PAREN, STRING_LITERAL, R_PAREN, SEMI, // prompt("How many rows for your multiplication table?");
        ];
        assert_eq!(&tokens[..expect.len()], expect);

        let tokens = tokens
            .into_iter()
            .filter(|k| *k != WHITESPACE)
            .collect::<Vec<_>>();
        let expect = vec![
            // var rows = prompt("How many rows for your multiplication table?");
            VAR_KW, IDENT, EQ, IDENT, L_PAREN, STRING_LITERAL, R_PAREN, SEMI,

            // var cols = prompt("How many columns for your multiplication table?");
            VAR_KW, IDENT, EQ, IDENT, L_PAREN, STRING_LITERAL, R_PAREN, SEMI,

            // if(rows == "" || rows == null) rows = 10;
            IF_KW, L_PAREN, IDENT, EQEQ, STRING_LITERAL, OR, IDENT, EQEQ, NULL_KW, R_PAREN, IDENT, EQ, NUMBER_LITERAL, SEMI,

            // if(cols== "" || cols== null) cols = 10;
            IF_KW, L_PAREN, IDENT, EQEQ, STRING_LITERAL, OR, IDENT, EQEQ, NULL_KW, R_PAREN, IDENT, EQ, NUMBER_LITERAL, SEMI,

            // createTable(rows, cols);
            IDENT, L_PAREN, IDENT, COMMA, IDENT, R_PAREN, SEMI,

            // function createTable(rows, cols) {
            FUNCTION_KW, IDENT, L_PAREN, IDENT, COMMA, IDENT, R_PAREN, L_CURLY
        ];
        assert_eq!(&tokens[..expect.len()], expect.as_slice());
    }

    #[test]
    fn test_scan_sample2() {
        let example = crate::samples::SAMPLE_2;
        let tokens = JavascriptLexer::new().tokenize(example);
        let errors = tokens.iter().enumerate().filter(|(_, t)| t.kind == ERROR).collect::<Vec<_>>();
        assert!(errors.is_empty(), "Found errors: {:?}", errors);
    }
}
