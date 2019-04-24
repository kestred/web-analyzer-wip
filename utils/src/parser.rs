//! An example of how to implement a parsing using the utils.
//!
//! ```ignore
//! use crate::MyLexer; // implements `web_grammars_utils::Lexer`
//! use web_grammars_utils::{TextTreeSource, TextTokenSource};
//!
//! fn parse(text: &str) -> SyntaxNode {
//!     let tokens = MyLexer::tokenize(&text);
//!     let token_source = TextTokenSource::new(text, &tokens);
//!     let mut tree_sink = TextTreeSink::new(text, &tokens);
//!     ra_parser::parse(&token_source, &mut tree_sink);
//!     tree_sink.finish()
//! }
//! ```

mod token_source;
mod tree_sink;

use crate::Token;
use crate::syntax_kind::{COMMENT, EOF};
use rowan::{SyntaxKind, SyntaxNode, TreeArc};
use std::fmt::Debug;

use self::token_source::{TokenSource, TextTokenSource};
use self::tree_sink::TextTreeSink;

pub trait Grammar<E>
where
    E: 'static + Debug + Send + Sync,
{
    fn parse(&self, parser: &mut Parser<E>) -> SyntaxKind;
}

impl<F, E> Grammar<E> for F
where
    F: Fn(&mut Parser<E>) -> SyntaxKind,
    E: 'static + Debug + Send + Sync,
{
    fn parse(&self, parser: &mut Parser<E>) -> SyntaxKind {
        self(parser)
    }
}

pub struct Parser<'a, E: 'static + Debug + Send + Sync> {
    source_pos: usize,
    source: TextTokenSource<'a>,
    sink: TextTreeSink<'a, E>,
}

impl<'a, E: 'static + Debug + Send + Sync> Parser<'a, E> {
    pub fn new(text: &'a str, tokens: &'a [Token]) -> Parser<'a, E> {
        Parser {
            source_pos: 0,
            source: TextTokenSource::extract(text, tokens, |k| k == COMMENT),
            sink: TextTreeSink::new(text, tokens),
        }
    }

    pub fn parse<G: Grammar<E>>(mut self, grammar: G) -> TreeArc<SyntaxNode> {
        let start = self.source_pos;
        let kind = grammar.parse(&mut self);
        let len = self.source_pos - start;
        assert!(self.source_pos > start);
        self.sink.token(kind, len, |k| k == COMMENT);
        self.sink.finish()
    }

    // fn nth(&self, n: usize) -> SyntaxKind {
    //     self.source.token_kind(self.source_pos + n)
    // }

    // fn bump(&mut self) {
    //     let kind = self.nth(0);
    //     if kind == EOF {
    //         return;
    //     }
    //     self.advance(1);
    // }

    // fn advance(&mut self, n: usize) {
    //     self.source_pos += n;
    // }
}
