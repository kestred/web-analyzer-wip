use crate::grammar;
use crate::lexer::JavascriptLexer;
use crate::syntax_kind::{self, *};
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
            preserve_comments: false,
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

impl Expression {
    fn new(root: &SyntaxNode) -> TreeArc<Expression> {
        // N.B. an `Expression` can be one of many different syntax kinds
        Expression::cast(root).unwrap().to_owned()
    }

    pub fn parse(text: &str) -> (TreeArc<Expression>, &str) {
        let tokens = JavascriptLexer::new().tokenize(text);
        let parser = Parser::new((text, &tokens).into(), ParseConfig {
            debug_repr: syntax_kind::as_debug_repr,
            max_rollback_size: 4,
            preserve_comments: false,
            preserve_whitespace: false,
        });
        let (root, remainder) = parser.parse(grammar::expression);
        (Expression::new(&root), remainder.text)
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

impl ArrayExpression {
    pub fn elements(&self) -> impl Iterator<Item = &Expression> {
        self.syntax.children().filter_map(Expression::cast)
    }
}

impl ObjectExpression {
    pub fn properties(&self) -> impl Iterator<Item = &Property> {
        self.syntax.children().filter_map(Property::cast)
    }
}

impl Property {
    pub fn key(&self) -> Option<&Expression> {
        self.syntax.first_child().and_then(Expression::cast)
    }

    pub fn value(&self) -> Option<&Expression> {
        self.syntax.last_child().and_then(Expression::cast)
    }

    pub fn computed(&self) -> bool {
        self.syntax.first_child_or_token().map(|t| t.kind()) == Some(L_SQUARE)
    }

    pub fn shorthand(&self) -> bool {
        self.syntax.children().count() == 0
    }
}
