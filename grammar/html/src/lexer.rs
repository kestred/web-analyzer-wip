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
    Template,
    Document,
}

pub struct HtmlLexer {
    mode: HtmlLexerMode,
    current_tag: Option<SmolStr>,
    template_pattern: Option<(&'static str, &'static str)>
}

impl HtmlLexer {
    pub fn new() -> HtmlLexer {
        HtmlLexer {
            mode: HtmlLexerMode::Document,
            current_tag: None,
            template_pattern: None,
        }
    }

    /// Set an open and close delimiter for interpolated sections or
    /// other "template directives" in an HTML document.  This allows
    /// the HtmlLexer to be used by a variety of markup formats.
    pub fn set_template_pattern(&mut self, open: &'static str, close: &'static str) {
        assert!(!open.is_empty() && !close.is_empty());
        self.template_pattern = Some((open, close));
    }

    fn template_opener(&self) -> &'static str {
        self.template_pattern.map(|(x, _)| x).unwrap_or("\0")
    }
}

impl Lexer for HtmlLexer {
    fn scan(&mut self, c: char, s: &mut Scanner) -> SyntaxKind {
        match self.mode {
            // The initial state of parsing.
            HtmlLexerMode::Document => {
                if is_whitespace(c) {
                    s.bump_while(is_whitespace);
                    return WHITESPACE;
                }

                let open = self.template_opener();
                let open_like = open.chars().next().unwrap();
                if c == open_like && s.at_str(open) {
                    self.mode = HtmlLexerMode::Template;
                } else if c == '<' {
                    self.mode = HtmlLexerMode::Tag;
                } else {
                    self.mode = HtmlLexerMode::Text;
                }
                self.scan(c, s)
            }
            // Either interpolation or a directive in an html template
            HtmlLexerMode::Template => {
                let (open, close) = &self.template_pattern.unwrap();
                debug_assert!(
                    c == open.chars().next().unwrap() && s.at_str(&open[1..]),
                    "expected to be at beginning of template during template lexer mode"
                );
                for _ in 1..open.len() {
                    s.bump();
                }

                let close_like = close.chars().next().unwrap();
                loop {
                    s.bump_while(|c| c != close_like);
                    if !s.at(close_like) {
                        break; // e.g. we are at EOF
                    } else if s.at_str(close) {
                        for _ in 0..close.len() {
                            s.bump();
                        }
                        if s.at_str(open) {
                            // no-op; e.g. stay in template mode
                        } else if s.at('<') {
                            self.mode = HtmlLexerMode::Tag;
                        } else {
                            self.mode = HtmlLexerMode::Text;
                        }
                        break;
                    } else if s.current().is_none() {
                        break;
                    }
                    s.bump(); // eat `close_like`
                }
                DELIMITED
            }
            // The contents of a script tag.
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
                SCRIPT_CONTENT
            }
            // The contents of a style tag.
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
                STYLE_CONTENT
            }
            HtmlLexerMode::Text => {
                if let Some((open, _)) = &self.template_pattern {
                    let open_like = open.chars().next().unwrap();
                    loop {
                        s.bump_while(|c| c != '<' && c != open_like);
                        if !s.at(open_like) {
                            if s.at('<') {
                                self.mode = HtmlLexerMode::Tag;
                            }
                            break;
                        } else if s.at_str(open) {
                            self.mode = HtmlLexerMode::Template;
                            break;
                        } else if s.at('<') {
                            self.mode = HtmlLexerMode::Tag;
                            break; // opener could be like `<{`; so double check this to handle ambiguity
                        } else if s.current().is_none() {
                            break;
                        }
                        s.bump(); // eat `open_like`
                    }
                } else {
                    s.bump_while(|c| c != '<');
                    if s.at('<') {
                        self.mode = HtmlLexerMode::Tag;
                    }
                }
                if s.current_text().chars().all(|c| c.is_whitespace()) {
                    WHITESPACE
                } else {
                    TEXT
                }
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
                    if self.current_tag.is_none() {
                        self.current_tag = Some(s.current_text().into());
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
                                self.current_tag = Some("".into());
                                L_ANGLE_BANG
                            }
                            Some('/') => {
                                s.bump();
                                self.current_tag = Some("".into());
                                L_ANGLE_SLASH
                            }
                            _ => L_ANGLE,
                        }
                    }
                    '>' => {
                        self.mode = match self.current_tag.take() {
                            Some(ref tag) if tag == "script" => HtmlLexerMode::Script,
                            Some(ref tag) if tag == "style" => HtmlLexerMode::Style,
                            _ => {
                                let open = self.template_opener();
                                let open_like = open.chars().next().unwrap();
                                if s.at(open_like) && s.at_str(open) {
                                    HtmlLexerMode::Template
                                } else if s.at('<') {
                                    HtmlLexerMode::Tag
                                } else {
                                    HtmlLexerMode::Text
                                }
                            }
                        };
                        R_ANGLE
                    }
                    '/' => {
                        match s.current() {
                            Some('>') => {
                                s.bump();
                                self.mode = match self.current_tag.take() {
                                    Some(ref tag) if tag == "script" => HtmlLexerMode::Script,
                                    Some(ref tag) if tag == "style" => HtmlLexerMode::Style,
                                    _ => {
                                        let open = self.template_opener();
                                        let open_like = open.chars().next().unwrap();
                                        if s.at(open_like) && s.at_str(open) {
                                            HtmlLexerMode::Template
                                        } else if s.at('<') {
                                            HtmlLexerMode::Tag
                                        } else {
                                            HtmlLexerMode::Text
                                        }
                                    }
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
            L_ANGLE, IDENT, R_ANGLE, SCRIPT_CONTENT, L_ANGLE_SLASH, IDENT, R_ANGLE,
            L_ANGLE_SLASH, IDENT, R_ANGLE,
            L_ANGLE_SLASH, IDENT, R_ANGLE
        ];
        assert_eq!(tokens, expect, "{:#?}", tokens.iter().cloned().filter_map(as_debug_repr).map(|x| x.canonical.unwrap_or(x.name)).collect::<Vec<_>>());
    }
}