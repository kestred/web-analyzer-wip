use crate::syntax_kind::*;
use crate::scan::{
    is_html_tag_prefix,
    is_html_tag_suffix,
    scan_html_comment,
};
use grammar_utils::{Lexer, Scanner, SmolStr, SyntaxKind};
use grammar_utils::scan::{is_whitespace, scan_string};

#[derive(Copy, Clone, Debug, PartialEq)]
enum HtmlLexerMode {
    Tag,
    Text,
    Style,
    Script,
    Document,
}

pub struct HtmlLexer {
    mode: HtmlLexerMode,
    tag: Option<SmolStr>,
}

impl HtmlLexer {
    pub fn new() -> HtmlLexer {
        HtmlLexer {
            mode: HtmlLexerMode::Document,
            tag: None,
        }
    }
}

impl Lexer for HtmlLexer {
    fn scan(&mut self, c: char, s: &mut Scanner) -> SyntaxKind {
        match self.mode {
            HtmlLexerMode::Document => {
                if is_whitespace(c) {
                    s.bump_while(is_whitespace);
                    return WHITESPACE;
                }
                self.mode = match c {
                    '<' => HtmlLexerMode::Tag,
                    _ => HtmlLexerMode::Text,
                };
                self.scan(c, s)
            }
            HtmlLexerMode::Script => {
                while let Some(c) = s.current() {
                    if c == '<' && s.at_str("</script>") {
                        break;
                    }
                    s.bump();
                }
                if s.at('<') {
                    self.mode = HtmlLexerMode::Tag;
                }
                SCRIPT
            }
            HtmlLexerMode::Style => {
                while let Some(c) = s.current() {
                    if c == '<' && s.at_str("</style>") {
                        break;
                    }
                    s.bump();
                }
                if s.at('<') {
                    self.mode = HtmlLexerMode::Tag;
                }
                SCRIPT
            }
            HtmlLexerMode::Text => {
                s.bump_while(|c| c != '<');
                if s.at('<') {
                    self.mode = HtmlLexerMode::Tag;
                }
                return TEXT;
            }
            HtmlLexerMode::Tag => {
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
                    if self.tag.is_none() {
                        self.tag = Some(s.current_text().into());
                    }
                    return IDENTIFIER;
                }

                match c {
                    '\'' | '"' => {
                        scan_string(c, s);
                        return QUOTED;
                    }
                    '=' => EQ,
                    '<' => {
                        match s.current() {
                            Some('!') => {
                                s.bump();
                                self.tag = Some("".into());
                                L_ANGLE_BANG
                            }
                            Some('/') => {
                                s.bump();
                                self.tag = Some("".into());
                                L_ANGLE_SLASH
                            }
                            _ => L_ANGLE,
                        }
                    }
                    '>' => {
                        self.mode = match self.tag.take() {
                            Some(ref tag) if tag == "script" => HtmlLexerMode::Script,
                            Some(ref tag) if tag == "style" => HtmlLexerMode::Style,
                            _ => HtmlLexerMode::Document,
                        };
                        R_ANGLE
                    }
                    '/' => {
                        match s.current() {
                            Some('>') => {
                                s.bump();
                                self.mode = match self.tag.take() {
                                    Some(ref tag) if tag == "script" => HtmlLexerMode::Script,
                                    Some(ref tag) if tag == "style" => HtmlLexerMode::Style,
                                    _ => HtmlLexerMode::Document,
                                };
                                SLASH_R_ANGLE
                            }
                            _ => SLASH,
                        }
                    }


                    // In tag mode, we emit many common symbols raw so that
                    // the lexer can be re-used usefully for enriched HTML formats.
                    '@' => AT,
                    '~' => TILDE,
                    '!' => BANG,
                    ':' => COLON,
                    '$' => DOLLAR,
                    '%' => PERCENT,
                    '^' => CARET,
                    '|' => PIPE,
                    '&' => AMPERSAND,
                    '*' => ASTERISK,
                    '?' => QUESTION,
                    '(' => L_PAREN,
                    ')' => R_PAREN,
                    '[' => L_SQUARE,
                    ']' => R_SQUARE,
                    '{' => L_CURLY,
                    '}' => R_CURLY,

                    // Or
                    _ => ERROR,
                }
            }
        }
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
  <script>
    if 1 > 2 && 3 < 4 {
        console.log('this isn't tokenized');
    }
  </script>
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
            L_ANGLE, IDENT, R_ANGLE, TEXT, L_ANGLE_SLASH, IDENT, R_ANGLE,
            L_ANGLE_SLASH, IDENT, R_ANGLE,
            L_ANGLE, IDENT, IDENT, EQ, QUOTED, R_ANGLE,
            L_ANGLE, IDENT, R_ANGLE, TEXT, L_ANGLE_SLASH, IDENT, R_ANGLE,
            L_ANGLE, IDENT, R_ANGLE, SCRIPT, L_ANGLE_SLASH, IDENT, R_ANGLE,
            L_ANGLE_SLASH, IDENT, R_ANGLE,
            L_ANGLE_SLASH, IDENT, R_ANGLE
        ];
        assert_eq!(tokens, expect, "{:#?}", tokens.iter().cloned().filter_map(as_debug_repr).map(|x| x.canonical.unwrap_or(x.name)).collect::<Vec<_>>());
    }
}