use crate::db::VueDatabase;
use crate::parse::ParseDatabase;
use html_grammar::ast as html;
use html_grammar::syntax_kind::{self as html_syntax, HTML, SCRIPT};
use javascript_grammar::ast as javascript;
use javascript_grammar::syntax_kind::{self as javascript_syntax, JAVASCRIPT};
use web_analysis_utils::FileId;
use web_grammar_utils::{AstNode, LanguageKind, SyntaxNode, SyntaxToken, SyntaxElement, TextRange, WalkEvent};
use web_grammar_utils::syntax_kind::default as default_syntax;

pub(crate) fn syntax_tree(
    db: &VueDatabase,
    file_id: FileId,
    file_lang: LanguageKind,
    text_source: Option<(TextRange, LanguageKind)>,
) -> String {
    let syntax = match file_lang {
        k if k == HTML => db.parse_html(file_id).syntax.to_owned(),
        k if k == JAVASCRIPT => db.parse_javascript(file_id).syntax.to_owned(),
        _ => panic!("unimplemented source language: {:?}", file_lang),
    };
    if let Some((text_range, text_lang)) = text_source {
        let node = match syntax.covering_node(text_range).into() {
            SyntaxElement::Node(_) => panic!("in call to `syntax_tree` with text source expected token, but found node"),
            SyntaxElement::Token(token) => {
                if let Some(tree) = syntax_tree_for_script(token, text_range, text_lang) {
                    return tree;
                }
                token.parent()
            }
        };

        debug_dump(text_lang, node)
    } else {
        debug_dump(file_lang, &syntax)
    }
}

/// Attempts parsing the script contents of a token as the given language.
fn syntax_tree_for_script(token: SyntaxToken, text_range: TextRange, text_lang: LanguageKind) -> Option<String> {
    match token.kind() {
        SCRIPT => syntax_tree_for_token(token, text_range, text_lang),
        _ => None,
    }
}

fn syntax_tree_for_token(node: SyntaxToken, text_range: TextRange, text_lang: LanguageKind) -> Option<String> {
    // Range of the full node
    let node_range = node.range();
    let text = node.text().to_string();

    // We start at some point inside the node
    // Either we have selected the whole string
    // or our selection is inside it
    let start = text_range.start() - node_range.start();

    // how many characters we have selected
    let len = text_range.len().to_usize();

    let node_len = node_range.len().to_usize();

    let start = start.to_usize();

    // We want to cap our length
    let len = len.min(node_len);

    // Ensure our slice is inside the expected range
    let end = if start + len < text.len() { start + len } else { text.len() - start };

    // If the source parses without errors, return its syntax.
    let text = &text[start..end];
    if text_lang == HTML {
        let (parsed, _) = html::Document::parse(text);
        if parsed.errors().is_empty() {
            return Some(debug_dump(HTML, &parsed.syntax));
        }
    } else if text_lang == JAVASCRIPT {
        let (parsed, _) = javascript::Program::parse(text);
        if parsed.errors().is_empty() {
            return Some(debug_dump(JAVASCRIPT, &parsed.syntax));
        }
    }

    None
}

