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
use crate::syntax_kind::{COMMENT, TOMBSTONE};
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
    stack: Vec<Event<E>>,
    source_pos: usize,
    source: TextTokenSource<'a>,
    sink: TextTreeSink<'a, E>,
}

impl<'a, E: 'static + Debug + Send + Sync> Parser<'a, E> {
    pub fn new(text: &'a str, tokens: &'a [Token]) -> Parser<'a, E> {
        Parser {
            stack: Vec::new(),
            source_pos: 0,
            source: TextTokenSource::extract(text, tokens, skip_predicate),
            sink: TextTreeSink::new(text, tokens),
        }
    }

    pub fn parse<G: Grammar<E>>(mut self, grammar: &G) -> TreeArc<SyntaxNode> {
        self.eval(grammar);
        self.finish()
    }

    pub fn eval<G: Grammar<E>>(&mut self, grammar: &G) {
        let start = self.start_marker();
        let kind = grammar.parse(self);
        self.complete_marker(start, kind);
    }

    fn finish(mut self) -> TreeArc<SyntaxNode> {
        for op in self.stack {
            match op {
                Event::Start { kind } if kind == TOMBSTONE => {}
                Event::Start { kind } => self.sink.start_node(kind, skip_predicate),
                Event::Finish => self.sink.finish_node(),
                Event::Error { error } => self.sink.error(error),
                Event::Span { kind, len } => self.sink.span(kind, len, skip_predicate),
            }
        }
        self.sink.finish()
    }

    fn start_marker(&mut self) -> Marker {
        let start = Marker::new(self.stack.len());
        self.stack.push(Event::Start { kind: TOMBSTONE });
        start
    }

    fn complete_marker(&mut self, marker: Marker, kind: SyntaxKind) {
        match self.stack[marker.pos] {
            Event::Start { kind: ref mut slot, .. } => {
                *slot = kind;
            }
            _ => unreachable!(),
        }
        self.stack.push(Event::Finish { });
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

struct Marker {
    // Pos is an offset into the stack of operations
    pos: usize
}

impl Marker {
    fn new(pos: usize) -> Marker {
        Marker { pos }
    }
}

enum Event<E> {
    Start { kind: SyntaxKind },
    Finish,
    Error { error: E },
    Span { kind: SyntaxKind, len: usize },
}

fn skip_predicate(k: SyntaxKind) -> bool {
    k == COMMENT
}
