//! An example of how to implement a parsing using the utils.
//!
//! ```ignore
//! use web_grammars_utils::{Lexer, Grammar, Parser, SyntaxKind};
//!
//! struct MyLexer { ... }
//!
//! impl Lexer for MyLexer { ... }
//!
//! fn my_grammar(p: &mut Parser) -> SyntaxKind { ... }
//!
//! fn parse(text: &str) -> SyntaxNode {
//!     let tokens = MyLexer::tokenize(text);
//!     let parser = Parser::new(text, &tokens);
//!     parser.parse(my_grammar)
//! }
//! ```

mod token_source;
mod tree_sink;

use crate::lexer::Token;
use crate::syntax_kind::{COMMENT, EOF, TOMBSTONE};
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
    events: Vec<Event<E>>,
    source_pos: usize,
    source: TextTokenSource<'a>,
    sink: TextTreeSink<'a, E>,
}

impl<'a, E> Parser<'a, E>
where
    E: 'static + Debug + Send + Sync
{
    pub fn new(text: &'a str, tokens: &'a [Token]) -> Parser<'a, E> {
        Parser {
            events: Vec::new(),
            source_pos: 0,
            source: TextTokenSource::extract(text, tokens, skip_predicate),
            sink: TextTreeSink::new(text, tokens),
        }
    }

    /// Parse the grammar completely and return the result root syntax node.
    pub fn parse<G: Grammar<E>>(mut self, grammar: &G) -> TreeArc<SyntaxNode> {
        self.eval(grammar);
        self.finish()
    }

    /// Evaluate a single grammar rule.
    ///
    /// This is intended to be called within a `Grammar` parsing function
    /// to begin parsing a sub-grammar.
    pub fn eval<G: Grammar<E>>(&mut self, grammar: &G) {
        let start = self.start_marker();
        let kind = grammar.parse(self);
        self.complete_marker(start, kind);
    }

    /// [Internal API] Emit error for the current node in the parse tree
    pub(crate) fn error(&mut self, error: E) {
        self.events.push(Event::Error { error });
    }

    /// [Internal API] Starts a new node in the syntax tree. All nodes and tokens
    /// consumed between the `start` and the corresponding `Parser::complete_marker`
    /// belong to the same node.
    pub(crate) fn start_marker(&mut self) -> Marker {
        let start = Marker::new(self.events.len());
        self.events.push(Event::Start { kind: TOMBSTONE });
        start
    }

    /// [Internal API] Finishes the syntax tree node and assigns `kind` to it.
    pub(crate) fn complete_marker(&mut self, marker: Marker, kind: SyntaxKind) {
        match self.events[marker.pos] {
            Event::Start { kind: ref mut slot, .. } => {
                *slot = kind;
            }
            _ => unreachable!(),
        }
        self.events.push(Event::Finish { });
    }

    /// [Internal API] Returns the kind of the current token.
    /// If parser has already reached the end of input,
    /// the special `EOF` kind is returned.
    pub(crate) fn current(&self) -> SyntaxKind {
        self.nth(0)
    }

    /// [Internal API] Checks if the current token is `kind`.
    pub(crate) fn at(&self, kind: SyntaxKind) -> bool {
        self.current() == kind
    }

    /// [Internal API] Lookahead returning the kind of the next nth token.
    pub(crate) fn nth(&self, n: usize) -> SyntaxKind {
        self.source.token_kind(self.source_pos + n)
    }

    /// [Internal API] Advances the parser by one token unconditionally.
    pub(crate) fn bump(&mut self) {
        let kind = self.nth(0);
        if kind == EOF {
            return;
        }
        self.advance(kind, 1);
    }

    /// Advance the parser.
    fn advance(&mut self, kind: SyntaxKind, len: usize) {
        self.source_pos += len;
        self.events.push(Event::Span { kind, len });
    }

    /// Consume the parser and apply it's events to create the syntax tree.
    fn finish(mut self) -> TreeArc<SyntaxNode> {
        for op in self.events {
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
}

impl<'a, E> Parser<'a, E>
where
    E: 'static + Debug + Send + Sync + From<String>
{
    /// Emit an error message for the current node in the parse tree.
    pub fn errmsg<T: Into<String>>(&mut self, message: T) {
        let error = E::from(message.into());
        self.events.push(Event::Error { error })
    }
}

/// See `Parser::start_marker`.
pub(crate) struct Marker {
    // Pos is an offset into the parser's events of events
    pos: usize
}

impl Marker {
    fn new(pos: usize) -> Marker {
        Marker { pos }
    }
}

/// The `Parser` builds up a list of `Event`s which are
/// then converted to a tree structure at the end of parsing.
///
/// This allows for more fine-grained control of parsing in the middle.
enum Event<E> {
    Start { kind: SyntaxKind },
    Finish,
    Error { error: E },
    Span { kind: SyntaxKind, len: usize },
}

fn skip_predicate(k: SyntaxKind) -> bool {
    k == COMMENT
}
