use crate::grammar;
use crate::lexer::HtmlLexer;
use crate::syntax_kind::{self, DOCUMENT, ELEMENT};
use web_grammar_utils::{ast_node, Lexer, Location, Parser, SyntaxNode, TreeArc};
use web_grammar_utils::parser::ParseConfig;

ast_node!(Document, DOCUMENT);
ast_node!(Element, ELEMENT);

impl Document {
    fn new(root: TreeArc<SyntaxNode>) -> TreeArc<Document> {
        assert_eq!(root.kind(), DOCUMENT);
        TreeArc::cast(root)
    }

    pub fn parse(text: &str) -> (TreeArc<Document>, &str) {
        let tokens = HtmlLexer::new().tokenize(text);
        let parser = Parser::new((text, &tokens).into(), ParseConfig {
            debug_repr: syntax_kind::as_debug_repr,
            max_rollback_size: 4,
            preserve_whitespace: false,
        });
        let (root, remainder) = parser.parse(grammar::html_document);
        let node = Document::new(root.to_owned());
        (node, remainder.text)
    }

    pub fn errors(&self) -> &[(String, Location)] {
        self.syntax.root_data().unwrap().downcast_ref::<Vec<(String, Location)>>().unwrap()
    }
}