fn debug_dump(lang: LanguageKind, node: &SyntaxNode) -> String {
    use std::fmt::Write;

    let as_debug_repr = match lang {
        k if k == HTML => html_syntax::as_debug_repr,
        k if k == JAVASCRIPT => javascript_syntax::as_debug_repr,
        _ => default_syntax::as_debug_repr,
    };
    let as_str = |k| as_debug_repr(k).map(|k| k.name).unwrap_or("__");
    let mut errors = match lang {
        k if k == HTML => node.ancestors().find_map(html::Document::cast).map(|x| x.errors().to_vec()),
        k if k == JAVASCRIPT => node.ancestors().find_map(javascript::Program::cast).map(|x| x.errors().to_vec()),
        _ => None,
    }.unwrap_or_default();
    errors.sort_by_key(|(_, loc)| loc.offset());
    let mut err_pos = 0;
    let mut level = 0;
    let mut buf = String::new();
    macro_rules! indent {
        () => {
            for _ in 0..level {
                buf.push_str("  ");
            }
        };
    }

    for event in node.preorder_with_tokens() {
        match event {
            WalkEvent::Enter(element) => {
                indent!();
                match element {
                    SyntaxElement::Node(node) => writeln!(buf, "{}@{:?}", as_str(node.kind()), node.range()).unwrap(),
                    SyntaxElement::Token(token) => {
                        writeln!(buf, "{}@{:?}", as_str(token.kind()), token.range()).unwrap();
                        let off = token.range().end();
                        while err_pos < errors.len() && errors[err_pos].1.offset() <= off {
                            indent!();
                            writeln!(buf, "err: `{:?}`", errors[err_pos].0).unwrap();
                            err_pos += 1;
                        }
                    }
                }
                level += 1;
            }
            WalkEvent::Leave(_) => level -= 1,
        }
    }

    assert_eq!(level, 0);
    for (err, _) in errors[err_pos..].iter() {
        writeln!(buf, "err: `{:?}`", err).unwrap();
    }

    buf
}

#[cfg(test)]
mod tests {
    use crate::db::VueDatabase;
    use crate::parse::ParseDatabase;
    use difference::Changeset;
    use html_grammar::syntax_kind::HTML;
    use javascript_grammar::syntax_kind::JAVASCRIPT;
    use web_analysis_utils::{FileId, SourceDatabase};

    macro_rules! assert_diff {
        ($left:expr, $right:expr) => {
            assert_diff!($left, $right,)
        };
        ($left:expr, $right:expr, $($tt:tt)*) => {{
            let left = $left;
            let right = $right;
            if left != right {
                if left.trim() == right.trim() {
                    eprintln!("Left:\n{:?}\n\nRight:\n{:?}\n\nWhitespace difference\n", left, right);
                } else {
                    let changeset = Changeset::new(right, left, "\n");
                    eprintln!("Left:\n{}\n\nRight:\n{}\n\nDiff:\n{}\n", left, right, changeset);
                }
                eprintln!($($tt)*);
                panic!("'assertion failed: `(left == right)`");
            }
        }};
    }

