use crate::grammar;
use crate::lexer::HtmlLexer;
use crate::syntax_kind::{self, DOCUMENT, ELEMENT, SCRIPT_BLOCK, STYLE_BLOCK};
use code_grammar::{ast_node, Lexer, Location, Parser, SyntaxError, SyntaxNode, SyntaxToken, TreeArc};
use code_grammar::parser::ParseConfig;

ast_node!(Document, DOCUMENT);
ast_node!(Element, ELEMENT);
ast_node!(Script, SCRIPT_BLOCK);
ast_node!(Style, STYLE_BLOCK);

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
            preserve_comments: true,
            preserve_whitespace: true,
        });
        let (root, remainder) = parser.parse(grammar::document);
        let node = Document::new(root.to_owned());
        (node, remainder.text)
    }

    pub fn errors(&self) -> Vec<SyntaxError> {
        self.syntax
            .root_data().unwrap()
            .downcast_ref::<Vec<(String, Location)>>().unwrap()
            .into_iter()
            .cloned()
            .map(|(msg, loc)| SyntaxError::new(msg, loc))
            .collect()
    }
}

impl Script {
    pub fn source(&self) -> Option<SyntaxToken> {
        self.syntax.first_token()
    }
}