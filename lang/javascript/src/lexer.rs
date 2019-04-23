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

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_lex_sample1() {
        let example = r#"
var rows = prompt("How many rows for your multiplication table?");
var cols = prompt("How many columns for your multiplication table?");
if(rows == "" || rows == null) rows = 10;
if(cols== "" || cols== null) cols = 10;
createTable(rows, cols);
function createTable(rows, cols) {
    var j=1;
    var output = "<table border='1' width='500' cellspacing='0'cellpadding='5'>";
    for(i=1;i<=rows;i++) {
        output = output + "<tr>";
        while(j<=cols) {
            output = output + "<td>" + i*j + "</td>";
            j = j+1;
        }
        output = output + "</tr>";
        j = 1;
    }
    output = output + "</table>";
    document.write(output);
}
"#;
        let tokens = JavascriptLexer::new()
            .tokenize(example)
            .into_iter()
            .map(|t| t.kind)
            .collect::<Vec<_>>();
        let expect = &[
            WHITESPACE, VAR_KW, WHITESPACE, IDENT, WHITESPACE, EQ, // var rows =
            WHITESPACE, IDENT, L_PAREN, STRING_LIT, R_PAREN, SEMI, // prompt("How many rows for your multiplication table?");
        ];
        assert_eq!(&tokens[..expect.len()], expect);

        let tokens = tokens
            .into_iter()
            .filter(|k| *k != WHITESPACE)
            .collect::<Vec<_>>();
        let expect = vec![
            // var rows = prompt("How many rows for your multiplication table?");
            VAR_KW, IDENT, EQ, IDENT, L_PAREN, STRING_LIT, R_PAREN, SEMI,

            // var cols = prompt("How many columns for your multiplication table?");
            VAR_KW, IDENT, EQ, IDENT, L_PAREN, STRING_LIT, R_PAREN, SEMI,

            // if(rows == "" || rows == null) rows = 10;
            IF_KW, L_PAREN, IDENT, EQEQ, STRING_LIT, OR, IDENT, EQEQ, NULL_KW, R_PAREN, IDENT, EQ, NUMBER_LIT, SEMI,

            // if(cols== "" || cols== null) cols = 10;
            IF_KW, L_PAREN, IDENT, EQEQ, STRING_LIT, OR, IDENT, EQEQ, NULL_KW, R_PAREN, IDENT, EQ, NUMBER_LIT, SEMI,

            // createTable(rows, cols);
            IDENT, L_PAREN, IDENT, COMMA, IDENT, R_PAREN, SEMI,

            // function createTable(rows, cols) {
            FUNCTION_KW, IDENT, L_PAREN, IDENT, COMMA, IDENT, R_PAREN, L_CURLY
        ];
        assert_eq!(&tokens[..expect.len()], expect.as_slice());


    }
}