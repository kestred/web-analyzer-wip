use crate::syntax_kind::*;
use crate::scan::{
    is_html_tag_prefix,
    is_html_tag_suffix,
    scan_html_symbol,
    scan_html_comment,
};
use web_grammars_utils::{Lexer, Scanner, SyntaxKind};
use web_grammars_utils::scan::{is_whitespace, scan_string};

pub struct HtmlLexer {
    // N.B. There will probably have config in here eventually...
}

impl HtmlLexer {
    pub fn new() -> HtmlLexer {
        HtmlLexer {}
    }
}

impl Lexer for HtmlLexer {
    fn scan(&mut self, c: char, s: &mut Scanner) -> SyntaxKind {
        if is_whitespace(c) {
            s.bump_while(is_whitespace);
            return WHITESPACE;
        }

        match c {
            '<' => {
                if scan_html_comment(s, false) {
                    return COMMENT;
                }
            }
            _ => (),
        }

        if is_html_tag_prefix(c) {
            s.bump_while(is_html_tag_suffix);
            return IDENT;
        }

        if let Some(kind) = scan_html_symbol(c, s) {
            return kind;
        }

        match c {
            '\'' | '"' => {
                scan_string(c, s);
                return QUOTED_STRING;
            }
            _ => (),
        }

        s.bump_while(|c| !is_whitespace(c) && c != '<' && c != '\'' && c != '"');
        RAW_TEXT
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_scan_sample1() {
        let example = r#"
<!DOCTYPE HTML>
<html>
<!-- where is this comment in the DOM? -->
<head>
  <title>Hello</title>
</head>
<body id="body">
  <p>Welcome to this example.</p>
</body>
</html>
"#;

        let tokens = HtmlLexer::new()
            .tokenize(example)
            .into_iter()
            .map(|t| t.kind)
            .filter(|k| *k != WHITESPACE)
            .collect::<Vec<_>>();

        let expect = vec![
            L_ANGLE_BANG, IDENT, IDENT, R_ANGLE,
            L_ANGLE, IDENT, R_ANGLE,
            COMMENT,
            L_ANGLE, IDENT, R_ANGLE,
            L_ANGLE, IDENT, R_ANGLE, IDENT, L_ANGLE_SLASH, IDENT, R_ANGLE,
            L_ANGLE_SLASH, IDENT, R_ANGLE,
            L_ANGLE, IDENT, IDENT, EQ, QUOTED_STRING, R_ANGLE,
            L_ANGLE, IDENT, R_ANGLE, IDENT, IDENT, IDENT, IDENT, RAW_TEXT, L_ANGLE_SLASH, IDENT, R_ANGLE,
            L_ANGLE_SLASH, IDENT, R_ANGLE,
            L_ANGLE_SLASH, IDENT, R_ANGLE
        ];
        assert_eq!(tokens, expect);
    }
}