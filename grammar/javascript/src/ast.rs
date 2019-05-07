use crate::grammar;
use crate::lexer::JavascriptLexer;
use crate::syntax_kind::{self, PROGRAM};
use grammar_utils::{AstNode, Lexer, Location, Parser, SyntaxError, SyntaxNode, TreeArc};
use grammar_utils::parser::ParseConfig;

// Re-export automatically generated nodes
pub use crate::generated::ast::*;

impl Program {
    fn new(root: &SyntaxNode) -> TreeArc<Program> {
        assert_eq!(root.kind(), PROGRAM);
        Program::cast(root).unwrap().to_owned()
    }

    pub fn parse(text: &str) -> (TreeArc<Program>, &str) {
        let tokens = JavascriptLexer::new().tokenize(text);
        let parser = Parser::new((text, &tokens).into(), ParseConfig {
            debug_repr: syntax_kind::as_debug_repr,
            max_rollback_size: 4,
            preserve_whitespace: false,
        });
        let (root, remainder) = parser.parse(grammar::program);
        (Program::new(&root), remainder.text)
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