    #[test]
    fn test_syntax_tree_without_range() {
        let mut db = VueDatabase::default();
        let file_id = FileId(1);
        db.set_file_text(file_id, "function foo() {}".to_string().into());
        let syn = super::syntax_tree(&db, file_id, JAVASCRIPT, None);
        assert_diff!(
            syn.trim(),
            r#"
PROGRAM@[0; 17)
  FUNCTION_DECLARATION@[0; 17)
    FUNCTION_KW@[0; 8)
    WHITESPACE@[8; 9)
    IDENTIFIER@[9; 12)
    L_PAREN@[12; 13)
    R_PAREN@[13; 14)
    WHITESPACE@[14; 15)
    L_CURLY@[15; 16)
    R_CURLY@[16; 17)
"#
            .trim()
        );

        let mut db = VueDatabase::default();
        let file_id = FileId(1);
        db.set_file_text(file_id, "<template><img alt='Hello World' /></template>".to_string().into());
        let syn = super::syntax_tree(&db, file_id, HTML, None);
        assert_diff!(
            syn.trim(),
            r#"
DOCUMENT@[0; 46)
  ELEMENT@[0; 46)
    L_ANGLE@[0; 1)
    IDENTIFIER@[1; 9)
    R_ANGLE@[9; 10)
    ELEMENT@[10; 35)
      L_ANGLE@[10; 11)
      IDENTIFIER@[11; 14)
      WHITESPACE@[14; 15)
      ATTRIBUTE@[15; 32)
        IDENTIFIER@[15; 18)
        EQ@[18; 19)
        QUOTED@[19; 32)
      WHITESPACE@[32; 33)
      SLASH_R_ANGLE@[33; 35)
    L_ANGLE_SLASH@[35; 37)
    IDENTIFIER@[37; 45)
    R_ANGLE@[45; 46)
"#
            .trim()
        );
    }

/*
    #[test]
    fn test_syntax_tree_inside_script() {
        let (analysis, range) = single_file_with_range(
            r#"fn test() {
    assert!("
<|>fn foo() {
}<|>
fn bar() {
}
    ", "");
}"#
            .trim(),
        );
        let syn = analysis.syntax_tree(range.file_id, Some(range.range));
        assert_diff!(
            syn.trim(),
            r#"
SOURCE_FILE@[0; 12)
  FN_DEF@[0; 12)
    FN_KW@[0; 2) "fn"
    WHITESPACE@[2; 3) " "
    NAME@[3; 6)
      IDENT@[3; 6) "foo"
    PARAM_LIST@[6; 8)
      L_PAREN@[6; 7) "("
      R_PAREN@[7; 8) ")"
    WHITESPACE@[8; 9) " "
    BLOCK@[9; 12)
      L_CURLY@[9; 10) "{"
      WHITESPACE@[10; 11) "\n"
      R_CURLY@[11; 12) "}"
"#
            .trim()
        );

        // With a raw string
        let (analysis, range) = single_file_with_range(
            r###"fn test() {
    assert!(r#"
<|>fn foo() {
}<|>
fn bar() {
}
    "#, "");
}"###
                .trim(),
        );
        let syn = analysis.syntax_tree(range.file_id, Some(range.range));
        assert_diff!(
            syn.trim(),
            r#"
SOURCE_FILE@[0; 12)
  FN_DEF@[0; 12)
    FN_KW@[0; 2) "fn"
    WHITESPACE@[2; 3) " "
    NAME@[3; 6)
      IDENT@[3; 6) "foo"
    PARAM_LIST@[6; 8)
      L_PAREN@[6; 7) "("
      R_PAREN@[7; 8) ")"
    WHITESPACE@[8; 9) " "
    BLOCK@[9; 12)
      L_CURLY@[9; 10) "{"
      WHITESPACE@[10; 11) "\n"
      R_CURLY@[11; 12) "}"
"#
            .trim()
        );

        // With a raw string
        let (analysis, range) = single_file_with_range(
            r###"fn test() {
    assert!(r<|>#"
fn foo() {
}
fn bar() {
}"<|>#, "");
}"###
                .trim(),
        );
        let syn = analysis.syntax_tree(range.file_id, Some(range.range));
        assert_diff!(
            syn.trim(),
            r#"
SOURCE_FILE@[0; 25)
  FN_DEF@[0; 12)
    FN_KW@[0; 2) "fn"
    WHITESPACE@[2; 3) " "
    NAME@[3; 6)
      IDENT@[3; 6) "foo"
    PARAM_LIST@[6; 8)
      L_PAREN@[6; 7) "("
      R_PAREN@[7; 8) ")"
    WHITESPACE@[8; 9) " "
    BLOCK@[9; 12)
      L_CURLY@[9; 10) "{"
      WHITESPACE@[10; 11) "\n"
      R_CURLY@[11; 12) "}"
  WHITESPACE@[12; 13) "\n"
  FN_DEF@[13; 25)
    FN_KW@[13; 15) "fn"
    WHITESPACE@[15; 16) " "
    NAME@[16; 19)
      IDENT@[16; 19) "bar"
    PARAM_LIST@[19; 21)
      L_PAREN@[19; 20) "("
      R_PAREN@[20; 21) ")"
    WHITESPACE@[21; 22) " "
    BLOCK@[22; 25)
      L_CURLY@[22; 23) "{"
      WHITESPACE@[23; 24) "\n"
      R_CURLY@[24; 25) "}"
"#
            .trim()
        );
    }
*/
}
