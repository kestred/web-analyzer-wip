use crate::parse::{ParseDatabase, SourceLanguage};
use analysis_utils::FileId;
use grammar_utils::{ast, AstNode, SyntaxNode, SyntaxToken, SyntaxElement, TextRange};
use html_grammar::ast as html;
use html_grammar::syntax_kind as html_syntax;
use javascript_grammar::ast as javascript;
use javascript_grammar::syntax_kind as javascript_syntax;
// use vue_grammar::ast as vue;
use vue_grammar::syntax_kind as vue_syntax;

pub(crate) fn debug_syntax_tree(
    db: &dyn ParseDatabase,
    file_id: FileId,
    text_source: Option<(TextRange, SourceLanguage)>,
) -> String {
    let file_lang = db.input_language(file_id.into()).unwrap(/* FIXME: don't unwrap */);
    let syntax = match file_lang {
        SourceLanguage::Html => db.parse_html(file_id.into()).syntax.to_owned(),
        SourceLanguage::Javascript => db.parse_javascript(file_id.into()).syntax.to_owned(),
        SourceLanguage::Vue => db.parse_vue(file_id.into()).syntax.to_owned(),
        _ => panic!("unimplemented source language: {:?}", file_lang),
    };
    if let Some((text_range, text_lang)) = text_source {
        let node = match syntax.covering_node(text_range).into() {
            SyntaxElement::Node(node) => {
                node
            },
            SyntaxElement::Token(token) => {
                if let Some(tree) = debug_syntax_tree_for_script(token, text_range, text_lang) {
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
fn debug_syntax_tree_for_script(token: SyntaxToken, text_range: TextRange, text_lang: SourceLanguage) -> Option<String> {
    match token.kind() {
        html_syntax::SCRIPT_BLOCK
        | html_syntax::SCRIPT_CONTENT
        | html_syntax::STYLE_BLOCK
        | html_syntax::STYLE_CONTENT =>
            debug_syntax_tree_for_token(token, text_range, text_lang),
        _ => None,
    }
}

fn debug_syntax_tree_for_token(node: SyntaxToken, text_range: TextRange, text_lang: SourceLanguage) -> Option<String> {
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
    match text_lang {
        SourceLanguage::Html => {
            let (parsed, _) = html::Document::parse(text);
            if parsed.errors().is_empty() {
                return Some(debug_dump(text_lang, &parsed.syntax));
            }
        }
        SourceLanguage::Javascript => {
            let (parsed, _) = javascript::Program::parse(text);
            if parsed.errors().is_empty() {
                return Some(debug_dump(text_lang, &parsed.syntax));
            }
        }
        _ => (),
    }

    None
}

fn debug_dump(lang: SourceLanguage, node: &SyntaxNode) -> String {
    let as_debug_repr = match lang {
        SourceLanguage::Html => html_syntax::as_debug_repr,
        SourceLanguage::Javascript => javascript_syntax::as_debug_repr,
        SourceLanguage::Typescript => unimplemented!(),
        SourceLanguage::Vue => vue_syntax::as_debug_repr,
    };
    let errors = match lang {
        SourceLanguage::Html => node.ancestors().find_map(html::Document::cast).map(|x| x.errors().to_vec()),
        SourceLanguage::Javascript => node.ancestors().find_map(javascript::Program::cast).map(|x| x.errors().to_vec()),
        SourceLanguage::Typescript => unimplemented!(),
        SourceLanguage::Vue => node.ancestors().find_map(javascript::Program::cast).map(|x| x.errors().to_vec()),
    }.unwrap_or_default();
    let formatter = |k| as_debug_repr(k).map(|k| k.name).unwrap_or("UNKNOWN_SYNTAX_KIND");
    ast::debug_dump(node, errors, formatter)
}

#[cfg(test)]
mod tests {
    use crate::database::RootDatabase;
    use crate::parse::SourceLanguage::{Html, Javascript};
    use super::*;
    use analysis_utils::{FileId, SourceDatabase};
    use grammar_utils::TextRange;
    use test_utils::assert_diff;

    #[test]
    fn test_debug_syntax_tree_without_range() {
        let mut db = RootDatabase::default();
        let file_id = FileId(1);
        db.set_file_text(file_id, "function foo() {} /* a comment */".to_string().into());
        db.set_file_relative_path(file_id, "example.js".into());
        let syn = debug_syntax_tree(&db, file_id, None);
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
    BLOCK_STATEMENT@[15; 17)
      L_CURLY@[15; 16)
      R_CURLY@[16; 17)
"#
            .trim()
        );

        let mut db = RootDatabase::default();
        let file_id = FileId(1);
        db.set_file_text(file_id, "<template><img alt='Hello World' /></template>".to_string().into());
        db.set_file_relative_path(file_id, "example.html".into());
        let syn = debug_syntax_tree(&db, file_id, None);
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

    #[test]
    fn test_debug_syntax_tree_inside_script() {
        let mut db = RootDatabase::default();
        let file_id = FileId(1);
        let file_text = "<script>function foo() {}</script>";
        db.set_file_text(file_id, file_text.to_string().into());
        db.set_file_relative_path(file_id, "example.html".into());
        let start = file_text.chars().position(|c| c == 'f').unwrap() as u32;
        let end = file_text.chars().position(|c| c == '}').unwrap() as u32 + 1;
        let range = TextRange::from_to(start.into(), end.into());
        let syn = debug_syntax_tree(&db, file_id, Some((range, Javascript)));
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
    BLOCK_STATEMENT@[15; 17)
      L_CURLY@[15; 16)
      R_CURLY@[16; 17)
"#
            .trim()
        );
    }
}
