use crate::VueDatabase;
use code_analysis::SourceId;
use code_grammar::{ast, AstNode};
use html_grammar::ast as html;
use html_grammar::syntax_kind as html_syntax;
use javascript_grammar::ast as js;
use javascript_grammar::syntax_kind as js_syntax;
use vue_grammar::syntax_kind as vue_syntax;

pub(crate) fn syntax_tree(
    db: &impl VueDatabase,
    source_id: SourceId,
    source_ext: &str,
) -> String {
    let syntax = match source_ext {
        "htm" | "html" => db.html_ast(source_id).syntax.to_owned(),
        "js" => db.javascript_ast(source_id).syntax.to_owned(),
        "vue" => db.vue_ast(source_id).syntax.to_owned(),

        "css" => panic!("query `syntax_tree` is not implemented for CSS"),
        "ts" => panic!("query `syntax_tree` is not implemented for TypeScript"),
        _ => panic!("unknown source extension: {:?}", source_ext),
    };
    let as_debug_repr = match source_ext {
        "htm" | "html" => html_syntax::as_debug_repr,
        "js" => js_syntax::as_debug_repr,
        "vue" => vue_syntax::as_debug_repr,
        _ => unreachable!(),
    };
    let errors = match source_ext {
        "htm" | "html" => syntax.ancestors().find_map(html::Document::cast).map(|x| x.errors().to_vec()),
        "js" => syntax.ancestors().find_map(js::Program::cast).map(|x| x.errors().to_vec()),
        "vue" => syntax.ancestors().find_map(js::Program::cast).map(|x| x.errors().to_vec()),
        _ => unreachable!(),
    }.unwrap_or_default();
    let formatter = |k| as_debug_repr(k).map(|k| k.name).unwrap_or("UNKNOWN_SYNTAX_KIND");
    ast::debug_dump(&syntax, errors, formatter)
}

#[cfg(test)]
mod tests {
    use crate::db::RootDatabase;
    use crate::parse::FileLanguage::{Html, Javascript};
    use super::*;
    use code_analysis::{FileId, SourceDatabase};
    use code_grammar::TextRange;
    use test_utils::assert_diff;

    #[test]
    fn test_syntax_tree() {
        let mut db = RootDatabase::default();
        let file_id = FileId(1);
        db.set_file_text(file_id, "function foo() {} /* a comment */".to_string().into());
        db.set_file_relative_path(file_id, "example.js".into());
        let syn = syntax_tree(&db, file_id, None);
        assert_diff!(
            syn.trim(),
            r#"
PROGRAM@[0; 17)
  FUNCTION_DECLARATION@[0; 17)
    FUNCTION_KW@[0; 8)  "function"
    WHITESPACE@[8; 9)
    IDENTIFIER@[9; 12)
      IDENTIFIER@[9; 12)  "foo"
    L_PAREN@[12; 13)  "("
    R_PAREN@[13; 14)  ")"
    WHITESPACE@[14; 15)
    BLOCK_STATEMENT@[15; 17)
      L_CURLY@[15; 16)  "{"
      R_CURLY@[16; 17)  "}"
"#
            .trim()
        );

        let mut db = RootDatabase::default();
        let file_id = FileId(1);
        db.set_file_text(file_id, "<template><img alt='Hello World' /></template>".to_string().into());
        db.set_file_relative_path(file_id, "example.html".into());
        let syn = syntax_tree(&db, file_id, None);
        assert_diff!(
            syn.trim(),
            r#"
DOCUMENT@[0; 46)
  ELEMENT@[0; 46)
    L_ANGLE@[0; 1)  "<"
    IDENTIFIER@[1; 9)  "template"
    R_ANGLE@[9; 10)  ">"
    ELEMENT@[10; 35)
      L_ANGLE@[10; 11)  "<"
      IDENTIFIER@[11; 14)  "img"
      WHITESPACE@[14; 15)
      ATTRIBUTE@[15; 32)
        IDENTIFIER@[15; 18)  "alt"
        EQ@[18; 19)  "="
        QUOTED@[19; 32)  "\'Hello World\'"
      WHITESPACE@[32; 33)
      SLASH_R_ANGLE@[33; 35)  "/>"
    L_ANGLE_SLASH@[35; 37)  "</"
    IDENTIFIER@[37; 45)  "template"
    R_ANGLE@[45; 46)  ">"
"#
            .trim()
        );
    }
}
